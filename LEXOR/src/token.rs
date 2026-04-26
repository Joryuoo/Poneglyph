#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // readable tokens / keywords
    ScriptArea,
    StartScript,
    EndScript,
    

    Declare, 
    Identifier(String),

    // datatypes
    IntType, 
    FloatType,
    CharType,
    StringType,
    BoolType,
    
    If,
    Else,
    ElseIf,
    StartIf,
    EndIf,

    // lOOPS
    For,
    StartFor,
    EndFor,
    RepeatWhen,
    StartRepeat,
    EndRepeat,
    

    // I/O
    Print, // PRINT:
    Scan, // SCAN:

    // Literals
    IntLiteral(i32), 
    FloatLiteral(f64), 
    CharLiteral(char), 
    StringLiteral(String), 
    BoolLiteral(bool), 

    // Operators
    Assign, // =
    Concat, // &
    Add, // +
    Subtract, // -
    Multiply, // *
    Divide, // /
    Modulo, // %
    Exponentiate, // ^

    //boolean operators
    And, // AND
    Or, // OR
    Not, // NOT

    // logical operators
    Equal, // ==
    NotEqual, // !=
    LessThan, // <
    GreaterThan, // >
    LessThanOrEqual, // <=
    GreaterThanOrEqual, // >=

    // Block Structuring
    LeftParen, // (
    RightParen, // )
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    Dollar,
}   