use std::{fs::File, time::Instant};

use crate::{json, lazylexer, lazyparser, lexer, parser, token::TokenType, typedefs::Statement};

pub fn test_lazy_lexer() {
    let file = File::open("input/state.txt").expect("Failed to open file");
    let mut lexer = lazylexer::Lexer::new(file);
    loop {
        let token = lexer.advance();
        if token.token_type == TokenType::EOF || token.token_type == TokenType::Undefined {
            break;
        }
    }
}

pub fn test_eager_lexer() {
    let source = match std::fs::read_to_string("input/state.txt") {
        Ok(file) => file,
        Err(error) => panic!("{}", error),
    };
    let mut lexer = lexer::Lexer::new(source);
    loop {
        let token = lexer.advance();
        if token.token_type == TokenType::EOF || token.token_type == TokenType::Undefined {
            break;
        }
    }
}

pub fn test_lazy_parser() {
    let file = File::open("input/state.txt").expect("Failed to open file");
    let mut lexer = lazylexer::Lexer::new(file);
    let mut parser = lazyparser::Parser::new(&mut lexer);
    parser.parse_program().unwrap();
}

pub fn test_eager_parser() {
    let source = match std::fs::read_to_string("input/state.txt") {
        Ok(file) => file,
        Err(error) => panic!("{}", error),
    };
    let mut lexer = lexer::Lexer::new(source);
    let mut parser = parser::Parser::new(&mut lexer);
    parser.parse_program().unwrap();
}

fn compare_lexers() {
    let start_eager = Instant::now();
    test_eager_lexer();
    let duration_eager = start_eager.elapsed();

    let start_lazy = Instant::now();
    test_lazy_lexer();
    let duration_lazy = start_lazy.elapsed();

    println!("Eager lexer time: {:?}", duration_eager);
    println!("Lazy lexer time: {:?}", duration_lazy);
}

pub fn compare_parsers() {
    let start_eager = Instant::now();
    test_eager_parser();
    let duration_eager = start_eager.elapsed();

    let start_lazy = Instant::now();
    test_lazy_parser();
    let duration_lazy = start_lazy.elapsed();

    println!("Eager lexer time: {:?}", duration_eager);
    println!("Lazy lexer time: {:?}", duration_lazy);

    println!(
        "Speedup (lazy/eager) = {:?}",
        duration_eager.div_duration_f64(duration_lazy)
    );
}

pub fn simple_parse() {
    let source = match std::fs::read_to_string("input/state.txt") {
        Ok(file) => file,
        Err(error) => panic!("{}", error),
    };
    let mut lexer = lexer::Lexer::new(source);
    let mut parser = parser::Parser::new(&mut lexer);

    let program = parser.parse_program().unwrap();
    match program {
        Statement::Assignment(assignment) => {
            println!("{}", json::parse_expr_to_json(&assignment.value, 0));
        }
    }
}

pub fn lazy_parse() {
    let file = File::open("input/state.txt").expect("Failed to open file");
    let mut lexer = lazylexer::Lexer::new(file);
    let mut parser = lazyparser::Parser::new(&mut lexer);

    let program = parser.parse_program().unwrap();
    match program {
        Statement::Assignment(assignment) => {
            println!("{}", json::parse_expr_to_json(&assignment.value, 0));
        }
    }
}
