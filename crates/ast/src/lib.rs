
pub mod declarations;
pub mod expressions;
pub mod stmt;
pub mod types;

use stmt::Statement;

/// `AstNode` can represent all top-level constructs in our language.
// nOTE: this will change in future, as we enhance more feature 
// for Shiruu
#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {

    Statement(Statement),
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstRoot {
    pub nodes: Vec<AstNode>,
}
