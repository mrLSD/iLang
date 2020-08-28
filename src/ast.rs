//! Full AST representation
//!
//! Based on *EBNF* grammar
use nom::IResult;
use nom_locate::LocatedSpan;

/// Span is basic lexical component
pub(crate) type Span<'a> = LocatedSpan<&'a str>;

pub(crate) type ParseResult<'a, T> = IResult<Span<'a>, T>;

/// Ident (identifier) token
/// It's basic component for many tokens and rules
pub type Ident<'a> = Span<'a>;

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
pub type ParameterValue<'a> = Ident<'a>;

/// Parameter type used for broad cases type representation
/// especial for ParameterValue
pub type ParameterType<'a> = Vec<Ident<'a>>;

/// Return type for functions
pub type ReturnType<'a> = ParameterType<'a>;

/// Parameter value type - contain Values and its type
#[derive(Debug, Clone, PartialEq)]
pub enum ParameterValueType<'a> {
    Value(ParameterValue<'a>),
    ValueType(ParameterValue<'a>, ParameterType<'a>),
}

/// List of parameters values
#[derive(Debug, Clone, PartialEq)]
pub enum ParameterValueList<'a> {
    ParameterValue(ParameterValue<'a>),
    ParameterList(Vec<ParameterValueType<'a>>),
}

/// List of parameters
#[derive(Debug, Clone, PartialEq)]
pub enum ParameterList<'a> {
    ParameterList(ParameterValueList<'a>),
    ParameterValueList(Vec<ParameterValueList<'a>>),
}

/// List of Values
pub type ValueList<'a> = Vec<ParameterValue<'a>>;

/// Name of functions
pub type FunctionName<'a> = Ident<'a>;

/// Name of modules
pub type ModuleName<'a> = Ident<'a>;

/// Qualified namespace definitions
pub type QualifiedNamespace<'a> = Ident<'a>;

/// Name of namespace
pub type NamespaceName<'a> = Ident<'a>;

/// Function call names
pub type FunctionCallName<'a> = Vec<FunctionName<'a>>;
