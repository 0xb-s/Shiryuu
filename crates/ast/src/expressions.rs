
use super::types::BasicType;


#[derive(Debug, Clone, PartialEq)]
pub enum Expression {

    VariableRef(String),

    /// this for now could be only (integer or float).
    NumberLiteral {
        value: String,
        number_type: BasicType,
    },

    /// A string 
    StringLiteral(String),

    ///binary operation like `+` or `-` with left and right subexpressions.
    BinaryOp {
        op: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

/// A binary operator supported by the language: + or -
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Sub,
}
