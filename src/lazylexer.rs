use crate::token::{Token, TokenType};
use phf::phf_map;
use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Read};

static SINGLE_CHAR_TOKENS: phf::Map<char, TokenType> = phf_map! {
    '{' => TokenType::LBrace,
    '}' => TokenType::RBrace,
    '=' => TokenType::Equal,
};

pub struct Lexer<R: Read> {
    reader: BufReader<R>,
    buffer: VecDeque<char>, // Stores buffered characters for lookahead
    pos: usize,
}

impl<R: Read> Lexer<R> {
    /// Initializes a new lexer with a reader
    pub fn new(reader: R) -> Self {
        Self {
            reader: BufReader::new(reader),
            buffer: VecDeque::new(),
            pos: 0,
        }
    }

    /// Reads and buffers characters lazily
    fn fill_buffer(&mut self) {
        if self.buffer.is_empty() {
            let mut line = String::new();
            if self.reader.read_line(&mut line).unwrap_or(0) > 0 {
                self.buffer.extend(line.chars());
            }
        }
    }

    /// Returns the current character without consuming it
    pub fn get_current_char(&mut self) -> Option<char> {
        self.fill_buffer();
        self.buffer.front().copied()
    }

    /// Consumes and returns the next token
    pub fn advance(&mut self) -> Token {
        self.fill_buffer();
        let current = match self.buffer.pop_front() {
            Some(c) => c,
            None => return Token::eof(),
        };

        if SINGLE_CHAR_TOKENS.contains_key(&current) {
            return Token::new(SINGLE_CHAR_TOKENS[&current].clone(), current.to_string());
        } else if current.is_alphabetic() {
            return self.consume_identifier(current);
        } else if current.is_numeric() {
            return self.consume_numeric(current);
        } else if current == '"' {
            return self.consume_string();
        } else if current.is_whitespace() {
            self.skip_spaces();
            return self.advance();
        } else if current == '#' {
            self.skip_comment();
            return self.advance();
        }

        Token::undefined(current)
    }

    fn consume_identifier(&mut self, first_char: char) -> Token {
        let mut buffer = String::from(first_char);

        while let Some(current) = self.get_current_char() {
            if !current.is_alphanumeric() && current != '_' {
                break;
            }
            buffer.push(self.buffer.pop_front().unwrap());
        }

        Token::new(TokenType::Identifier, buffer)
    }

    fn consume_numeric(&mut self, first_char: char) -> Token {
        let mut buffer = String::from(first_char);
        let mut is_float = false;

        while let Some(current) = self.get_current_char() {
            if current == '.' && !is_float {
                is_float = true;
            } else if !current.is_numeric() {
                break;
            }
            buffer.push(self.buffer.pop_front().unwrap());
        }

        if is_float {
            Token::new(TokenType::Float, buffer)
        } else {
            Token::new(TokenType::Integer, buffer)
        }
    }

    fn consume_string(&mut self) -> Token {
        let mut buffer = String::new();
        let mut is_valid_string = false;

        while let Some(current) = self.buffer.pop_front() {
            if current == '"' {
                is_valid_string = true;
                break;
            }
            buffer.push(current);
        }

        if is_valid_string {
            Token::new(TokenType::String, buffer)
        } else {
            panic!("LexerError: unexpected end of string at {}", self.pos);
        }
    }

    fn skip_comment(&mut self) {
        while let Some(current) = self.get_current_char() {
            if current == '\n' {
                break;
            }
            self.buffer.pop_front();
        }
    }

    fn skip_spaces(&mut self) {
        while let Some(current) = self.get_current_char() {
            if !current.is_whitespace() {
                break;
            }
            self.buffer.pop_front();
        }
    }
}
