#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // unit types
    Integer,
    Float,
    Identifier,
    String,

    // symbols
    Equal,
    LBrace,
    RBrace,

    // errors
    EOF,
    Undefined,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Self {
        Token { token_type, value }
    }

    pub fn eof() -> Self {
        Token {
            token_type: TokenType::EOF,
            value: "".to_string(),
        }
    }

    pub fn undefined(character: char) -> Self {
        Token {
            token_type: TokenType::Undefined,
            value: character.to_string(),
        }
    }

    pub fn is_unit(&self) -> bool {
        return self.token_type == TokenType::Identifier
            || self.token_type == TokenType::Integer
            || self.token_type == TokenType::Float
            || self.token_type == TokenType::String;
    }

    pub fn is_key(&self) -> bool {
        return self.token_type == TokenType::Identifier || self.token_type == TokenType::Integer;
    }
}
