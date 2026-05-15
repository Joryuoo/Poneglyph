use std::collections::HashMap;
use crate::ast::{Program, Statement, Expression};
use crate::token::Token;

#[derive(Debug, Clone)]
struct RuntimeVariable {
    declared_type: Token,
    value: Expression,
}

pub struct Interpreter {
    memory: HashMap<String, RuntimeVariable>,
    pub output: String,
    scan_inputs: Vec<String>,
    scan_cursor: usize,
    pub needs_input: bool,
    pub scan_target_count: usize,
}

impl Interpreter {
    pub fn new(scan_inputs_raw: &str) -> Self {
        let inputs: Vec<String> = if scan_inputs_raw.trim().is_empty() {
            Vec::new()
        } else {
            scan_inputs_raw.lines().map(|s| s.to_string()).collect()
        };

        Interpreter {
            memory: HashMap::new(),
            output: String::new(),
            scan_inputs: inputs,
            scan_cursor: 0,
            needs_input: false,
            scan_target_count: 0,
        }
    }

    pub fn interpret(&mut self, program: Program) {
        self.needs_input = false;
        self.scan_target_count = 0;

        for stmt in program.statements {
            if let Err(e) = self.execute_statement(stmt) {
                if e == "__SCAN_WAIT__" {
                    self.needs_input = true;
                    return;
                }

                self.output.push_str(&format!("Runtime Error: {}\n", e));
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
                    if self.memory.contains_key(&name) {
                        return Err(format!("Declaration Error: Variable '{}' is already declared.", name));
                    }

                    let value = if let Some(expr) = value_opt {
                        let evaluated = self.evaluate_expression(expr)?;
                        self.coerce_value_for_type(&var_type, evaluated, &name)?
                    } else {
                        self.default_value_for_type(&var_type)?
                    };

                    self.memory.insert(
                        name,
                        RuntimeVariable {
                            declared_type: var_type.clone(),
                            value,
                        },
                    );
                }
                Ok(())
            }

            // 2. Assignments (x = y = 20)
            Statement::Assignment { targets, value } => {
                let evaluated = self.evaluate_expression(value)?;

                for name in targets {
                    let declared_type = self
                        .memory
                        .get(&name)
                        .ok_or_else(|| format!("Unsa man nang '{}'? Wa na gi-declare dong!", name))?
                        .declared_type
                        .clone();

                    let coerced = self.coerce_value_for_type(&declared_type, evaluated.clone(), &name)?;

                    if let Some(variable) = self.memory.get_mut(&name) {
                        variable.value = coerced;
                    }
                }
                Ok(())
            }

            // 3. Input (SCAN: x, y)
            Statement::Scan(targets) => {
                if self.scan_cursor >= self.scan_inputs.len() {
                    self.scan_target_count = targets.len();
                    return Err("__SCAN_WAIT__".to_string());
                }

                let input_text = self.scan_inputs[self.scan_cursor].clone();
                self.scan_cursor += 1;

                // Echo the input to the frontend output, like a terminal.
                self.output.push_str(&input_text);
                self.output.push('\n');

                let inputs: Vec<&str> = input_text.trim().split(',').collect();

                if inputs.len() != targets.len() {
                    return Err(format!("Input Error: You need to type exactly {} values separated by commas!", targets.len()));
                }

                for (i, name) in targets.iter().enumerate() {
                    let declared_type = self
                        .memory
                        .get(name)
                        .ok_or_else(|| format!("Unsa man nang '{}'? Wa na gi-declare dong!", name))?
                        .declared_type
                        .clone();

                    let expr = self.parse_scan_input_for_type(inputs[i].trim(), &declared_type, name)?;

                    if let Some(variable) = self.memory.get_mut(name) {
                        variable.value = expr;
                    }
                }
                Ok(())
            }

            // 4. Display (PRINT: x & "hello")
            Statement::Print(expr) => {
                let val = self.evaluate_expression(expr)?;
                self.output.push_str(&self.value_to_output(val));
                Ok(())
            }

            // 5. IF / ELSE IF / ELSE Logic
            Statement::If { condition, body, else_ifs, else_body } => {
                let cond_val = self.evaluate_expression(condition)?;
                let mut block_executed = false;

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

                if !block_executed {
                    for (elif_cond, elif_body) in else_ifs {
                        let elif_val = self.evaluate_expression(elif_cond)?;
                        if let Expression::BoolType(is_true) = elif_val {
                            if is_true {
                                for stmt in elif_body {
                                    self.execute_statement(stmt)?;
                                }
                                block_executed = true;
                                break;
                            }
                        } else {
                            return Err("Type Error: Your ELSE IF condition must be a BOOL expression!".to_string());
                        }
                    }
                }

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
                self.execute_statement(*initialization)?;

                loop {
                    let cond_val = self.evaluate_expression(condition.clone())?;

                    if let Expression::BoolType(is_true) = cond_val {
                        if !is_true {
                            break;
                        }

                        for stmt in &body {
                            self.execute_statement(stmt.clone())?;
                        }

                        self.execute_statement(*update.clone())?;
                    } else {
                        return Err("Type Error: FOR loop condition must be a BOOL!".to_string());
                    }
                }
                Ok(())
            }

            // 7. REPEAT WHEN Loop
            Statement::Repeat { condition, body } => {
                loop {
                    let cond_val = self.evaluate_expression(condition.clone())?;

                    if let Expression::BoolType(is_true) = cond_val {
                        if !is_true {
                            break;
                        }

                        for stmt in &body {
                            self.execute_statement(stmt.clone())?;
                        }
                    } else {
                        return Err("Type Error: REPEAT WHEN condition must be a BOOL!".to_string());
                    }
                }
                Ok(())
            }

            _ => Err("Statement execution failed or is unrecognized.".to_string()),
        }
    }

    fn default_value_for_type(&self, var_type: &Token) -> Result<Expression, String> {
        match var_type {
            Token::IntType => Ok(Expression::IntType(0)),
            Token::FloatType => Ok(Expression::FloatType(0.0)),
            Token::CharType => Ok(Expression::CharType(' ')),
            Token::BoolType => Ok(Expression::BoolType(false)),
            _ => Err("Type Error: Unknown or unsupported data type.".to_string()),
        }
    }

    fn coerce_value_for_type(
        &self,
        var_type: &Token,
        value: Expression,
        variable_name: &str,
    ) -> Result<Expression, String> {
        match (var_type, value) {
            (Token::IntType, Expression::IntType(n)) => Ok(Expression::IntType(n)),

            // FLOAT can accept whole-number INT literals/values.
            (Token::FloatType, Expression::FloatType(n)) => Ok(Expression::FloatType(n)),
            (Token::FloatType, Expression::IntType(n)) => Ok(Expression::FloatType(n as f64)),

            (Token::CharType, Expression::CharType(c)) => Ok(Expression::CharType(c)),
            (Token::BoolType, Expression::BoolType(b)) => Ok(Expression::BoolType(b)),

            // Give a specific message for quoted/case-wrong BOOL values.
            (Token::BoolType, Expression::StringType(s))
                if s.eq_ignore_ascii_case("true") || s.eq_ignore_ascii_case("false") =>
            {
                Err(format!(
                    "Type Error: BOOL values are case-sensitive. Use \"TRUE\" or \"FALSE\" for '{}'.",
                    variable_name
                ))
            }

            (Token::BoolType, Expression::StringType(_)) => Err(format!(
                "Type Error: '{}' is declared as BOOL. Use only \"TRUE\" or \"FALSE\".",
                variable_name
            )),

            (_, wrong_value) => Err(format!(
                "Type Error: Migo, you declared '{}' as a {:?}, but gave it {:?}.",
                variable_name, var_type, wrong_value
            )),
        }
    }

    fn parse_scan_input_for_type(
        &self,
        input: &str,
        declared_type: &Token,
        variable_name: &str,
    ) -> Result<Expression, String> {
        match declared_type {
            Token::IntType => input
                .parse::<i32>()
                .map(Expression::IntType)
                .map_err(|_| format!("Input Error: '{}' expects an INT value.", variable_name)),

            Token::FloatType => input
                .parse::<f64>()
                .map(Expression::FloatType)
                .map_err(|_| format!("Input Error: '{}' expects a FLOAT value.", variable_name)),

            Token::CharType => {
                let mut chars = input.chars();
                match (chars.next(), chars.next()) {
                    (Some(c), None) => Ok(Expression::CharType(c)),
                    _ => Err(format!("Input Error: '{}' expects a single CHAR value.", variable_name)),
                }
            }

            Token::BoolType => match input {
                "TRUE" => Ok(Expression::BoolType(true)),
                "FALSE" => Ok(Expression::BoolType(false)),
                other if other.eq_ignore_ascii_case("true") || other.eq_ignore_ascii_case("false") => {
                    Err(format!(
                        "Input Error: BOOL values are case-sensitive. Use TRUE or FALSE for '{}'.",
                        variable_name
                    ))
                }
                _ => Err(format!("Input Error: '{}' expects TRUE or FALSE.", variable_name)),
            },

            _ => Err(format!("Input Error: '{}' has an unsupported declared type.", variable_name)),
        }
    }

    // --- EXPRESSION CALCULATOR ---
    fn evaluate_expression(&mut self, expr: Expression) -> Result<Expression, String> {
        match expr {
            Expression::IntType(n) => Ok(Expression::IntType(n)),
            Expression::FloatType(d) => Ok(Expression::FloatType(d)),
            Expression::StringType(w) => Ok(Expression::StringType(w)),
            Expression::BoolType(t) => Ok(Expression::BoolType(t)),
            Expression::CharType(l) => Ok(Expression::CharType(l)),

            Expression::Identifier(name) => {
                if let Some(variable) = self.memory.get(&name) {
                    Ok(variable.value.clone())
                } else {
                    Err(format!("Variable '{}' not found!", name))
                }
            }

            Expression::UnaryOp { operator, right } => {
                let right_val = self.evaluate_expression(*right)?;
                match operator {
                    Token::Subtract => match right_val {
                        Expression::IntType(n) => Ok(Expression::IntType(-n)),
                        Expression::FloatType(d) => Ok(Expression::FloatType(-d)),
                        _ => Err("Math Error: Can only make numbers negative.".to_string()),
                    },
                    Token::Add => match right_val {
                        Expression::IntType(_) | Expression::FloatType(_) => Ok(right_val),
                        _ => Err("Math Error: Unary + only works with numbers.".to_string()),
                    },
                    Token::Not => match right_val {
                        Expression::BoolType(b) => Ok(Expression::BoolType(!b)),
                        _ => Err("Logic Error: NOT only works with BOOL types.".to_string()),
                    },
                    _ => Err(format!("Invalid unary operator: {:?}", operator)),
                }
            }

            Expression::BinaryOp { left, operator, right } => {
                let left_val = self.evaluate_expression(*left)?;
                let right_val = self.evaluate_expression(*right)?;

                match operator {
                    Token::Concat => {
                        let l_str = self.value_to_output(left_val);
                        let r_str = self.value_to_output(right_val);
                        Ok(Expression::StringType(format!("{}{}", l_str, r_str)))
                    }

                    Token::Add => self.numeric_binary_op(left_val, right_val, |l, r| l + r, |l, r| l + r, "add"),
                    Token::Subtract => self.numeric_binary_op(left_val, right_val, |l, r| l - r, |l, r| l - r, "subtract"),
                    Token::Multiply => self.numeric_binary_op(left_val, right_val, |l, r| l * r, |l, r| l * r, "multiply"),

                    Token::Divide => match (left_val, right_val) {
                        (Expression::IntType(_), Expression::IntType(0)) => Err("Math Error: Division by zero!".to_string()),
                        (Expression::IntType(l), Expression::IntType(r)) => Ok(Expression::IntType(l / r)),

                        (l, r) => {
                            let (l, r) = self.as_float_pair(l, r, "divide")?;
                            if r == 0.0 {
                                Err("Math Error: Division by zero!".to_string())
                            } else {
                                Ok(Expression::FloatType(l / r))
                            }
                        }
                    },

                    Token::Modulo => match (left_val, right_val) {
                        (Expression::IntType(l), Expression::IntType(r)) => {
                            if r == 0 {
                                Err("Math Error: Modulo by zero!".to_string())
                            } else {
                                Ok(Expression::IntType(l % r))
                            }
                        }
                        _ => Err("Math Error: Modulo only works with INT types.".to_string()),
                    },

                    Token::LessThan => self.numeric_compare(left_val, right_val, |l, r| l < r),
                    Token::GreaterThan => self.numeric_compare(left_val, right_val, |l, r| l > r),
                    Token::LessThanOrEqual => self.numeric_compare(left_val, right_val, |l, r| l <= r),
                    Token::GreaterThanOrEqual => self.numeric_compare(left_val, right_val, |l, r| l >= r),

                    Token::Equal => self.equals(left_val, right_val, true),
                    Token::NotEqual => self.equals(left_val, right_val, false),

                    Token::And => match (left_val, right_val) {
                        (Expression::BoolType(l), Expression::BoolType(r)) => Ok(Expression::BoolType(l && r)),
                        _ => Err("Logic Error: AND only works with BOOL types.".to_string()),
                    },

                    Token::Or => match (left_val, right_val) {
                        (Expression::BoolType(l), Expression::BoolType(r)) => Ok(Expression::BoolType(l || r)),
                        _ => Err("Logic Error: OR only works with BOOL types.".to_string()),
                    },

                    _ => Err(format!("Operator {:?} is not fully implemented yet!", operator)),
                }
            }

            _ => Err("Expression execution failed.".to_string()),
        }
    }

    fn numeric_binary_op(
        &self,
        left: Expression,
        right: Expression,
        int_op: fn(i32, i32) -> i32,
        float_op: fn(f64, f64) -> f64,
        op_name: &str,
    ) -> Result<Expression, String> {
        match (left, right) {
            (Expression::IntType(l), Expression::IntType(r)) => Ok(Expression::IntType(int_op(l, r))),
            (l, r) => {
                let (l, r) = self.as_float_pair(l, r, op_name)?;
                Ok(Expression::FloatType(float_op(l, r)))
            }
        }
    }

    fn numeric_compare(
        &self,
        left: Expression,
        right: Expression,
        cmp: fn(f64, f64) -> bool,
    ) -> Result<Expression, String> {
        let (l, r) = self.as_float_pair(left, right, "compare")?;
        Ok(Expression::BoolType(cmp(l, r)))
    }

    fn as_float_pair(
        &self,
        left: Expression,
        right: Expression,
        op_name: &str,
    ) -> Result<(f64, f64), String> {
        let l = match left {
            Expression::IntType(n) => n as f64,
            Expression::FloatType(n) => n,
            _ => return Err(format!("Math Error: Can only {} numbers.", op_name)),
        };

        let r = match right {
            Expression::IntType(n) => n as f64,
            Expression::FloatType(n) => n,
            _ => return Err(format!("Math Error: Can only {} numbers.", op_name)),
        };

        Ok((l, r))
    }

    fn equals(
        &self,
        left: Expression,
        right: Expression,
        should_be_equal: bool,
    ) -> Result<Expression, String> {
        let result = match (left, right) {
            (Expression::IntType(l), Expression::IntType(r)) => l == r,
            (Expression::FloatType(l), Expression::FloatType(r)) => l == r,
            (Expression::IntType(l), Expression::FloatType(r)) => (l as f64) == r,
            (Expression::FloatType(l), Expression::IntType(r)) => l == (r as f64),
            (Expression::BoolType(l), Expression::BoolType(r)) => l == r,
            (Expression::StringType(l), Expression::StringType(r)) => l == r,
            (Expression::CharType(l), Expression::CharType(r)) => l == r,
            _ => return Err("Type Error: Cannot compare incompatible data types.".to_string()),
        };

        Ok(Expression::BoolType(if should_be_equal { result } else { !result }))
    }

    // Converts runtime values into the exact text that should be sent to the frontend output.
    fn value_to_output(&self, expr: Expression) -> String {
        match expr {
            Expression::IntType(n) => n.to_string(),
            Expression::FloatType(d) => d.to_string(),
            Expression::StringType(w) => w,
            Expression::BoolType(t) => {
                if t { "TRUE".to_string() } else { "FALSE".to_string() }
            }
            Expression::CharType(l) => l.to_string(),
            _ => "".to_string(),
        }
    }
}
