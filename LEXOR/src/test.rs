use std::io::{self, Write};
use crate::lexer;
use crate::parser;
use crate::interpreter;

pub fn run_test() {
    println!("LEXOR Test.");
    println!("Type 'exit' on a new line to RUN the program\n");

    // collect all lines of code
    let mut program_code = String::new();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let trimmed = input.trim();

        // stop muna
        if trimmed.to_lowercase() == "exit" {
            break;
        }

        // new line sa code
        program_code.push_str(&input);
    }

    println!("\n--- Output ---");

    if program_code.trim().is_empty() {
        println!("Ayo ayo, Migo!");
        return;
    }

    // Now run the compiler pipeline ONCE on the entire block of code!
    match lexer::tokenize(&program_code) {
        Ok(tokens) => {
            let mut parser = parser::Parser::new(tokens);
            match parser.parse() {
                Ok(program) => {
                    let mut my_interpreter = interpreter::Interpreter::new("");
                    my_interpreter.interpret(program);
                },
                Err(e) => println!("{}", e), // Using your custom error formatting if you implemented errors.rs!
            }
        }
        Err(e) => println!("{}", e), 
    }
    
    println!("--- Program Finished ---\n");
}