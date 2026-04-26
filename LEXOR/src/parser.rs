use crate::token::Token;
use crate::ast::{Program, Statement, Expression};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    // --- CONSTRUCTOR ---
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    // --- MAIN ENGINE ---
    pub fn parse(&mut self) -> Result<Program, String> {
        //check if naay tokens. If not, throw an error about empty file.
        if self.is_at_end() {
            return Err("Syntax Error: Migo, your file is completely empty! You need at least a 'SCRIPT AREA'.".to_string());
        }


        let mut statements = Vec::new();
        
        
        // Dapat naa script area then start script before anything else
        self.consume(&Token::ScriptArea, "Syntax Error: LEXOR programs MUST begin with 'SCRIPT AREA'")?;
        self.consume(&Token::StartScript, "Syntax Error: Expected 'START SCRIPT' after 'SCRIPT AREA'")?;

        // Loop through everything until we hit END SCRIPT or run out of tokens
        while !self.is_at_end() && !self.check(&Token::EndScript) {
            statements.push(self.parse_statement()?); // push every token statements
        }
        
        self.consume(&Token::EndScript, "Syntax Error: LEXOR programs MUST finish with 'END SCRIPT'")?;

        //check if naa pay code after end script.
        if !self.is_at_end() {
            // Optional: Grab the rogue token to show the user exactly what caused the error
            let rogue_token = self.peek().unwrap();
            return Err(format!("Syntax Error: No code is allowed after 'END SCRIPT'! I found: {:?}", rogue_token));
        }

        Ok(Program { statements })
    }

    // --- STATEMENT ROUTER ---
    fn parse_statement(&mut self) -> Result<Statement, String> {
        if self.check(&Token::Declare) { return self.parse_declaration(); }
        if self.check(&Token::Print) { return self.parse_print(); }
        if self.check(&Token::Scan) { return self.parse_scan(); }
        if self.check(&Token::If) { return self.parse_if(); }
        if self.check(&Token::For) { return self.parse_for(); }
        if self.check(&Token::RepeatWhen) { return self.parse_repeat(); }
        
        // Default to Assignment (x = y = 5)
        self.parse_assignment()
    }

    // --- STATEMENT BUILDERS ---

    // Handles: DECLARE INT x, y, z=5
    fn parse_declaration(&mut self) -> Result<Statement, String> {
        self.advance(); // consume 'DECLARE'
        
        // Ensure it's a valid type (INT, FLOAT, CHAR, BOOL, STRING)
        if !self.match_type(&[Token::IntType, Token::FloatType, Token::CharType, Token::BoolType, Token::StringType]) {
            return Err("Syntax Error: Expected a valid data type after DECLARE.".to_string());
        }
        let var_type = self.previous().clone();
        
        let mut declarations = Vec::new();

        loop {
            let name = match self.advance() {
                Some(Token::Identifier(n)) => n.clone(),
                _ => return Err("Syntax Error: Expected variable name in declaration.".to_string()),
            };

            let value = if self.match_type(&[Token::Assign]) {
                Some(self.parse_expression()?)
            } else {
                None
            };

            declarations.push((name, value));

            // Keep looping as long as there are commas
            if !self.match_type(&[Token::Comma]) {
                break;
            }
        }
        // Notice: NO SEMICOLON CONSUMED!
        Ok(Statement::Declaration { var_type, declarations })
    }

    // Handles: x = y = 4
    fn parse_assignment(&mut self) -> Result<Statement, String> {
        let mut targets = Vec::new();
        
        let first_name = match self.advance() {
            Some(Token::Identifier(n)) => n.clone(),
            _ => return Err("Syntax Error: Expected a variable name or valid statement.".to_string()),
        };
        targets.push(first_name);

        self.consume(&Token::Assign, "Syntax Error: Expected '=' after variable.")?;

        // Keep checking if the next token is an Identifier followed by an '='
        while let Some(Token::Identifier(next_name)) = self.peek().cloned() {
            if let Some(Token::Assign) = self.peek_next() {
                targets.push(next_name);
                self.advance(); // eat identifier
                self.advance(); // eat '='
            } else {
                break;
            }
        }

        let value = self.parse_expression()?;
        Ok(Statement::Assignment { targets, value })
    }

    // Handles: PRINT: x & y
    fn parse_print(&mut self) -> Result<Statement, String> {
        self.advance(); // consume 'PRINT'
        self.consume(&Token::Colon, "Syntax Error: Expected ':' after PRINT.")?;
        let expr = self.parse_expression()?;
        Ok(Statement::Print(expr))
    }

    // Handles: SCAN: x, y
    fn parse_scan(&mut self) -> Result<Statement, String> {
        self.advance(); // consume 'SCAN'
        self.consume(&Token::Colon, "Syntax Error: Expected ':' after SCAN.")?;
        
        let mut targets = Vec::new();
        loop {
            match self.advance() {
                Some(Token::Identifier(n)) => targets.push(n.clone()),
                _ => return Err("Syntax Error: Expected variable name for SCAN.".to_string()),
            };

            if !self.match_type(&[Token::Comma]) {
                break;
            }
        }
        Ok(Statement::Scan(targets))
    }

    // Handles IF / ELSE IF / ELSE using START IF and END IF
    fn parse_if(&mut self) -> Result<Statement, String> {
        self.advance(); // consume 'IF'
        self.consume(&Token::LeftParen, "Syntax Error: Expected '(' after IF.")?;
        let condition = self.parse_expression()?;
        self.consume(&Token::RightParen, "Syntax Error: Expected ')' after condition.")?;

        self.consume(&Token::StartIf, "Syntax Error: Expected 'START IF'.")?;
        let mut body = Vec::new();
        while !self.check(&Token::EndIf) && !self.check(&Token::ElseIf) && !self.check(&Token::Else) && !self.is_at_end() {
            body.push(self.parse_statement()?);
        }
        self.consume(&Token::EndIf, "Syntax Error: Expected 'END IF'.")?;

        // Handle ELSE IFs
        let mut else_ifs = Vec::new();
        while self.match_type(&[Token::ElseIf]) {
            self.consume(&Token::LeftParen, "Syntax Error: Expected '(' after ELSE IF.")?;
            let cond = self.parse_expression()?;
            self.consume(&Token::RightParen, "Syntax Error: Expected ')' after condition.")?;
            
            self.consume(&Token::StartIf, "Syntax Error: Expected 'START IF'.")?;
            let mut elif_body = Vec::new();
            while !self.check(&Token::EndIf) && !self.is_at_end() {
                elif_body.push(self.parse_statement()?);
            }
            self.consume(&Token::EndIf, "Syntax Error: Expected 'END IF'.")?;
            else_ifs.push((cond, elif_body));
        }

        // Handle ELSE
        let mut else_body = None;
        if self.match_type(&[Token::Else]) {
            self.consume(&Token::StartIf, "Syntax Error: Expected 'START IF' after ELSE.")?;
            let mut e_body = Vec::new();
            while !self.check(&Token::EndIf) && !self.is_at_end() {
                e_body.push(self.parse_statement()?);
            }
            self.consume(&Token::EndIf, "Syntax Error: Expected 'END IF'.")?;
            else_body = Some(e_body);
        }

        Ok(Statement::If { condition, body, else_ifs, else_body })
    }

    // Handles FOR (init, cond, update) \n START FOR \n ... \n END FOR
    fn parse_for(&mut self) -> Result<Statement, String> {
        self.advance(); // consume 'FOR'
        self.consume(&Token::LeftParen, "Syntax Error: Expected '(' after FOR.")?;
        
        let initialization = Box::new(self.parse_assignment()?);
        self.consume(&Token::Comma, "Syntax Error: Expected ',' after initialization in FOR loop.")?;
        
        let condition = self.parse_expression()?;
        self.consume(&Token::Comma, "Syntax Error: Expected ',' after condition in FOR loop.")?;
        
        let update = Box::new(self.parse_assignment()?);
        self.consume(&Token::RightParen, "Syntax Error: Expected ')' to close FOR condition.")?;

        self.consume(&Token::StartFor, "Syntax Error: Expected 'START FOR'.")?;
        let mut body = Vec::new();
        while !self.check(&Token::EndFor) && !self.is_at_end() {
            body.push(self.parse_statement()?);
        }
        self.consume(&Token::EndFor, "Syntax Error: Expected 'END FOR'.")?;

        Ok(Statement::For { initialization, condition, update, body })
    }

    // Handles REPEAT WHEN (cond) \n START REPEAT \n ... \n END REPEAT
    fn parse_repeat(&mut self) -> Result<Statement, String> {
        self.advance(); // consume 'REPEAT WHEN'
        self.consume(&Token::LeftParen, "Syntax Error: Expected '(' after REPEAT WHEN.")?;
        let condition = self.parse_expression()?;
        self.consume(&Token::RightParen, "Syntax Error: Expected ')' to close condition.")?;

        self.consume(&Token::StartRepeat, "Syntax Error: Expected 'START REPEAT'.")?;
        let mut body = Vec::new();
        while !self.check(&Token::EndRepeat) && !self.is_at_end() {
            body.push(self.parse_statement()?);
        }
        self.consume(&Token::EndRepeat, "Syntax Error: Expected 'END REPEAT'.")?;

        Ok(Statement::Repeat { condition, body })
    }

    // --- EXPRESSION WATERFALL (Order of Operations) ---

    fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_logical_or()
    }

    // LOGICAL OR
    fn parse_logical_or(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_logical_and()?;
        while self.match_type(&[Token::Or]) {
            let operator = self.previous().clone();
            let right = self.parse_logical_and()?;
            expr = Expression::BinaryOp { left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    // LOGICAL AND
    fn parse_logical_and(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_equality()?;
        while self.match_type(&[Token::And]) {
            let operator = self.previous().clone();
            let right = self.parse_equality()?;
            expr = Expression::BinaryOp { left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    // EQUALITY (==, <>)
    fn parse_equality(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_comparison()?;
        while self.match_type(&[Token::Equal, Token::NotEqual]) {
            let operator = self.previous().clone();
            let right = self.parse_comparison()?;
            expr = Expression::BinaryOp { left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    // COMPARISON (<, >, <=, >=)
    fn parse_comparison(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_term()?;
        while self.match_type(&[Token::LessThan, Token::GreaterThan, Token::LessThanOrEqual, Token::GreaterThanOrEqual]) {
            let operator = self.previous().clone();
            let right = self.parse_term()?;
            expr = Expression::BinaryOp { left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    // TERM (+, -, &)
    fn parse_term(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_factor()?;
        while self.match_type(&[Token::Add, Token::Subtract, Token::Concat]) {
            let operator = self.previous().clone();
            let right = self.parse_factor()?;
            expr = Expression::BinaryOp { left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    // FACTOR (*, /, %)
    fn parse_factor(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_unary()?;
        while self.match_type(&[Token::Multiply, Token::Divide, Token::Modulo]) {
            let operator = self.previous().clone();
            let right = self.parse_unary()?;
            expr = Expression::BinaryOp { left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    // UNARY (-, +, NOT) e.g. -60
    fn parse_unary(&mut self) -> Result<Expression, String> {
        if self.match_type(&[Token::Subtract, Token::Add, Token::Not]) {
            let operator = self.previous().clone();
            let right = Box::new(self.parse_unary()?);
            return Ok(Expression::UnaryOp { operator, right });
        }
        self.parse_primary()
    }

    // PRIMARY (Values)
    fn parse_primary(&mut self) -> Result<Expression, String> {
        let token = self.advance().ok_or("Syntax Error: Unexpected end of file.")?.clone();
        
        match token {
            Token::IntLiteral(n) => Ok(Expression::IntType(n)),
            Token::FloatLiteral(d) => Ok(Expression::FloatType(d)),
            Token::StringLiteral(w) => Ok(Expression::StringType(w)),
            Token::CharLiteral(l) => Ok(Expression::CharType(l)),
            Token::BoolLiteral(t) => Ok(Expression::BoolType(t)),
            Token::Identifier(id) => Ok(Expression::Identifier(id)),
            
            // This is a brilliant trick: If it sees '$' during math/concat, it treats it as a newline string!
            Token::Dollar => Ok(Expression::StringType("\n".to_string())),
            
            Token::LeftParen => {
                let expr = self.parse_expression()?;
                self.consume(&Token::RightParen, "Syntax Error: Expected ')' after expression.")?;
                Ok(expr)
            }

            // Handles [#] by converting anything inside brackets to a literal string (like "#")
            Token::LeftBracket => {
                let mut content = String::new();
                while !self.check(&Token::RightBracket) && !self.is_at_end() {
                    let next = self.advance().unwrap();
                    match next {
                        Token::Identifier(s) => content.push_str(s),
                        _ => content.push_str(""), 
                    }
                }
                self.consume(&Token::RightBracket, "Syntax Error: Expected ']'")?;
                Ok(Expression::StringType(content))
            }

            _ => Err(format!("Syntax Error: I expected a value, variable, or math, but found {:?}", token)),
        }
    }

    // --- HELPER TOOLS ---

    fn check(&self, token_type: &Token) -> bool {
        if self.is_at_end() { return false; }
        std::mem::discriminant(self.peek().unwrap()) == std::mem::discriminant(token_type)
    }

    fn match_type(&mut self, types: &[Token]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, token_type: &Token, message: &str) -> Result<&Token, String> {
        if self.check(token_type) { 
            Ok(self.advance().unwrap()) 
        } else { 
            Err(message.to_string()) 
        }
    }

    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() { self.current += 1; }
        self.previous_opt()
    }

    fn is_at_end(&self) -> bool { self.peek().is_none() }
    fn peek(&self) -> Option<&Token> { self.tokens.get(self.current) }
    
    // NEW Helper: Peek Next (used for chained assignments like x = y = 4)
    fn peek_next(&self) -> Option<&Token> { self.tokens.get(self.current + 1) }
    
    fn previous(&self) -> &Token { &self.tokens[self.current - 1] }
    fn previous_opt(&self) -> Option<&Token> { self.tokens.get(self.current - 1) }
}