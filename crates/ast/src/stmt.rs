
use super::declarations::VariableDeclaration;
use super::expressions::Expression;

// For now it support variable declaration
// and expression like addition
//This will be enhanced in future, as we continue
// The developement of Shiryuu
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// A variable declaration statement
    VarDecl(VariableDeclaration),

    /// A statement that evaluates an expression
    Expr(Expression),
}
