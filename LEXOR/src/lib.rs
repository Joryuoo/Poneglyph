//this is for the Frontend UI 
// for web ass
use wasm_bindgen::prelude::*;

mod token;
mod lexer;
mod ast;
mod parser;
mod interpreter;

#[wasm_bindgen]
pub fn run_agartha_code(source_code: &str) -> String {
    // Lexing
    let tokens = match lexer::tokenize(source_code) {
        Ok(t) => t,
        Err(e) => return format!("Lexer Error: {}", e),
    };

    // Parsing
    let mut parser = parser::Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => return format!("Syntax Error: {}", e),
    };

    // Interpreting
    let mut my_interpreter = interpreter::Interpreter::new();
    my_interpreter.interpret(program);

    // Return the captured output back to JavaScript!
    my_interpreter.output
}