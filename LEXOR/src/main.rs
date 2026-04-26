//hybrid implementation
// main (source, temporary pa) 
//            |
// Lexical Analyzer (lexer.rs)
//           | lexical units(token.rs / Token)
// Syntax Analyzer (parser.rs)
//           | parse tree (ast.rs)
// Interpreter (interpreter.rs) 

mod token;
mod lexer;
mod ast;
mod parser;
mod interpreter;
mod test; 
mod simulate;
use std::env;

fn main() {
    //test::run_test();

    //simulate.rs
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let input_file = &args[1];
        
        // --- THE EXTENSION ENFORCER ---
        if !input_file.ends_with(".agt") {
            println!("Compiler Error: Unsa mani? I only read Agartha files (.agt) dong!");
            return;
        }

        // --- THE FOLDER ROUTER ---
        // If the user just typed "code.agt", prepend the "scripts/" folder to it.
        // If they already typed a full path (like "other_folder/code.agt"), leave it alone.
        let filepath = if input_file.contains('/') || input_file.contains('\\') {
            input_file.to_string()
        } else {
            format!("scripts/{}", input_file) 
        };
        
        simulate::run_file(&filepath);
    } else {
        test::run_test();
    }
}