use crate::token::Token;

pub fn tokenize(input: &str) -> Result<Vec<(Token, usize)>, String>{ 
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut current_line = 1;

    while let Some(&c) = chars.peek(){

        // Count newlines for our error tracker!
        if c == '\n' {
            current_line += 1; 
            chars.next();
            continue;
        } else if c.is_whitespace(){
            chars.next(); 
            continue;
        }

        // pang check sa keyword / identifiers
        if c.is_alphabetic() || c == '_'{
            let mut word = String::new();

            while let Some(&ch) = chars.peek(){
                if ch.is_alphanumeric() || ch == '_' {
                    word.push(ch);
                    chars.next();
                } else{
                    break;
                }
            }

            // for multi word reading like "START SCRIPT", "SCRIPT AREA", etc.
            let mut lookahead = chars.clone();
            while let Some(&ws) = lookahead.peek(){
                if ws.is_whitespace(){
                    lookahead.next(); 
                } else{
                    break;
                }
            }

            let mut nextword = String::new();
            while let Some(&nw) = lookahead.peek(){
                if nw.is_alphanumeric() || nw == '_'{ 
                    nextword.push(nw);
                    lookahead.next();
                } else{
                    break;
                }
            }

            let token = match word.as_str(){
                // multiword
                "SCRIPT" => {
                    if nextword == "AREA" {
                        chars = lookahead; 
                        Token::ScriptArea 
                    } else{
                        return Err(format!("Lexer Error on Line {}: Invalid syntax after SCRIPT", current_line));
                    }
                },
                "START" => {
                    match nextword.as_str(){
                        "SCRIPT" => {chars = lookahead; Token::StartScript},
                        "IF" => {chars = lookahead; Token::StartIf},
                        "FOR" => {chars = lookahead; Token::StartFor},
                        "REPEAT" => {chars = lookahead; Token::StartRepeat},
                        _ => return Err(format!("Lexer Error on Line {}: Invalid START command", current_line)),
                    }
                },
                "END" => {
                    match nextword.as_str(){
                        "SCRIPT" => {chars = lookahead; Token::EndScript},
                        "IF" => {chars = lookahead; Token::EndIf},
                        "FOR" => {chars = lookahead; Token::EndFor},
                        "REPEAT" => {chars = lookahead; Token::EndRepeat},
                        _ => return Err(format!("Lexer Error on Line {}: Invalid END command", current_line)),
                    }
                },
                "ELSE" => {
                    if nextword == "IF"{
                        chars = lookahead;
                        Token::ElseIf
                    } else{
                        Token::Else
                    }
                },
                "REPEAT" => {
                    if nextword == "WHEN"{
                        chars = lookahead;
                        Token::RepeatWhen
                    } else{
                        return Err(format!("Lexer Error on Line {}: Invalid REPEAT command", current_line));
                    }
                },
                // SingleWord Keywords
                "DECLARE" => Token::Declare,
                "INT" => Token::IntType,
                "FLOAT" => Token::FloatType,
                "CHAR" => Token::CharType,
                "STRING" => Token::StringType, 
                "BOOL" => Token::BoolType,
                "IF" => Token::If,
                "FOR" => Token::For,
                "PRINT" => Token::Print,
                "SCAN" => Token::Scan,
                "AND" => Token::And,
                "OR" => Token::Or,
                "NOT" => Token::Not,
                _ => Token::Identifier(word),
            };
    
            tokens.push((token, current_line));
            continue;
        }

        // for numbers
        if c.is_ascii_digit(){
            let mut num_str = String::new();
            let mut has_decimal = false;

            while let Some(&ch) = chars.peek(){
                if ch.is_ascii_digit(){
                    num_str.push(ch);
                    chars.next();
                } else if ch == '.' && !has_decimal{ 
                    num_str.push(ch);
                    has_decimal = true;
                    chars.next();
                } else if ch == '.' && has_decimal{
                    return Err(format!("Lexer Error on Line {}: Multiple decimals found in number", current_line)); 
                } else{
                    break;
                }
            }

            if has_decimal{
                if let Ok(val) = num_str.parse::<f64>(){
                    tokens.push((Token::FloatLiteral(val), current_line));
                } 
            } else{
                if let Ok(val) = num_str.parse::<i32>(){
                    tokens.push((Token::IntLiteral(val), current_line));
                }
            }
            continue;
        }

        // for operators and other symbols
        match c{
            '=' => {
                chars.next(); 
                if let Some(&'=') = chars.peek(){  
                    tokens.push((Token::Equal, current_line));
                    chars.next(); 
                } else{
                    tokens.push((Token::Assign, current_line));
                }
            }

            '<' => {
                chars.next(); 
                if let Some(&'=') = chars.peek(){
                    tokens.push((Token::LessThanOrEqual, current_line));
                    chars.next(); 
                } else if let Some(&'>') = chars.peek(){
                    tokens.push((Token::NotEqual, current_line));
                    chars.next();
                } else{
                    tokens.push((Token::LessThan, current_line));
                }
            }
            
            '>' => {
                chars.next();
                if let Some(&'=') = chars.peek(){
                    tokens.push((Token::GreaterThanOrEqual, current_line));
                    chars.next(); 
                } else{
                    tokens.push((Token::GreaterThan, current_line));
                }
            }
            
            '%' => {
                chars.next();

                // %% comment
                if let Some(&'%') = chars.peek() {
                    while let Some(&ch) = chars.peek() {
                        if ch == '\n' {
                            break;
                        }
                        chars.next();
                    }
                    continue;
                }

                // modulo operator
                tokens.push((Token::Modulo, current_line));
            }

            // for string literal
            '"' => {
                chars.next(); 
                let mut val = String::new();  
                while let Some(&ch) = chars.peek(){
                    if ch == '"' { 
                        break;
                    }
                    val.push(ch);
                    chars.next(); 
                }
                
                if let Some(&'"') = chars.peek(){
                    chars.next(); 
                    if val == "TRUE" {
                        tokens.push((Token::BoolLiteral(true), current_line));
                    } else if val == "FALSE" {
                        tokens.push((Token::BoolLiteral(false), current_line));
                    } else {
                        tokens.push((Token::StringLiteral(val), current_line));
                    }
                } else{
                    return Err(format!("Lexer Error on Line {}: Asa man imong end quote dong?", current_line));
                }
            }

            // character literal
            '\'' => {
                chars.next(); 
                if let Some(&ch) = chars.peek(){
                    chars.next(); 
                    if let Some(&'\'') = chars.peek() { 
                        chars.next(); 
                        tokens.push((Token::CharLiteral(ch), current_line)); 
                    } else{
                        return Err(format!("Lexer Error on Line {}: Missing end quote for character", current_line));
                    }
                }
            }

            // for escape codes
            '[' => {
                chars.next(); 
                let mut val = String::new();

                if let Some(&']') = chars.peek() {
                    val.push(']');
                    chars.next(); 
                }

                while let Some(&ch) = chars.peek() {
                    if ch == ']' { break; }
                    val.push(ch);
                    chars.next();
                }
                
                if let Some(&']') = chars.peek() {
                    chars.next(); 
                    tokens.push((Token::StringLiteral(val), current_line));
                } else {
                    return Err(format!("Lexer Error on Line {}: Missing closing bracket ']'", current_line));
                }
            }

            // single character operators
            '/' => { tokens.push((Token::Divide, current_line)); chars.next(); }
            '+' => {
                chars.next();
                if let Some(&'+') = chars.peek() {
                    tokens.push((Token::Increment, current_line));
                    chars.next();
                } else {
                    tokens.push((Token::Add, current_line));
                }
            }
            '-' => {
                chars.next();
                if let Some(&'-') = chars.peek() {
                    tokens.push((Token::Decrement, current_line));
                    chars.next();
                } else {
                    tokens.push((Token::Subtract, current_line));
                }
            }
            '*' => { tokens.push((Token::Multiply, current_line)); chars.next(); }
            '^' => { tokens.push((Token::Exponentiate, current_line)); chars.next(); } 
            '$' => { tokens.push((Token::Dollar, current_line)); chars.next(); }
            ',' => { tokens.push((Token::Comma, current_line)); chars.next(); }
            ':' => { tokens.push((Token::Colon, current_line)); chars.next(); }
            '&' => { tokens.push((Token::Concat, current_line)); chars.next(); }
            '(' => { tokens.push((Token::LeftParen, current_line)); chars.next(); }
            ')' => { tokens.push((Token::RightParen, current_line)); chars.next(); }
            
            _ => return Err(format!("Lexer Error on Line {}: Unsa mani dong! Unrecognized symbol '{}'", current_line, c))
        }
    }
    Ok(tokens)
}
