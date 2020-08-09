//! Full AST representation
//!

use crate::tokens::Span;

/// Ident (identifier) token
/// It's baise component for many tokens and rules
#[derive(Debug, Clone, PartialEq)]
pub struct Ident<'a>(pub Span<'a>);

/// Expression Operations
/// Describe type of operations for expressions
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionOperation {
    Plus,
    Minis,
    Multiply,
    Divide,
    ShiftLeft,
    ShiftRight,
}
