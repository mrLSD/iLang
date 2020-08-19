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
    error::ParseError,
    multi::many0,
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
use nom::combinator::map;

/// Span is basic lexical component
pub(crate) type Span<'a> = LocatedSpan<&'a str>;

/// Apply parser func for delimited space
/// ## RULE:
/// ```js
/// [MULTISPACE] parser-func [MULTISPACE]
/// ```
pub fn delimited_space<'a, O, F>(func: F) -> impl Fn(Span<'a>) -> IResult<Span, O>
where
    F: Fn(Span<'a>) -> IResult<Span, O>,
{
    delimited(multispace0, func, multispace0)
}

/// Apply parser for brackets case
/// ## RULE:
/// ```js
/// [MULTISPACE] "(" [MULTISPACE] parser-func [MULTISPACE] ")" [MULTISPACE]
/// ```
pub fn get_from_brackets<'a, O, F>(func: F) -> impl Fn(Span<'a>) -> IResult<Span, O>
where
    F: Fn(Span<'a>) -> IResult<Span, O>,
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
pub fn get_ident_from_brackets(data: Span) -> IResult<Span, ast::Ident> {
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
/// (alpha+)(alpha | number | '_')*
/// ```
pub fn ident(data: Span) -> IResult<Span, ast::Ident> {
    let _ = alpha1(data)?;
    let (i, o) = alphanum_and_underscore0(data)?;
    Ok((i, ast::Ident(o)))
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
pub fn expression_operations(data: Span) -> IResult<Span, ast::ExpressionOperation> {
    let (i, o) = alt((
        tag("+"),
        tag("-"),
        tag("*"),
        tag("/"),
        tag("<<<"),
        tag(">>>"),
    ))(data)?;
    Ok((
        i,
        match *o.fragment() {
            "+" => ast::ExpressionOperation::Plus,
            "-" => ast::ExpressionOperation::Minus,
            "*" => ast::ExpressionOperation::Multiply,
            "/" => ast::ExpressionOperation::Divide,
            "<<<" => ast::ExpressionOperation::ShiftLeft,
            ">>>" => ast::ExpressionOperation::ShiftRight,
            _ => unreachable!(),
        },
    ))
}

/// Parse parameter value
/// ## RULES:
/// ```js
/// ident-value
/// ```
pub fn parameter_value(data: Span) -> IResult<Span, ast::ParameterValue> {
    let (i, o) = ident_value(data)?;
    Ok((i, ast::ParameterValue(o)))
}

/// Parse ident value with space and brackets
/// ## RULES:
/// ```js
/// (ident | "(" ident ")")
/// ```
pub fn ident_value(data: Span) -> IResult<Span, ast::Ident> {
    delimited_space(alt((ident, get_ident_from_brackets)))(data)
}

/// Parse parameter type. It can contain type sequence
/// ## RULES:
/// ```js
/// (ident-value ["*" ident-value] | "(" ident-value ["*" ident-value] ")")+
/// ```
pub fn parameter_type(data: Span) -> IResult<Span, ast::ParameterType> {
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

/// ## RULES:
/// ```js
/// (parameter-value ":" parameter-type | "(" parameter-value ":" parameter-type ")")
/// ```
pub fn parameter_value_type(data: Span) -> IResult<Span, ast::ParameterValueType> {
    let value_type = tuple((
        parameter_value,
        preceded(delimited_space(tag(":")), parameter_type),
    ));
    let value_type_bracketes = get_from_brackets(tuple((
        parameter_value,
        preceded(delimited_space(tag(":")), parameter_type),
    )));

    let (i, o) = alt((value_type, value_type_bracketes))(data)?;
    Ok((i, ast::ParameterValueType(o.clone().0, o.clone().1)))
}
