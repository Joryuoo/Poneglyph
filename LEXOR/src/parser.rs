use crate::token::Token;
use crate::ast::{Program, Statement, Expression};

pub struct Parser {
    tokens: Vec<(Token, usize)>, 
    current: usize,
    seen_executable: bool, 
    last_parsed_line: usize, // --- NEW: Tracks the line of the previous statement ---
}

impl Parser {
    // CONSTRUCTOR 
    pub fn new(tokens: Vec<(Token, usize)>) -> Self {
        Parser { tokens, current: 0, seen_executable: false, last_parsed_line: 0 }
    }

    fn line(&self) -> usize {
        if self.is_at_end() {
            self.tokens.last().map(|(_, l)| *l).unwrap_or(1)
        } else {
            self.tokens[self.current].1
        }
    }

    // --- NEW HELPER: Get the line of the token we just processed ---
    fn previous_line(&self) -> usize {
        if self.current > 0 {
            self.tokens[self.current - 1].1
        } else {
            0
        }
    }

    // --- THE BOUNCER: Enforces 1 statement per line ---
    fn step_statement(&mut self) -> Result<Statement, String> {
        let current_line = self.line();
        
        // If the NEW statement starts on the exact same line the LAST statement ended on...
        if self.last_parsed_line != 0 && current_line == self.last_parsed_line {
            return Err(format!("Syntax Error on Line {}: Multiple statements on the same line are not allowed. Please use a newline.", current_line));
        }
        
        let stmt = self.parse_statement()?; // Parse the statement normally
        self.last_parsed_line = self.previous_line(); // Record exactly where this statement ended
        
        Ok(stmt)
    }

    // --- MAIN ENGINE ---
    pub fn parse(&mut self) -> Result<Program, String> {
        if self.is_at_end() {
            return Err("Syntax Error: Migo, your file is completely empty! You need at least a 'SCRIPT AREA'.".to_string());
        }

        let mut statements = Vec::new();
        
        self.consume(&Token::ScriptArea, "LEXOR programs MUST begin with 'SCRIPT AREA'")?;
        self.consume(&Token::StartScript, "Expected 'START SCRIPT' after 'SCRIPT AREA'")?;

        // Execution Loop - Now uses step_statement!
        while !self.is_at_end() && !self.check(&Token::EndScript) {
            statements.push(self.step_statement()?); 
        }
        
        self.consume(&Token::EndScript, "LEXOR programs MUST finish with 'END SCRIPT'")?;

        if !self.is_at_end() {
            let rogue_token = self.peek().unwrap();
            if std::mem::discriminant(rogue_token) == std::mem::discriminant(&Token::EndScript) {
                return Err(format!("Syntax Error on Line {}: You can only have ONE 'END SCRIPT' per file!", self.line()));
            }
            return Err(format!("Syntax Error on Line {}: No code is allowed after 'END SCRIPT'! I found: {:?}", self.line(), rogue_token));
        }

        Ok(Program { statements })
    }

    // --- STATEMENT ROUTER ---
    fn parse_statement(&mut self) -> Result<Statement, String> {
        if self.check(&Token::ScriptArea) {
            return Err(format!("Syntax Error on Line {}: You already declared 'SCRIPT AREA'. You can only have ONE per file!", self.line()));
        }
        if self.check(&Token::StartScript) {
            return Err(format!("Syntax Error on Line {}: You already declared 'START SCRIPT'. You can only have ONE per file!", self.line()));
        }

        // --- BUG 1 FIX: Blocks declarations after executable code ---
        if self.check(&Token::Declare) { 
            if self.seen_executable {
                return Err(format!("Syntax Error on Line {}: Declarations must be placed immediately after START SCRIPT. You cannot declare variables after executable code.", self.line()));
            }
            return self.parse_declaration(); 
        }

        // Lock out future declarations
        self.seen_executable = true;

        if self.check(&Token::Print) { return self.parse_print(); }
        if self.check(&Token::Scan) { return self.parse_scan(); }
        if self.check(&Token::If) { return self.parse_if(); }
        if self.check(&Token::For) { return self.parse_for(); }
        if self.check(&Token::RepeatWhen) { return self.parse_repeat(); }
        
        self.parse_assignment()
    }

    // STATEMENT BUILDERS

    fn parse_declaration(&mut self) -> Result<Statement, String> {
        self.advance(); 
        if !self.match_type(&[Token::IntType, Token::FloatType, Token::CharType, Token::BoolType, Token::StringType]) {
            return Err(format!("Syntax Error on Line {}: Expected a valid data type after DECLARE.", self.line()));
        }
        let var_type = self.previous().clone();
        
        let mut declarations = Vec::new();

        loop {
            let name = match self.advance() {
                Some(Token::Identifier(n)) => n.clone(),
                _ => return Err(format!("Syntax Error on Line {}: Expected variable name in declaration.", self.line())),
            };

            let value = if self.match_type(&[Token::Assign]) {
                Some(self.parse_expression()?)
            } else {
                None
            };

            declarations.push((name, value));

            if !self.match_type(&[Token::Comma]) {
                break;
            }
        }
        Ok(Statement::Declaration { var_type, declarations })
    }

    fn parse_assignment(&mut self) -> Result<Statement, String> {
        let mut targets = Vec::new();
        
        let first_name = match self.advance() {
            Some(Token::Identifier(n)) => n.clone(),
            _ => return Err(format!("Syntax Error on Line {}: Expected a variable name or valid statement.", self.line())),
        };
        targets.push(first_name);

        self.consume(&Token::Assign, "Expected '=' after variable.")?;

        while let Some(Token::Identifier(next_name)) = self.peek().cloned() {
            if let Some(Token::Assign) = self.peek_next() {
                targets.push(next_name);
                self.advance(); 
                self.advance(); 
            } else {
                break;
            }
        }

        let value = self.parse_expression()?;
        Ok(Statement::Assignment { targets, value })
    }

    fn parse_print(&mut self) -> Result<Statement, String> {
        self.advance(); 
        self.consume(&Token::Colon, "Expected ':' after PRINT.")?;
        let expr = self.parse_expression()?;
        Ok(Statement::Print(expr))
    }

    fn parse_scan(&mut self) -> Result<Statement, String> {
        self.advance(); 
        self.consume(&Token::Colon, "Expected ':' after SCAN.")?;
        
        let mut targets = Vec::new();
        loop {
            match self.advance() {
                Some(Token::Identifier(n)) => targets.push(n.clone()),
                _ => return Err(format!("Syntax Error on Line {}: Expected variable name for SCAN.", self.line())),
            };

            if !self.match_type(&[Token::Comma]) {
                break;
            }
        }
        Ok(Statement::Scan(targets))
    }

    fn parse_if(&mut self) -> Result<Statement, String> {
        self.advance(); 
        self.consume(&Token::LeftParen, "Expected '(' after IF.")?;
        let condition = self.parse_expression()?;
        self.consume(&Token::RightParen, "Expected ')' after condition.")?;

        self.consume(&Token::StartIf, "Expected 'START IF'.")?;
        let mut body = Vec::new();
        while !self.check(&Token::EndIf) && !self.check(&Token::ElseIf) && !self.check(&Token::Else) && !self.is_at_end() {
            body.push(self.step_statement()?); // Now uses step_statement!
        }
        self.consume(&Token::EndIf, "Expected 'END IF'.")?;

        let mut else_ifs = Vec::new();
        while self.match_type(&[Token::ElseIf]) {
            self.consume(&Token::LeftParen, "Expected '(' after ELSE IF.")?;
            let cond = self.parse_expression()?;
            self.consume(&Token::RightParen, "Expected ')' after condition.")?;
            
            self.consume(&Token::StartIf, "Expected 'START IF'.")?;
            let mut elif_body = Vec::new();
            while !self.check(&Token::EndIf) && !self.is_at_end() {
                elif_body.push(self.step_statement()?); // Now uses step_statement!
            }
            self.consume(&Token::EndIf, "Expected 'END IF'.")?;
            else_ifs.push((cond, elif_body));
        }

        let mut else_body = None;
        if self.match_type(&[Token::Else]) {
            self.consume(&Token::StartIf, "Expected 'START IF' after ELSE.")?;
            let mut e_body = Vec::new();
            while !self.check(&Token::EndIf) && !self.is_at_end() {
                e_body.push(self.step_statement()?); // Now uses step_statement!
            }
            self.consume(&Token::EndIf, "Expected 'END IF'.")?;
            else_body = Some(e_body);
        }

        Ok(Statement::If { condition, body, else_ifs, else_body })
    }

    fn parse_for(&mut self) -> Result<Statement, String> {
        self.advance(); 
        self.consume(&Token::LeftParen, "Expected '(' after FOR.")?;
        
        // Allowed on the same line since they are strictly part of the FOR construct
        let initialization = Box::new(self.parse_assignment()?);
        self.consume(&Token::Comma, "Expected ',' after initialization in FOR loop.")?;
        
        let condition = self.parse_expression()?;
        self.consume(&Token::Comma, "Expected ',' after condition in FOR loop.")?;
        
        let update = Box::new(self.parse_assignment()?);
        self.consume(&Token::RightParen, "Expected ')' to close FOR condition.")?;

        self.consume(&Token::StartFor, "Expected 'START FOR'.")?;
        let mut body = Vec::new();
        while !self.check(&Token::EndFor) && !self.is_at_end() {
            body.push(self.step_statement()?); // Now uses step_statement!
        }
        self.consume(&Token::EndFor, "Expected 'END FOR'.")?;

        Ok(Statement::For { initialization, condition, update, body })
    }

    fn parse_repeat(&mut self) -> Result<Statement, String> {
        self.advance(); 
        self.consume(&Token::LeftParen, "Expected '(' after REPEAT WHEN.")?;
        let condition = self.parse_expression()?;
        self.consume(&Token::RightParen, "Expected ')' to close condition.")?;

        self.consume(&Token::StartRepeat, "Expected 'START REPEAT'.")?;
        let mut body = Vec::new();
        while !self.check(&Token::EndRepeat) && !self.is_at_end() {
            body.push(self.step_statement()?); // Now uses step_statement!
        }
        self.consume(&Token::EndRepeat, "Expected 'END REPEAT'.")?;

        Ok(Statement::Repeat { condition, body })
    }

    // EXPRESSION WATERFALL

    fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_logical_and()?;
        while self.match_type(&[Token::Or]) {
            let operator = self.previous().clone();
            let right = self.parse_logical_and()?;
            expr = Expression::BinaryOp { left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn parse_logical_and(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_equality()?;
        while self.match_type(&[Token::And]) {
            let operator = self.previous().clone();
            let right = self.parse_equality()?;
            expr = Expression::BinaryOp { left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_comparison()?;
        while self.match_type(&[Token::Equal, Token::NotEqual]) {
            let operator = self.previous().clone();
            let right = self.parse_comparison()?;
            expr = Expression::BinaryOp { left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_term()?;
        while self.match_type(&[Token::LessThan, Token::GreaterThan, Token::LessThanOrEqual, Token::GreaterThanOrEqual]) {
            let operator = self.previous().clone();
            let right = self.parse_term()?;
            expr = Expression::BinaryOp { left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_factor()?;
        while self.match_type(&[Token::Add, Token::Subtract, Token::Concat]) {
            let operator = self.previous().clone();
            let right = self.parse_factor()?;
            expr = Expression::BinaryOp { left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_unary()?;
        while self.match_type(&[Token::Multiply, Token::Divide, Token::Modulo]) {
            let operator = self.previous().clone();
            let right = self.parse_unary()?;
            expr = Expression::BinaryOp { left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expression, String> {
        if self.match_type(&[Token::Subtract, Token::Add, Token::Not]) {
            let operator = self.previous().clone();
            let right = Box::new(self.parse_unary()?);
            return Ok(Expression::UnaryOp { operator, right });
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        let current_line = self.line(); 
        
        let token = self.advance()
            .ok_or(format!("Syntax Error on Line {}: Unexpected end of file.", current_line))?
            .clone();
        
        match token {
            Token::IntLiteral(n) => Ok(Expression::IntType(n)),
            Token::FloatLiteral(d) => Ok(Expression::FloatType(d)),
            Token::StringLiteral(w) => Ok(Expression::StringType(w)),
            Token::CharLiteral(l) => Ok(Expression::CharType(l)),
            Token::BoolLiteral(t) => Ok(Expression::BoolType(t)),
            Token::Identifier(id) => Ok(Expression::Identifier(id)),
            
            Token::Dollar => Ok(Expression::StringType("\n".to_string())),
            
            Token::LeftParen => {
                let expr = self.parse_expression()?;
                self.consume(&Token::RightParen, "Expected ')' after expression.")?;
                Ok(expr)
            }

            Token::LeftBracket => {
                let mut content = String::new();
                while !self.check(&Token::RightBracket) && !self.is_at_end() {
                    let next = self.advance().unwrap();
                    match next {
                        Token::Identifier(s) => content.push_str(s),
                        _ => content.push_str(""), 
                    }
                }
                self.consume(&Token::RightBracket, "Expected ']'")?;
                Ok(Expression::StringType(content))
            }

            _ => Err(format!("Syntax Error on Line {}: I expected a value, variable, or math, but found {:?}", current_line, token)),
        }
    }

    // helper functions for navigating tokens
    fn is_at_end(&self) -> bool { self.peek().is_none() }
    
    fn peek(&self) -> Option<&Token> { 
        self.tokens.get(self.current).map(|(t, _)| t) 
    }
    
    fn peek_next(&self) -> Option<&Token> { 
        self.tokens.get(self.current + 1).map(|(t, _)| t) 
    }
    
    fn previous(&self) -> &Token { 
        &self.tokens[self.current - 1].0 
    }
    
    fn previous_opt(&self) -> Option<&Token> { 
        self.tokens.get(self.current - 1).map(|(t, _)| t) 
    }

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
            Err(format!("Syntax Error on Line {}: {}", self.line(), message)) 
        }
    }

    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() { self.current += 1; }
        self.previous_opt()
    }
}