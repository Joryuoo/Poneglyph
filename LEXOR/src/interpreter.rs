use std::collections::HashMap;
use crate::ast::{Program, Statement, Expression};
use crate::token::Token; 
use std::io::{self, Write}; 

pub struct Interpreter {
    memory: HashMap<String, Expression>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { memory: HashMap::new() }
    }

    pub fn interpret(&mut self, program: Program) {
        for stmt in program.statements {
            if let Err(e) = self.execute_statement(stmt) {
                println!("Runtime Error: {}", e);
                break; 
            }
        }
    }

    // --- STATEMENT EXECUTOR ---
    fn execute_statement(&mut self, stmt: Statement) -> Result<(), String> {
        match stmt {
            // 1. Declarations (DECLARE INT x, y, z = 5)
            Statement::Declaration { var_type, declarations } => {
                for (name, value_opt) in declarations {
                    if let Some(expr) = value_opt {
                        let final_val = self.evaluate_expression(expr)?;
                        
                        // Strict Type Checking mapped exactly to your ast.rs
                        let is_valid = match (&var_type, &final_val) {
                            (Token::IntType, Expression::IntType(_)) => true,
                            (Token::FloatType, Expression::FloatType(_)) => true,
                            (Token::CharType, Expression::CharType(_)) => true,
                            (Token::BoolType, Expression::BoolType(_)) => true,
                            (Token::StringType, Expression::StringType(_)) => true,
                            _ => false, 
                        };

                        if !is_valid {
                            return Err(format!("Type Error: Migo, you declared '{}' as a {:?}, but gave it the wrong data type!", name, var_type));
                        }

                        self.memory.insert(name, final_val);
                    } else {
                        // If no value was given (like 'x' and 'y'), initialize them with a default value 
                        let default_val = match var_type {
                            Token::IntType => Expression::IntType(0),
                            Token::FloatType => Expression::FloatType(0.0),
                            Token::CharType => Expression::CharType(' '), // Blank space
                            Token::BoolType => Expression::BoolType(false),
                            Token::StringType => Expression::StringType("".to_string()),
                            _ => return Err("Type Error: Unknown data type".to_string()),
                        };
                        self.memory.insert(name, default_val);
                    }
                }
                Ok(())
            }

            // 2. Assignments (x = y = 20)
            Statement::Assignment { targets, value } => {
                let final_val = self.evaluate_expression(value)?;
                
                // Assign the computed value to ALL targets in the chain
                for name in targets {
                    if !self.memory.contains_key(&name) {
                        return Err(format!("Unsa man nang '{}'? Wa na gi-declare dong!", name));
                    }
                    // In a strictly typed language, you'd also check if the new value matches the old variable type here!
                    self.memory.insert(name.clone(), final_val.clone());
                }
                Ok(())
            }

            // 3. Input (SCAN: x, y)
            Statement::Scan(targets) => {
                io::stdout().flush().unwrap(); 
                let mut input_text = String::new();
                io::stdin().read_line(&mut input_text).unwrap();
                
                let inputs: Vec<&str> = input_text.trim().split(',').collect();

                if inputs.len() != targets.len() {
                    return Err(format!("Input Error: You need to type exactly {} values separated by commas!", targets.len()));
                }

                for (i, name) in targets.iter().enumerate() {
                    if !self.memory.contains_key(name) {
                        return Err(format!("Unsa man nang '{}'? Wa na gi-declare dong!", name));
                    }

                    let trimmed = inputs[i].trim();

                    // Auto-detect what type of data the user typed
                    let expr = if let Ok(n) = trimmed.parse::<i32>() {
                        Expression::IntType(n)
                    } else if let Ok(d) = trimmed.parse::<f64>() {
                        Expression::FloatType(d)
                    } else if trimmed == "TRUE" {
                        Expression::BoolType(true)
                    } else if trimmed == "FALSE" {
                        Expression::BoolType(false)
                    } else if trimmed.len() == 1 {
                        Expression::CharType(trimmed.chars().next().unwrap())
                    } else {
                        Expression::StringType(trimmed.to_string())
                    };

                    self.memory.insert(name.clone(), expr);
                }
                Ok(())
            }

            // 4. Display (PRINT: x & "hello")
            Statement::Print(expr) => {
                let val = self.evaluate_expression(expr)?;
                // LEXOR uses PRINT: which outputs directly. 
                print!("{}", self.stringify(val)); 
                io::stdout().flush().unwrap();
                Ok(())
            }

            // 5. IF / ELSE IF / ELSE Logic
            Statement::If { condition, body, else_ifs, else_body } => {
                let cond_val = self.evaluate_expression(condition)?;
                let mut block_executed = false;

                // 1. Check the main IF condition
                if let Expression::BoolType(is_true) = cond_val {
                    if is_true {
                        for stmt in body {
                            self.execute_statement(stmt)?;
                        }
                        block_executed = true;
                    }
                } else {
                    return Err("Type Error: Migo, your IF condition must be a BOOL expression!".to_string());
                }

                // 2. Check the ELSE IF conditions (if the main IF was false)
                if !block_executed {
                    for (elif_cond, elif_body) in else_ifs {
                        let elif_val = self.evaluate_expression(elif_cond)?;
                        if let Expression::BoolType(is_true) = elif_val {
                            if is_true {
                                for stmt in elif_body {
                                    self.execute_statement(stmt)?;
                                }
                                block_executed = true;
                                break; // We found a true condition, stop checking the others!
                            }
                        } else {
                            return Err("Type Error: Your ELSE IF condition must be a BOOL expression!".to_string());
                        }
                    }
                }

                // 3. Run the ELSE block (if absolutely nothing else was true)
                if !block_executed {
                    if let Some(e_body) = else_body {
                        for stmt in e_body {
                            self.execute_statement(stmt)?;
                        }
                    }
                }

                Ok(())
            }

            // 6. FOR Loop
            Statement::For { initialization, condition, update, body } => {
                // 1. Run the initialization step exactly once
                self.execute_statement(*initialization)?;

                // 2. Start the continuous loop
                loop {
                    // Check the condition
                    let cond_val = self.evaluate_expression(condition.clone())?;
                    
                    if let Expression::BoolType(is_true) = cond_val {
                        if !is_true { 
                            break; // Exit the loop if the condition is FALSE
                        }

                        // Run everything inside the loop body
                        for stmt in &body {
                            self.execute_statement(stmt.clone())?; 
                        }

                        // Run the update step (e.g., x = x + 1)
                        self.execute_statement(*update.clone())?;

                    } else {
                        return Err("Type Error: FOR loop condition must be a BOOL!".to_string());
                    }
                }
                Ok(())
            }

            // 7. REPEAT WHEN Loop (Acts like a While Loop)
            Statement::Repeat { condition, body } => {
                loop {
                    // Re-evaluate the condition at the start of every loop
                    let cond_val = self.evaluate_expression(condition.clone())?;
                    
                    if let Expression::BoolType(is_true) = cond_val {
                        if !is_true { 
                            break; // Stop repeating if it becomes FALSE
                        }

                        // Run everything inside the loop body
                        for stmt in &body {
                            self.execute_statement(stmt.clone())?;
                        }
                    } else {
                        return Err("Type Error: REPEAT WHEN condition must be a BOOL!".to_string());
                    }
                }
                Ok(())
            }

            // Catch anything missed
            _ => Err("Statement execution failed or is unrecognized.".to_string())

        }
    }

    // --- EXPRESSION CALCULATOR ---
    fn evaluate_expression(&mut self, expr: Expression) -> Result<Expression, String> {
        match expr {
            // Raw Values (Mapped to your AST names)
            Expression::IntType(n) => Ok(Expression::IntType(n)),
            Expression::FloatType(d) => Ok(Expression::FloatType(d)),
            Expression::StringType(w) => Ok(Expression::StringType(w)),
            Expression::BoolType(t) => Ok(Expression::BoolType(t)),
            Expression::CharType(l) => Ok(Expression::CharType(l)),

            // Variables
            Expression::Identifier(name) => {
                if let Some(val) = self.memory.get(&name) {
                    Ok(val.clone())
                } else {
                    Err(format!("Variable '{}' not found!", name))
                }
            }

            // Unary Operations (e.g., -60 or +5)
            Expression::UnaryOp { operator, right } => {
                let right_val = self.evaluate_expression(*right)?;
                match operator {
                    Token::Subtract => {
                        match right_val {
                            Expression::IntType(n) => Ok(Expression::IntType(-n)),
                            Expression::FloatType(d) => Ok(Expression::FloatType(-d)),
                            _ => Err("Math Error: Can only make numbers negative.".to_string())
                        }
                    },
                    Token::Add => Ok(right_val), // Positive just returns the number

                    Token::Not => {
                        match right_val {
                            Expression::BoolType(b) => Ok(Expression::BoolType(!b)),
                            _ => Err("Logic Error: NOT only works with BOOL types.".to_string())
                        }
                    },
                    _ => Err(format!("Invalid unary operator: {:?}", operator))
                }
            }

            // Binary Operations (Math and Concat)
            Expression::BinaryOp { left, operator, right } => {
                let left_val = self.evaluate_expression(*left)?;
                let right_val = self.evaluate_expression(*right)?;

                match operator {
                    // CONCATENATION (&)
                    Token::Concat => {
                        let l_str = self.stringify(left_val);
                        let r_str = self.stringify(right_val);
                        Ok(Expression::StringType(format!("{}{}", l_str, r_str)))
                    },
                    
                    // ADDITION (+)
                    Token::Add => {
                        match (left_val, right_val) {
                            (Expression::IntType(l), Expression::IntType(r)) => Ok(Expression::IntType(l + r)),
                            (Expression::FloatType(l), Expression::FloatType(r)) => Ok(Expression::FloatType(l + r)),
                            _ => Err("Math Error: Can only add matching numbers together.".to_string())
                        }
                    },

                    // SUBTRACTION (-)
                    Token::Subtract => {
                        match (left_val, right_val) {
                            (Expression::IntType(l), Expression::IntType(r)) => Ok(Expression::IntType(l - r)),
                            (Expression::FloatType(l), Expression::FloatType(r)) => Ok(Expression::FloatType(l - r)),
                            _ => Err("Math Error: Can only subtract numbers.".to_string())
                        }
                    },
                    
                    // MULTIPLICATION (*)
                    Token::Multiply => {
                        match (left_val, right_val) {
                            (Expression::IntType(l), Expression::IntType(r)) => Ok(Expression::IntType(l * r)),
                            (Expression::FloatType(l), Expression::FloatType(r)) => Ok(Expression::FloatType(l * r)),
                            _ => Err("Math Error: Can only multiply numbers.".to_string())
                        }
                    },
                    
                    // DIVISION (/)
                    Token::Divide => {
                        match (left_val, right_val) {
                            (Expression::IntType(l), Expression::IntType(r)) => {
                                if r == 0 { return Err("Math Error: Division by zero!".to_string()); }
                                Ok(Expression::IntType(l / r))
                            },
                            (Expression::FloatType(l), Expression::FloatType(r)) => {
                                if r == 0.0 { return Err("Math Error: Division by zero!".to_string()); }
                                Ok(Expression::FloatType(l / r))
                            },
                            _ => Err("Math Error: Can only divide numbers.".to_string())
                        }
                    },

                    // MODULO (%) - Required by LEXOR
                    Token::Modulo => {
                        match (left_val, right_val) {
                            (Expression::IntType(l), Expression::IntType(r)) => {
                                if r == 0 { return Err("Math Error: Modulo by zero!".to_string()); }
                                Ok(Expression::IntType(l % r))
                            },
                            _ => Err("Math Error: Modulo only works with INT types.".to_string())
                        }
                    },

                    //RELATIONAL OPERATORS 
                    Token::LessThan => {
                        match (left_val, right_val) {
                            (Expression::IntType(l), Expression::IntType(r)) => Ok(Expression::BoolType(l < r)),
                            (Expression::FloatType(l), Expression::FloatType(r)) => Ok(Expression::BoolType(l < r)),
                            _ => Err("Math Error: Can only compare numbers.".to_string())
                        }
                    },
                    Token::GreaterThan => {
                        match (left_val, right_val) {
                            (Expression::IntType(l), Expression::IntType(r)) => Ok(Expression::BoolType(l > r)),
                            (Expression::FloatType(l), Expression::FloatType(r)) => Ok(Expression::BoolType(l > r)),
                            _ => Err("Math Error: Can only compare numbers.".to_string())
                        }
                    },
                    Token::Equal => {
                        match (left_val, right_val) {
                            (Expression::IntType(l), Expression::IntType(r)) => Ok(Expression::BoolType(l == r)),
                            (Expression::FloatType(l), Expression::FloatType(r)) => Ok(Expression::BoolType(l == r)),
                            (Expression::BoolType(l), Expression::BoolType(r)) => Ok(Expression::BoolType(l == r)),
                            (Expression::StringType(l), Expression::StringType(r)) => Ok(Expression::BoolType(l == r)),
                            (Expression::CharType(l), Expression::CharType(r)) => Ok(Expression::BoolType(l == r)),
                            _ => Err("Type Error: Cannot compare different data types.".to_string())
                        }
                    },
                    Token::NotEqual => {
                        match (left_val, right_val) {
                            (Expression::IntType(l), Expression::IntType(r)) => Ok(Expression::BoolType(l != r)),
                            (Expression::FloatType(l), Expression::FloatType(r)) => Ok(Expression::BoolType(l != r)),
                            (Expression::BoolType(l), Expression::BoolType(r)) => Ok(Expression::BoolType(l != r)),
                            (Expression::StringType(l), Expression::StringType(r)) => Ok(Expression::BoolType(l != r)),
                            (Expression::CharType(l), Expression::CharType(r)) => Ok(Expression::BoolType(l != r)),
                            _ => Err("Type Error: Cannot compare different data types.".to_string())
                        }
                    },

                    Token::LessThanOrEqual => {
                        match (left_val, right_val) {
                            (Expression::IntType(l), Expression::IntType(r)) => Ok(Expression::BoolType(l <= r)),
                            (Expression::FloatType(l), Expression::FloatType(r)) => Ok(Expression::BoolType(l <= r)),
                            _ => Err("Math Error: Can only compare numbers.".to_string())
                        }
                    },
                    Token::GreaterThanOrEqual => {
                        match (left_val, right_val) {
                            (Expression::IntType(l), Expression::IntType(r)) => Ok(Expression::BoolType(l >= r)),
                            (Expression::FloatType(l), Expression::FloatType(r)) => Ok(Expression::BoolType(l >= r)),
                            _ => Err("Math Error: Can only compare numbers.".to_string())
                        }
                    },

                    // LOGICAL OPERATORS (AND, OR)
                    Token::And => {
                        match (left_val, right_val) {
                            (Expression::BoolType(l), Expression::BoolType(r)) => Ok(Expression::BoolType(l && r)),
                            _ => Err("Logic Error: AND only works with BOOL types.".to_string())
                        }
                    },
                    Token::Or => {
                        match (left_val, right_val) {
                            (Expression::BoolType(l), Expression::BoolType(r)) => Ok(Expression::BoolType(l || r)),
                            _ => Err("Logic Error: OR only works with BOOL types.".to_string())
                        }
                    },

                    _ => Err(format!("Operator {:?} is not fully implemented yet!", operator))
                }
            }
            
            // Catch anything else
            _ => Err("Expression execution failed.".to_string()),
        }
    }

    // --- HELPER TOOL ---
    fn stringify(&self, expr: Expression) -> String {
        match expr {
            Expression::IntType(n) => n.to_string(),
            Expression::FloatType(d) => d.to_string(),
            Expression::StringType(w) => w,
            Expression::BoolType(t) => {
                if t { "TRUE".to_string() } else { "FALSE".to_string() } // LEXOR uses uppercase TRUE/FALSE
            },
            Expression::CharType(l) => l.to_string(),
            _ => "".to_string(),
        }
    }
}