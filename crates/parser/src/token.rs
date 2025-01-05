
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // keywords
    KwInt,
    KwFloat,
    KwUint,
    KwString,

    // symbols
    Equal,
    Plus,
    Minus,
    Semicolon,
    LParen,
    RParen,

    // identifiers and literals
    Identifier(String),
    NumberLiteral(String),
    StringLiteral(String),

    // end
    EOF,
}
