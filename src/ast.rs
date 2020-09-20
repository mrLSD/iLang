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

/// Accessibility modifiers
pub type AccessibilityModifier<'a> = Ident<'a>;

/// Qualified namespace definitions
pub type QualifiedNamespace<'a> = Ident<'a>;

/// Module definition
#[derive(Debug, Clone, PartialEq)]
pub struct Module<'a> {
    pub accessibility: Option<AccessibilityModifier<'a>>,
    pub module_name: Vec<ModuleName<'a>>,
}

/// Name of namespace
pub type NamespaceName<'a> = Ident<'a>;

/// Namespace definition
pub type Namespace<'a> = Vec<NamespaceName<'a>>;

/// Function call names
pub type FunctionCallName<'a> = Vec<FunctionName<'a>>;

/// Let binding value list
pub type LetValueList<'a> = Vec<ParameterValueList<'a>>;

/// Let binding
#[derive(Debug, Clone, PartialEq)]
pub struct LetBinding<'a> {
    pub value_list: LetValueList<'a>,
    pub function_body: FunctionBody<'a>,
}

/// Function body
pub type FunctionBody<'a> = Vec<FunctionBodyStatement<'a>>;

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionBodyStatement<'a> {
    LetBinding(LetBinding<'a>),
    FunctionCall(FunctionCall<'a>),
    Expression(Box<Expression<'a>>),
}

/// Expression basic statement
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionFunctionValueCall<'a> {
    FunctionValue(FunctionValue<'a>),
    FunctionCall(FunctionCall<'a>),
}

/// Expression basic statement
#[derive(Debug, Clone, PartialEq)]
pub struct Expression<'a> {
    pub function_statement: ExpressionFunctionValueCall<'a>,
    pub operation_statement: Option<ExpressionOperation>,
    pub expression: Option<Box<Expression<'a>>>,
}

/// Function value statement
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionValue<'a> {
    ValueList(ValueList<'a>),
    Expression(Box<Expression<'a>>),
}

/// Return statement
pub type ReturnStatement<'a> = FunctionValue<'a>;

/// Function call statement
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall<'a> {
    pub function_call_name: FunctionCallName<'a>,
    pub function_value: Vec<FunctionValue<'a>>,
}

/// Function modifiers
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionModifier {
    Inline,
}

/// Function statement
#[derive(Debug, Clone, PartialEq)]
pub struct Function<'a> {
    pub modifier: Option<FunctionModifier>,
    pub function_name: FunctionName<'a>,
    pub parameter_list: ParameterList<'a>,
    pub return_type: Option<ReturnType<'a>>,
    pub function_body: FunctionBody<'a>,
}

/// Main statement
#[derive(Debug, Clone, PartialEq)]
pub enum MainStatement<'a> {
    Namespace(Namespace<'a>),
    Module(Module<'a>),
    Function(Function<'a>),
    LetBinding(LetBinding<'a>),
}

/// Main - entry point for all definitions
pub type Main<'a> = Vec<MainStatement<'a>>;

/// A string fragment contains a fragment of a string being parsed: either
/// a non-empty Literal (a series of non-escaped characters), a single
/// parsed escaped character, or a block of escaped whitespace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringFragment<'a> {
    Literal(&'a str),
    EscapedChar(char),
    EscapedWS,
}

/// String identifier
/// Basic component for string parser
pub struct StringIdent(pub String);
