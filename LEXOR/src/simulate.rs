use std::fs;
use crate::lexer;
use crate::parser;
use crate::interpreter;

pub fn run_file(filepath: &str) {
    // 1. Try to read the file into a String
    let program_code = match fs::read_to_string(filepath) {
        Ok(content) => content,
        Err(error) => {
            println!("File Error: Cannot read '{}' dong! ({})", filepath, error);
            return;
        }
    };

    println!("--- Executing: {} ---\n", filepath);

    // 2. Feed the file content into the Compiler Pipeline
    match lexer::tokenize(&program_code) {
        Ok(tokens) => {
            let mut parser = parser::Parser::new(tokens);
            match parser.parse() {
                Ok(program) => {
                    let mut my_interpreter = interpreter::Interpreter::new("");
                    my_interpreter.interpret(program);
                    println!("{}", my_interpreter.output);
                },
                Err(e) => println!("{}", e), 
            }
        }
        Err(e) => println!("{}", e), 
    }
    
    println!("\n\n--- Execution Finished ---");
}
