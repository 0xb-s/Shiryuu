

use super::token::Token;

pub fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.trim().chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            // Whitespace
            w if w.is_whitespace() => {
                chars.next();
            }

            // Single-character tokens
            '=' => {
                chars.next();
                tokens.push(Token::Equal);
            }
            '+' => {
                chars.next();
                tokens.push(Token::Plus);
            }
            '-' => {
                chars.next();
                tokens.push(Token::Minus);
            }
            ';' => {
                chars.next();
                tokens.push(Token::Semicolon);
            }
            '(' => {
                chars.next();
                tokens.push(Token::LParen);
            }
            ')' => {
                chars.next();
                tokens.push(Token::RParen);
            }

            // String literal (naive approach, looks for quotes)
            '"' => {
                tokens.push(parse_string_literal(&mut chars)?);
            }

            // Check for letters (keywords, identifiers)
            alpha if alpha.is_alphabetic() => {
                tokens.push(parse_identifier_or_keyword(&mut chars));
            }

            // Check for digits (number literal)
            digit if digit.is_ascii_digit() => {
                tokens.push(parse_number_literal(&mut chars));
            }

            other => {
                return Err(format!("Unrecognized character: '{}'", other));
            }
        }
    }

    tokens.push(Token::EOF);
    Ok(tokens)
}

/// Parse a double-quoted string literal.
fn parse_string_literal<I>(chars: &mut std::iter::Peekable<I>) -> Result<Token, String>
where
    I: Iterator<Item = char>,
{
    chars.next();

    let mut content = String::new();

    while let Some(&ch) = chars.peek() {
        if ch == '"' {
            // closing quote
            chars.next();
            return Ok(Token::StringLiteral(content));
        } else {
            content.push(ch);
            chars.next();
        }
    }

    Err("Unclosed string literal".to_string())
}

/// Parse a keyword or identifier.
fn parse_identifier_or_keyword<I>(chars: &mut std::iter::Peekable<I>) -> Token
where
    I: Iterator<Item = char>,
{
    let mut ident = String::new();

    while let Some(&ch) = chars.peek() {
        if ch.is_alphanumeric() || ch == '_' {
            ident.push(ch);
            chars.next();
        } else {
            break;
        }
    }

    match ident.as_str() {
        "int" => Token::KwInt,
        "float" => Token::KwFloat,
        "uint" => Token::KwUint,
        "string" => Token::KwString,
        _ => Token::Identifier(ident),
    }
}

/// Parse a numeric literal (integer or float).
fn parse_number_literal<I>(chars: &mut std::iter::Peekable<I>) -> Token
where
    I: Iterator<Item = char>,
{
    let mut num = String::new();
    let mut has_dot = false;

    while let Some(&ch) = chars.peek() {
        if ch.is_ascii_digit() {
            num.push(ch);
            chars.next();
        } else if ch == '.' && !has_dot {
            has_dot = true;
            num.push(ch);
            chars.next();
        } else {
            break;
        }
    }

    Token::NumberLiteral(num)
}
