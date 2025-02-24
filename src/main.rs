use std::{env, fs::File};

use typedefs::Statement;

mod json;
mod lazylexer;
mod lazyparser;
mod lexer;
mod parser;
mod token;
mod typedefs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    let file = File::open(file_path).expect("Failed to open file");
    let mut lexer = lazylexer::Lexer::new(file);
    let mut parser = lazyparser::Parser::new(&mut lexer);

    let program = parser.parse_program().unwrap();
    match program {
        Statement::Assignment(assignment) => {
            println!("{}", json::parse_expr_to_json(&assignment.value, 0));
        }
    }
}
