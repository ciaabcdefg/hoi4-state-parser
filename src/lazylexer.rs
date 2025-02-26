use crate::token::{Token, TokenType};
use core::panic;
use phf::phf_map;
use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Read};

/// A mapping of one character to its corresponding token
static SINGLE_CHAR_TOKENS: phf::Map<char, TokenType> = phf_map! {
    '{' => TokenType::LBrace,
    '}' => TokenType::RBrace,
    '=' => TokenType::Equal,
};

/// A lazy lexer (reads source code line by line)
pub struct Lexer<R: Read> {
    reader: BufReader<R>,
    buffer: VecDeque<char>, // Stores buffered characters for lookahead
    pos: usize,
}

/// Check if given character is a typical identfiier (alphabetic or starts with an underscore '_')
fn is_char_identifier(c: char) -> bool {
    return c.is_alphabetic() || c == '_';
}

impl<R: Read> Lexer<R> {
    /// Initializes a new lazy lexer with a reader
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
        } else if is_char_identifier(current) {
            return self.consume_identifier(current);
        } else if current.is_numeric() || current == '-' {
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
            if !is_char_identifier(current) {
                break;
            }
            buffer.push(self.buffer.pop_front().unwrap());
        }

        Token::new(TokenType::Identifier, buffer)
    }

    fn consume_numeric(&mut self, first_char: char) -> Token {
        let mut buffer = String::from(first_char);
        let mut is_float = false;
        let mut is_identifier = false;
        let mut is_negative = false;

        while let Some(current) = self.get_current_char() {
            // A negative number must start with a hyphen,
            // and there mustn't be other instances of it
            // otherwise, it's an identifier
            if current == '-' {
                if !is_negative {
                    is_negative = true;
                } else {
                    is_identifier = true;
                }
            }
            // A float must contain only one instance of a dot '.'
            // otherwise, it's an identifier (ex. 1999.31.11 is an identifier, not a float)
            else if current == '.' {
                if is_float {
                    if !is_identifier {
                        is_identifier = true;
                    }
                } else {
                    is_float = true;
                }
            }
            // If a sequence starting with a number contains an identifier character
            // it immediately becomes an identifier
            else if is_char_identifier(current) {
                is_identifier = true;
            }
            // Finally, break if current character isn't a numeric character (if no cases above apply)
            else if !current.is_numeric() {
                break;
            }
            buffer.push(self.buffer.pop_front().unwrap());
        }

        if is_identifier {
            Token::new(TokenType::Identifier, buffer)
        } else if is_float {
            Token::new(TokenType::Float, buffer)
        } else {
            Token::new(TokenType::Integer, buffer)
        }
    }

    fn consume_string(&mut self) -> Token {
        let mut buffer = String::new();
        let mut is_valid_string = false;

        while let Some(current) = self.buffer.pop_front() {
            // Closes the string, which makes the string 'valid'
            // ex. "Hello World" is a valid string,
            // while "Hello World isn't because it isnt' properly terminated with a closing quote
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

    /// Ignores commented lines (starts with '#')
    fn skip_comment(&mut self) {
        while let Some(current) = self.get_current_char() {
            if current == '\n' {
                break;
            }
            self.buffer.pop_front();
        }
    }

    /// Skips through whitespaces until it hits a non-whitespace character
    /// without consuming it
    fn skip_spaces(&mut self) {
        while let Some(current) = self.get_current_char() {
            if !current.is_whitespace() {
                break;
            }
            self.buffer.pop_front();
        }
    }
}
