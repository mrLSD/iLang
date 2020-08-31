//! Parser tokens for grammar
//!
//! Parse grammar lexical constructions to AST tokens.
//!
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        alpha1,
        char,
        multispace0,
    },
    combinator::map,
    error::ParseError,
    multi::{
        many0,
        many1,
    },
    sequence::tuple,
    sequence::{
        delimited,
        preceded,
        terminated,
    },
    IResult,
    InputTakeAtPosition,
};

use crate::{
    ast,
    ast::{
        ParseResult,
        Span,
    },
    char::AsChar,
};
use nom::combinator::opt;

/// Apply parser func for delimited space
/// ## RULE:
/// ```js
/// [MULTISPACE] parser-func [MULTISPACE]
/// ```
pub fn delimited_space<'a, O, F>(func: F) -> impl Fn(Span<'a>) -> ParseResult<O>
where
    F: Fn(Span<'a>) -> ParseResult<O>,
{
    delimited(multispace0, func, multispace0)
}

/// Apply parser for brackets case
/// ## RULE:
/// ```js
/// [MULTISPACE] "(" [MULTISPACE] parser-func [MULTISPACE] ")" [MULTISPACE]
/// ```
pub fn get_from_brackets<'a, O, F>(func: F) -> impl Fn(Span<'a>) -> ParseResult<O>
where
    F: Fn(Span<'a>) -> ParseResult<O>,
{
    preceded(
        delimited_space(char('(')),
        terminated(func, delimited_space(char(')'))),
    )
}

/// Parse Ident from brackets
/// ## RULE:
/// ```js
/// [MULTISPACE] "(" [MULTISPACE] ident [MULTISPACE] ")" [MULTISPACE]
/// ```
pub fn get_ident_from_brackets(data: Span) -> ParseResult<ast::Ident> {
    get_from_brackets(ident)(data)
}

/// Alphanum characters with underscores. Based on ASCII.
/// ## RULES:
/// ```js
/// (alpha | number | '_')*
/// ```
pub fn alphanum_and_underscore0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    let f = |c: &char| c.is_alphanumeric() || c.as_char() == '_';
    input.split_at_position_complete(|item| !item.is_a(f))
}

/// Get ident token
///
/// First always should be Alpha char.
/// ## RULES:
/// ```js
/// ident = (alpha+)(alpha | number | '_')*
/// ```
pub fn ident(data: Span) -> ParseResult<ast::Ident> {
    let _ = alpha1(data)?;
    alphanum_and_underscore0(data)
}

/// Parse expression operations
/// ## RULES:
/// ```js
/// expression-operations = (
///     "+" | "-" |
///     "*" | "/" |
///     "<<<" | ">>>"
/// )
/// ```
pub fn expression_operations(data: Span) -> ParseResult<ast::ExpressionOperation> {
    map(
        alt((
            tag("+"),
            tag("-"),
            tag("*"),
            tag("/"),
            tag("<<<"),
            tag(">>>"),
        )),
        |o: Span| match *o.fragment() {
            "+" => ast::ExpressionOperation::Plus,
            "-" => ast::ExpressionOperation::Minus,
            "*" => ast::ExpressionOperation::Multiply,
            "/" => ast::ExpressionOperation::Divide,
            "<<<" => ast::ExpressionOperation::ShiftLeft,
            _ => ast::ExpressionOperation::ShiftRight,
        },
    )(data)
}

/// Parse parameter value
/// ## RULES:
/// ```js
/// parameter-value = ident-value
/// ```
pub fn parameter_value(data: Span) -> ParseResult<ast::ParameterValue> {
    ident_value(data)
}

/// Parse ident value with space and brackets
/// ## RULES:
/// ```js
/// ident-value = (ident | "(" ident ")")
/// ```
pub fn ident_value(data: Span) -> ParseResult<ast::Ident> {
    delimited_space(alt((ident, get_ident_from_brackets)))(data)
}

/// Parse parameter type. It can contain type sequence
/// ## RULES:
/// ```js
/// parameter-type = (ident-value ["*" ident-value] | "(" ident-value ["*" ident-value] ")")+
/// ```
pub fn parameter_type(data: Span) -> ParseResult<ast::ParameterType> {
    let type_list = tuple((
        ident_value,
        many0(preceded(delimited_space(tag("*")), ident_value)),
    ));
    let type_list_bracketes = get_from_brackets(tuple((
        ident_value,
        many0(preceded(delimited_space(tag("*")), ident_value)),
    )));

    map(
        alt((type_list, type_list_bracketes)),
        |(first, mut second)| {
            let mut res_list = vec![first];
            res_list.append(&mut second);
            res_list
        },
    )(data)
}

/// Value-Type parameters parser
/// ## RULES:
/// ```js
/// parameter-value-type = (parameter-value ":" parameter-type | "(" parameter-value ":" parameter-type ")")
/// ```
pub fn parameter_value_type(data: Span) -> ParseResult<ast::ParameterValueType> {
    let value_type = tuple((
        parameter_value,
        preceded(delimited_space(tag(":")), parameter_type),
    ));
    let value_type_bracketes = get_from_brackets(tuple((
        parameter_value,
        preceded(delimited_space(tag(":")), parameter_type),
    )));

    map(alt((value_type, value_type_bracketes)), |o| {
        ast::ParameterValueType::ValueType(o.0, o.1)
    })(data)
}

/// Parameters list with brackets parser
/// ## RULES:
/// ```js
/// parameter-list-brackets = "(" [(
///     parameter-value |
///     parameter-value-type
/// ) [","]]* ")"
/// ```
pub fn parameter_list_brackets(data: Span) -> ParseResult<ast::ParameterValueList> {
    let wrapper_parameter_value = &map(parameter_value, ast::ParameterValueType::Value);
    let (i, (param1, mut param2)) = get_from_brackets(tuple((
        alt((parameter_value_type, wrapper_parameter_value)),
        many0(preceded(
            delimited_space(tag(",")),
            alt((parameter_value_type, wrapper_parameter_value)),
        )),
    )))(data)?;
    let mut res = vec![param1];
    res.append(&mut param2);
    Ok((i, ast::ParameterValueList::ParameterList(res)))
}

/// Parameters value list
/// ## RULES:
/// ```js
/// parameter-value-list = (parameter-value | parameter-list-brackets)
/// ```
pub fn parameter_value_list(data: Span) -> ParseResult<ast::ParameterValueList> {
    alt((
        map(parameter_value, ast::ParameterValueList::ParameterValue),
        parameter_list_brackets,
    ))(data)
}

/// Parameters list
/// ## RULES:
/// ```js
/// parameter-list = (parameter-value-list+ | parameter-list-brackets)
/// ```
pub fn parameter_list(data: Span) -> ParseResult<ast::ParameterList> {
    alt((
        map(
            many1(parameter_value_list),
            ast::ParameterList::ParameterValueList,
        ),
        map(parameter_list_brackets, ast::ParameterList::ParameterList),
    ))(data)
}

/// Value list from parameter values
/// ## RULES:
/// ```js
/// value-list = (parameter-value | "(" (parameter-value [","])* ")")
/// ```
pub fn value_list(data: Span) -> ParseResult<ast::ValueList> {
    let val_list = map(
        get_from_brackets(tuple((
            parameter_value,
            many0(preceded(delimited_space(tag(",")), parameter_value)),
        ))),
        |(first, mut second)| {
            let mut res_list = vec![first];
            res_list.append(&mut second);
            res_list
        },
    );
    alt((map(parameter_value, |v| vec![v]), val_list))(data)
}

/// Let binding Value list from parameter values list
/// ## RULES:
/// ```js
/// let-value-list = (parameter-value-list [","])+
/// ```
pub fn let_value_list(data: Span) -> ParseResult<ast::LetValueList> {
    map(
        tuple((
            parameter_value_list,
            many0(preceded(delimited_space(tag(",")), parameter_value_list)),
        )),
        |(first, mut second)| {
            let mut res_list = vec![first];
            res_list.append(&mut second);
            res_list
        },
    )(data)
}

/// Let binding Value list from parameter values list
/// ## RULES:
/// ```js
/// namespace = "namespace" (namespace-name ".")* namespace-name
/// namespace-name = ident
/// ```
pub fn namespace(data: Span) -> ParseResult<ast::Namespace> {
    map(
        tuple((
            preceded(tag("namespace"), ident),
            many0(preceded(delimited_space(tag(".")), ident)),
        )),
        |(first, mut second)| {
            let mut res_list = vec![first];
            res_list.append(&mut second);
            res_list
        },
    )(data)
}

/// Accessibility modifiers parser
/// ## RULES:
/// ```js
/// accessibility-modifier = ("public" | "internal" | "private")
/// ```
pub fn accessibility_modifier(data: Span) -> ParseResult<ast::AccessibilityModifier> {
    alt((tag("public"), tag("internal"), tag("private")))(data)
}

/// Module parser
/// ## RULES:
/// ```js
/// module = "module" [accessibility-modifier] (qualified-namespace "." )* module-name
/// qualified-namespace = indent
/// module-name = ident
/// ```
pub fn module(data: Span) -> ParseResult<ast::Module> {
    map(
        tuple((
            preceded(
                tag("module"),
                tuple((opt(accessibility_modifier), delimited_space(ident))),
            ),
            //preceded(tag("module"), delimited_space(ident)),
            many0(preceded(delimited_space(tag(".")), delimited_space(ident))),
        )),
        |(first, mut second)| {
            let accessibility = first.0;
            let mut res_list = vec![first.1];
            //let mut res_list = vec![first];
            res_list.append(&mut second);
            ast::Module {
                accessibility: accessibility,
                module_name: res_list,
            }
        },
    )(data)
}
