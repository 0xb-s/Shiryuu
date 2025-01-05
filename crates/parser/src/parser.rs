
use super::token::Token;
use ast::{
    declarations::VariableDeclaration,
    expressions::{BinaryOperator, Expression},
    stmt::Statement,
    types::BasicType,
    AstNode, AstRoot,
};

#[derive(Debug)]
pub struct ParserError(pub String);

pub fn parse_statements(tokens: &[Token]) -> Result<AstRoot, String> {
    let mut pos = 0;
    let mut nodes = Vec::new();

    while pos < tokens.len() {
        if matches!(tokens[pos], Token::EOF) {
            break;
        }

        let statement = parse_statement(tokens, &mut pos)?;
        nodes.push(AstNode::Statement(statement));
    }

    Ok(AstRoot { nodes })
}

/// Parse a single statement.
/// - Variable declaration (e.g., `int x = 5;`)
/// - Or expression statement (e.g., `x = x + 1;`)
fn parse_statement(tokens: &[Token], pos: &mut usize) -> Result<Statement, String> {
    match tokens.get(*pos) {
        Some(Token::KwInt | Token::KwFloat | Token::KwUint | Token::KwString) => {
            parse_var_decl(tokens, pos)
        }
        _ => {
            // Otherwise parse as an expression statement
            let expr = parse_expression(tokens, pos)?;
            consume_semicolon(tokens, pos)?;
            Ok(Statement::Expr(expr))
        }
    }
}

/// Parse a variable declaration.
fn parse_var_decl(tokens: &[Token], pos: &mut usize) -> Result<Statement, String> {
    let var_type = match tokens.get(*pos) {
        Some(Token::KwInt) => BasicType::Int,
        Some(Token::KwFloat) => BasicType::Float,
        Some(Token::KwUint) => BasicType::Uint,
        Some(Token::KwString) => BasicType::String,
        _ => return Err(format!("Expected type keyword at {:?}", tokens.get(*pos))),
    };
    *pos += 1;

    let name = match tokens.get(*pos) {
        Some(Token::Identifier(s)) => s.clone(),
        _ => return Err(format!("Expected identifier at {:?}", tokens.get(*pos))),
    };
    *pos += 1;

    let mut initializer = None;
    if let Some(Token::Equal) = tokens.get(*pos) {
        *pos += 1; 
        let init_expr = parse_expression(tokens, pos)?;
        initializer = Some(init_expr);
    }

    consume_semicolon(tokens, pos)?;

    Ok(Statement::VarDecl(VariableDeclaration {
        var_type,
        name,
        initializer,
    }))
}

/// Parse expressions that might contain + or - operators with left-to-right associativity.
fn parse_expression(tokens: &[Token], pos: &mut usize) -> Result<Expression, String> {
    let mut expr = parse_primary(tokens, pos)?;

    while let Some(op_token) = tokens.get(*pos) {
        match op_token {
            Token::Plus => {
                *pos += 1;
                let right = parse_primary(tokens, pos)?;
                expr = Expression::BinaryOp {
                    op: BinaryOperator::Add,
                    left: Box::new(expr),
                    right: Box::new(right),
                };
            }
            Token::Minus => {
                *pos += 1;
                let right = parse_primary(tokens, pos)?;
                expr = Expression::BinaryOp {
                    op: BinaryOperator::Sub,
                    left: Box::new(expr),
                    right: Box::new(right),
                };
            }
            _ => break,
        }
    }

    Ok(expr)
}

/// parse  expression:

fn parse_primary(tokens: &[Token], pos: &mut usize) -> Result<Expression, String> {
    match tokens.get(*pos) {
        Some(Token::NumberLiteral(num_str)) => {
            
            let is_float = num_str.contains('.');
            let basic_type = if is_float {
                BasicType::Float
            } else {
     
                BasicType::Int
            };

            let expr = Expression::NumberLiteral {
                value: num_str.clone(),
                number_type: basic_type,
            };
            *pos += 1;
            Ok(expr)
        }
        Some(Token::StringLiteral(s)) => {
            let expr = Expression::StringLiteral(s.clone());
            *pos += 1;
            Ok(expr)
        }
        Some(Token::Identifier(name)) => {
            let expr = Expression::VariableRef(name.clone());
            *pos += 1;
            Ok(expr)
        }
        Some(Token::LParen) => {
            // parse a sub-expression
            *pos += 1; // consume '('
            let subexpr = parse_expression(tokens, pos)?;
            // expect closing ')'
            match tokens.get(*pos) {
                Some(Token::RParen) => {
                    *pos += 1;
                    Ok(subexpr)
                }
                _ => Err("Expected ')'".to_string()),
            }
        }
        Some(tok) => Err(format!("Unexpected token in expression: {:?}", tok)),
        None => Err("Unexpected end of tokens in parse_primary".to_string()),
    }
}


fn consume_semicolon(tokens: &[Token], pos: &mut usize) -> Result<(), String> {
    match tokens.get(*pos) {
        Some(Token::Semicolon) => {
            *pos += 1;
            Ok(())
        }
        other => Err(format!("Expected ';', found {:?}", other)),
    }
}
