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
use nom_locate::LocatedSpan;

use crate::{
    ast,
    char::AsChar,
};

/// Span is basic lexical component
pub(crate) type Span<'a> = LocatedSpan<&'a str>;

pub(crate) type ParseResult<'a, T> = IResult<Span<'a>, T>;

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
    map(alphanum_and_underscore0, ast::Ident)(data)
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
            ">>>" => ast::ExpressionOperation::ShiftRight,
            _ => unreachable!(),
        },
    )(data)
}

/// Parse parameter value
/// ## RULES:
/// ```js
/// parameter-value = ident-value
/// ```
pub fn parameter_value(data: Span) -> ParseResult<ast::ParameterValue> {
    map(ident_value, ast::ParameterValue)(data)
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
            ast::ParameterType(res_list)
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
        alt((wrapper_parameter_value, parameter_value_type)),
        many0(preceded(
            delimited_space(tag(",")),
            alt((wrapper_parameter_value, parameter_value_type)),
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
