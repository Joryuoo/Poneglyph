//this is for the Frontend UI 
// for web ass
use wasm_bindgen::prelude::*;

mod token;
mod lexer;
mod ast;
mod parser;
mod interpreter;

fn json_escape(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        match c {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                result.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => result.push(c),
        }
    }
    result
}

#[wasm_bindgen]
pub fn run_agartha_code(source_code: &str, scan_inputs: &str) -> String {
    // Lexing
    let tokens = match lexer::tokenize(source_code) {
        Ok(t) => t,
        Err(e) => return format!("{{\"status\":\"error\",\"output\":\"{}\"}}", json_escape(&e)),
    };

    // Parsing
    let mut parser = parser::Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => return format!("{{\"status\":\"error\",\"output\":\"{}\"}}", json_escape(&e)),
    };

    // Interpreting — pass scan inputs from frontend
    let mut my_interpreter = interpreter::Interpreter::new(scan_inputs);
    my_interpreter.interpret(program);

    // Return structured JSON so the frontend knows what happened
    if my_interpreter.needs_input {
        format!(
            "{{\"status\":\"scan_wait\",\"output\":\"{}\",\"count\":{}}}",
            json_escape(&my_interpreter.output),
            my_interpreter.scan_target_count
        )
    } else {
        format!(
            "{{\"status\":\"done\",\"output\":\"{}\"}}",
            json_escape(&my_interpreter.output)
        )
    }
}