#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Illegal,
    Eof,

    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Semicolon,

    // Keywords
    Int,
    Void,
    Return,

    // Values
    Constant(usize),
    Ident(String),
    Number(u64),
}

pub struct Token {
    r#type: TokenType,
    literal: String,
}

impl Token {}

pub fn lookup_ident(ident: &str) -> TokenType {
    match ident {
        "int" => TokenType::Int,
        "void" => TokenType::Void,
        "return" => TokenType::Return,
        _ => TokenType::Ident(ident.to_string()),
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TokenType::Illegal => "ILLEGAL",
            TokenType::Eof => "EOF",
            TokenType::LParen => "(",
            TokenType::RParen => ")",
            TokenType::LBrace => "{",
            TokenType::RBrace => "}",
            TokenType::LBracket => "[",
            TokenType::RBracket => "]",
            TokenType::Semicolon => ";",
            TokenType::Int => "int",
            TokenType::Void => "void",
            TokenType::Return => "return",
            TokenType::Constant(_) => "constant",
            TokenType::Ident(_) => "IDENT",
            TokenType::Number(_) => "NUMBER",
        };
        write!(f, "{}", s)
    }
}
