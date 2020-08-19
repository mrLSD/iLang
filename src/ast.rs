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
    Minus,
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
pub struct ParameterType<'a>(pub Vec<Ident<'a>>);

/// Return type for functions
pub type ReturnType<'a> = ParameterType<'a>;

/// Parameter value type - contain Values and its type
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterValueType<'a>(pub ParameterValue<'a>, pub ParameterType<'a>);

/// Parameters value list for brackets case
#[derive(Debug, Clone, PartialEq)]
pub enum ParameterListBrackets<'a> {
    ParameterValue(ParameterValue<'a>),
    ParameterValueType(ParameterValueType<'a>),
}

/// List of parameters values
#[derive(Debug, Clone, PartialEq)]
pub enum ParameterValueList<'a> {
    ParameterValue(ParameterValue<'a>),
    ParameterListBrackets(ParameterListBrackets<'a>),
}
