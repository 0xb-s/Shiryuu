/*!
`declarations.rs` - Defines structures for variable declarations.
*/

use super::expressions::Expression;
use super::types::BasicType;

/// Represents a single variable declaration.
///
/// e.g. `int x = 5;`
#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {

    pub var_type: BasicType,
    /// identifier  of the variable.
    pub name: String,
    /// optional initializer expression.
    
    pub initializer: Option<Expression>,
}
