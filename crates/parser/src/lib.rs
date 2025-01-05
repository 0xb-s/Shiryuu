

pub mod lexer;
pub mod parser;
pub mod token;

use ast::AstRoot;
use parser::parse_statements;

/// reads the entire source code 
/// tokenizes it, and produces an `AstRoot`.
pub fn parse_source(source: &str) -> Result<AstRoot, String> {
    let tokens = lexer::lex(source)?;
    let ast_root = parse_statements(&tokens)?;
    Ok(ast_root)
}
