//! Full AST representation
//!
//! Based on *EBNF* grammar

use crate::tokens::Span;

/// Ident (identifier) token
/// It's basic component for many tokens and rules
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

/// Parameter value used for broad cases
/// used for represent values
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterValue<'a>(pub Ident<'a>);

/// Parameter type used for broad cases type representation
/// especial for ParameterValue
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterType<'a>(pub Ident<'a>);
