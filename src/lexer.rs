// use crate::token::*;
use crate::token::{Token, TokenType};
use phf::phf_map;

static SINGLE_CHAR_TOKENS: phf::Map<char, TokenType> = phf_map! {
    '{' => TokenType::LBrace,
    '}' => TokenType::RBrace,
    '=' => TokenType::Equal,
};

pub struct Lexer {
    pub pos: usize,
    source: String,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self { pos: 0, source }
    }

    pub fn get_current_char(&self) -> Result<char, String> {
        self.source
            .chars()
            .nth(self.pos)
            .ok_or_else(|| format!("Index {} is out of bounds", self.pos))
    }

    pub fn advance(&mut self) -> Token {
        let current = match self.get_current_char() {
            Ok(c) => c,
            Err(_) => return Token::eof(),
        };

        if SINGLE_CHAR_TOKENS.contains_key(&current) {
            self.pos += 1;
            return Token::new(
                SINGLE_CHAR_TOKENS.get(&current).unwrap().clone(),
                current.to_string(),
            );
        } else if current.is_alphabetic() {
            return self.consume_identifier();
        } else if current.is_numeric() {
            return self.consume_numeric();
        } else if current == '"' {
            return self.consume_string();
        } else if current.is_whitespace() {
            self.skip_spaces();
            return self.advance();
        }

        Token::undefined(current)
    }

    fn consume_identifier(&mut self) -> Token {
        let mut buffer: String = String::new();

        while let Ok(current) = self.get_current_char() {
            if !current.is_alphanumeric() && current != '_' {
                break;
            }
            buffer.push(current);
            self.pos += 1;
        }

        Token::new(TokenType::Identifier, buffer)
    }

    fn consume_numeric(&mut self) -> Token {
        let mut buffer: String = String::new();
        let mut is_float: bool = false;
        while let Ok(current) = self.get_current_char() {
            if current == '.' {
                is_float = true;
            } else if !current.is_numeric() {
                break;
            }
            buffer.push(current);
            self.pos += 1;
        }

        match is_float {
            true => Token::new(TokenType::Float, buffer),
            false => Token::new(TokenType::Integer, buffer),
        }
    }

    fn consume_string(&mut self) -> Token {
        let mut buffer: String = String::new();
        let mut is_valid_string: bool = false;
        self.pos += 1;
        while let Ok(current) = self.get_current_char() {
            if current == '"' {
                is_valid_string = true;
                break;
            }
            self.pos += 1;
            buffer.push(current);
        }

        self.pos += 1;
        match is_valid_string {
            true => Token::new(TokenType::String, buffer),
            false => panic!("LexerError: unexpected end of string at {}", self.pos),
        }
    }

    fn skip_spaces(&mut self) {
        while let Ok(current) = self.get_current_char() {
            if !current.is_whitespace() {
                break;
            }
            self.pos += 1;
        }
    }
}
