use crate::token::Token;

pub fn tokenize(input: &str) -> Result<Vec<Token>, String>{ // returns vector of tokens if valid else an error string
    let mut tokens = Vec::new();
    let mut  chars = input.chars().peekable();

    //similar to for each loop
    while let Some(&c) = chars.peek(){

        //ignore white space
        if c.is_whitespace(){
            chars.next(); // consume
            continue;
        }

        //pang check sa keyword / identifiers
        if c.is_alphabetic() || c == '_'{
            let mut word = String::new();

            while let Some(&ch) = chars.peek(){
                if ch.is_alphanumeric() || ch == '_' {
                    word.push(ch);
                    chars.next();
                    // .next() will consume the said character
                    // similar siya sa .deque() sa queue maong dili na siya ma read sa outer loop once ma consume
                } else{
                    break;
                }
            }

            //for multi word reading like "START SCRIPT", "SCRIPT AREA", and etc

            let mut lookahead = chars.clone();
            while let Some(&ws) = lookahead.peek(){
                if ws.is_whitespace(){
                    lookahead.next(); //consume whitespace
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
            //pattern matching?

            let token = match word.as_str(){
                //multiword
                //SCRIPT AREA
                "SCRIPT" => {
                    if nextword == "AREA" {
                        chars = lookahead; 
                        Token::ScriptArea 
                    } else{
                        return Err(format!("Error"));
                    }
                },

                // START SCRIPT, START IF, START FOR, START 
                "START" => {
                    match nextword.as_str(){
                        "SCRIPT" => {chars = lookahead; Token::StartScript},
                        "IF" => {chars = lookahead; Token::StartIf},
                        "FOR" => {chars = lookahead; Token::StartFor},
                        "REPEAT" => {chars = lookahead; Token::StartRepeat},
                        _ => return Err(format!("Error!")),
                    }
                    
                },
                // END SCRIPT, END IF, END FOR, END REPEAT
                "END" => {
                    match nextword.as_str(){
                        "SCRIPT" => {chars = lookahead; Token::EndScript},
                        "IF" => {chars = lookahead; Token::EndIf},
                        "FOR" => {chars = lookahead; Token::EndFor},
                        "REPEAT" => {chars = lookahead; Token::EndRepeat},
                        _ => return Err(format!("Error!")),
                    }
                },

                //Else If
                "ELSE" => {
                    if nextword == "IF"{
                        chars = lookahead;
                        Token::ElseIf
                    } else{
                        Token::Else
                    }
                },

                // Repeat When
                "REPEAT" => {
                    if nextword == "WHEN"{
                        chars = lookahead;
                        Token::RepeatWhen
                    } else{
                        return Err(format!("Error"));
                    }
                },
                //SingleWord Keywords
                "DECLARE" => Token::Declare,
                //data types
                "INT" => Token::IntType,
                "FLOAT" => Token::FloatType,
                "CHAR" => Token::CharType,
                "STRING" => Token::StringType, //gi add rasad koni cuz y nut
                "BOOL" => Token::BoolType,
                //control
                "IF" => Token::If,
                "FOR" => Token::For,

                // io
                "PRINT" => Token::Print,
                "SCAN" => Token::Scan,

                "AND" => Token::And,
                "OR" => Token::Or,
                "NOT" => Token::Not,
                
                //if not any then it's an identifier
                _ => Token::Identifier(word),

            };
            tokens.push(token);
            continue;
        }

        //for numbers
        if c.is_ascii_digit(){
            let mut num_str = String::new();
            let mut has_decimal = false;

            //for numbers
            while let Some(&ch) = chars.peek(){
                if ch.is_ascii_digit(){
                    num_str.push(ch);
                    chars.next();
                } else if ch == '.' && !has_decimal{ //for floating point values 
                    num_str.push(ch);
                    has_decimal = true;
                    chars.next();
                } else if ch == '.' && has_decimal{
                    return Err(format!("Error MIGO")) //temporary only
                } else{
                    break;
                }
            }

            if has_decimal{
                //check if numerical values are valid
                if let Ok(val) = num_str.parse::<f64>(){
                    tokens.push(Token::FloatLiteral(val));
                } 
            } else{
                if let Ok(val) = num_str.parse::<i32>(){
                    tokens.push(Token::IntLiteral(val));
                }
            }
            continue;

        }

        //for operators and other symbols
        match c{
            '=' => {
                chars.next(); //consume the initial character
                if let Some(&'=') = chars.peek(){  
                    tokens.push(Token::Equal);
                    chars.next(); // consume the character
                    
                } else{
                    tokens.push(Token::Assign);
                }
            }

            '<' => {
                chars.next(); // consume the initial character
                if let Some(&'=') = chars.peek(){
                    tokens.push(Token::LessThanOrEqual);
                    chars.next(); //consume
                } else if let Some(&'>') = chars.peek(){
                    tokens.push(Token::NotEqual);
                    chars.next();
                } else{
                    tokens.push(Token::LessThan);

                }
            }
            
            '>' => {
                chars.next();
                if let Some(&'=') = chars.peek(){
                    tokens.push(Token::GreaterThanOrEqual);
                    chars.next(); //consume num num
                } else{
                    tokens.push(Token::GreaterThan);
                }
            }
            
            // % - modulo
            // %% - comment
            '%' => {
                chars.next();
                if let Some(&'%') = chars.peek(){
                    while let Some(&ch) = chars.peek(){
                        if ch == '\n' {
                            break;
                        }

                        chars.next(); //consume everything util newline
                    }
                    continue; // balik babaw sa loop to check for other tokens
                } else{
                    tokens.push(Token::Modulo); chars.next();
                }
            }

            //for string literal
            '"' => {
                chars.next(); // consume ang una na "
                let mut val = String::new();  // string literal
                while let Some(&ch) = chars.peek(){
                    if ch == '"' { //end string
                        break;
                    }
                    val.push(ch);
                    chars.next(); //consume character
                }
                //check if naa ang end quote
                if let Some(&'"') = chars.peek(){
                    chars.next(); // consume end quote
                    //rules ni boybesfren  
                    //BOOL – represents the literals true or false
                    if val == "TRUE" {
                        tokens.push(Token::BoolLiteral(true));
                    } else if val == "FALSE" {
                        tokens.push(Token::BoolLiteral(false));
                    } else {
                        tokens.push(Token::StringLiteral(val));
                    }
                    
                } else{
                    return Err(format!("Unsa mani dong! Asa man imong end quote dong?"));
                }
            }

            //character literal
            '\'' => {
                chars.next(); //consume the 1st single quote
                if let Some(&ch) = chars.peek(){
                    chars.next(); //consume the character

                    if let Some(&'\'') = chars.peek() { //look for the end single quote
                        chars.next(); //consume the end single quote
                        tokens.push(Token::CharLiteral(ch)); //store the character literal
                    } else{
                        return Err(format!("Error no enq quote"))
                    }
                }

            }


            // for escape codes
            '[' => {
                chars.next(); // Consume '['
                let mut val = String::new();

                // If the very first thing inside the bracket is ANOTHER bracket (like []]), 
                // capture it instead of breaking immediately
                if let Some(&']') = chars.peek() {
                    val.push(']');
                    chars.next(); // consume the inner ']'
                }

                while let Some(&ch) = chars.peek() {
                    if ch == ']' { break; }
                    val.push(ch);
                    chars.next();
                }
                
                if let Some(&']') = chars.peek() {
                    chars.next(); // Consume closing ']'
                    tokens.push(Token::StringLiteral(val));
                } else {
                    return Err(format!("Lexer Error: Missing closing bracket ']'."));
                }
            }



            //math op
            '/' => {
                tokens.push(Token::Divide);
                chars.next(); 
            }
            '+' => {tokens.push(Token::Add); chars.next();}
            '-' => {tokens.push(Token::Subtract); chars.next();}
            '*' => {tokens.push(Token::Multiply); chars.next();}
            '^' => {tokens.push(Token::Exponentiate); chars.next();} //gi add ra koni
            '$' => {tokens.push(Token::Dollar); chars.next();}
            ',' => {tokens.push(Token::Comma); chars.next();}
            ':' => {tokens.push(Token::Colon) ; chars.next();}
            '&' => {tokens.push(Token::Concat); chars.next();}
            '(' => {tokens.push(Token::LeftParen); chars.next();}
            ')' => {tokens.push(Token::RightParen); chars.next();}
            

            _ => return Err(format!("Unsa mani dong!"))
        }
    }
    Ok(tokens)
}
