use crate::{lazylexer, typedefs::*};
use core::panic;
use std::io::Read;

use crate::token::TokenType;

pub struct Parser<'a, R: Read> {
    lexer: &'a mut lazylexer::Lexer<R>,
}

impl<'a, R: Read> Parser<'a, R> {
    pub fn new(lexer: &'a mut lazylexer::Lexer<R>) -> Self {
        Self { lexer }
    }

    pub fn parse_program(&mut self) -> Result<Statement, String> {
        let cur_token = self.lexer.advance();
        match cur_token.token_type {
            TokenType::Identifier => {
                let next_token = self.lexer.advance();
                if next_token.token_type == TokenType::Equal {
                    let expr = self.parse_expr().unwrap();
                    return Ok(Statement::Assignment(AssignmentStatement {
                        identifier: cur_token,
                        value: expr,
                    }));
                }
                panic!("UnimplementedError");
            }
            _ => panic!("UnimplementedError"),
        }
    }

    pub fn parse_expr(&mut self) -> Result<Expression, String> {
        let cur_token = self.lexer.advance();
        if cur_token.is_unit() {
            return Ok(Expression::Unit(cur_token));
        } else if cur_token.token_type == TokenType::LBrace {
            let mut table_elements: Vec<TableElement> = vec![];
            loop {
                let cur_token = self.lexer.advance();
                if cur_token.token_type == TokenType::RBrace {
                    break;
                } else if cur_token.is_unit() {
                    let next_token = self.lexer.advance();
                    if next_token.token_type == TokenType::Equal && cur_token.is_key() {
                        let expr = self.parse_expr()?;
                        table_elements.push(TableElement::KeyValueElement(KeyValue {
                            key: cur_token,
                            value: expr,
                        }));
                    } else if next_token.is_unit() {
                        table_elements.push(TableElement::ArrayElement(cur_token));
                        table_elements.push(TableElement::ArrayElement(next_token));
                    } else if next_token.token_type == TokenType::RBrace {
                        table_elements.push(TableElement::ArrayElement(cur_token));
                        break;
                    } else {
                        return Err(format!(
                            "ParserError: unexpected token '{}'",
                            next_token.value
                        ));
                    }
                } else if cur_token.token_type == TokenType::EOF {
                    return Err("ParserError: unexpected end of file".to_string());
                } else if cur_token.token_type == TokenType::Undefined {
                    return Err(format!(
                        "ParserError: undefined token '{}'",
                        cur_token.value
                    ));
                } else {
                    return Err(format!(
                        "ParserError: unexpected token '{}'",
                        cur_token.value
                    ));
                }
            }
            return Ok(Expression::Table(Table {
                elements: table_elements,
            }));
        }
        return Err(format!(
            "ParserError: unexpected token '{}'",
            cur_token.value
        ));
    }
}
