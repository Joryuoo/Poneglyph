use crate::token::Token;

#[derive(Debug, Clone)]
//espression enum para for things that compute values  
pub enum Expression{
    IntType(i32),
    FloatType(f64),
    StringType(String),
    CharType(char),
    BoolType(bool),
    Identifier(String),

    BinaryOp{
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>, 
    },

    UnaryOp{
        operator: Token,
        right: Box<Expression>,
    }

}
// statements - mga di mu return og value
#[derive(Debug, Clone)]
pub enum Statement{
    // DECLARE DATATYPE var1, var2, var3 = 67;
    Declaration{
        var_type: Token,
        declarations: Vec<(String, Option<Expression>)>, // 
    },

    //assignment
    // NUMBER num = 67;
    Assignment{
        targets: Vec<String>, //mu handle og var1 = var2 = 67 
        value: Expression,
    },

    Increment{
        name: String,
        is_increment: bool,
    },

    Print(Expression),
    Scan(Vec<String>), // vector of variable names to store input

    If {
        condition: Expression,
        body: Vec<Statement>,
        //nesting???
        else_ifs: Vec<(Expression, Vec<Statement>)>, 
        else_body: Option<Vec<Statement>>,
    },

    // FOR (initialization, condition, update) 
    For {
        initialization: Box<Statement>, 
        condition: Expression,
        update: Box<Statement>,         
        body: Vec<Statement>,
    },

    // REPEAT WHEN (<BOOL expression>) 
    Repeat {
        condition: Expression,
        body: Vec<Statement>,
    }
}

// THE ROOT NODE 
#[derive(Debug)]
pub struct Program {
    //  just a massive list of statements executed top to bottom
    pub statements: Vec<Statement>, 
}
