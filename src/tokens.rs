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

/// Parse Ident from brackets
/// ## RULE:
/// ```
/// [MULTISPACE] "(" [MULTISPACE] ident [MULTISPACE] ")" [MULTISPACE]
/// ```
pub fn get_ident_from_brackets(data: Span) -> IResult<Span, ast::Ident> {
    preceded(
        delimited(multispace0, char('('), multispace0),
        terminated(ident, delimited(multispace0, char(')'), multispace0)),
    )(data)
}

/// Alphanum characters with underscores. Based on ASCII.
/// ## RULES:
/// ```
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
/// ```
/// (alpha+)(alpha | number | '_')*
/// ```
pub fn ident(data: Span) -> IResult<Span, ast::Ident> {
    let _ = alpha1(data)?;
    let (i, o) = alphanum_and_underscore0(data)?;
    Ok((i, ast::Ident(o)))
}

/// Parse expression operations
/// ## RULES:
/// ```
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
            "-" => ast::ExpressionOperation::Plus,
            "*" => ast::ExpressionOperation::Plus,
            "/" => ast::ExpressionOperation::Plus,
            "<<<" => ast::ExpressionOperation::Plus,
            ">>>" => ast::ExpressionOperation::Plus,
            _ => unreachable!(),
        },
    ))
}

/// Parse parameter value
/// ## RULES:
/// ```
/// (ident | "(" ident ")")
/// ```
pub fn parameter_value(data: Span) -> IResult<Span, ast::ParameterValue> {
    let (i, o) = alt((ident, get_ident_from_brackets))(data)?;
    Ok((i, ast::ParameterValue(o)))
}

#[cfg(test)]
mod test {
    use crate::ast::Ident;
    use crate::tokens::*;

    #[test]
    fn test_name() {
        assert!(ident(Span::new("test")).is_ok());
        assert!(ident(Span::new("123test")).is_err());
        assert!(ident(Span::new("test123")).is_ok());
        assert!(ident(Span::new("test123test")).is_ok());

        let n = ident(Span::new("test123 test"));
        assert!(n.is_ok());
        let n = n.unwrap();
        assert_eq!(*n.1, Span::new("test123"));
        assert_eq!(*n.0.fragment(), " test");

        let n = ident(Span::new("test_123a(test)"));
        assert!(n.is_ok());
        let n = n.unwrap();
        assert_eq!(n.1.clone(), Ident(Span::new("test_123a")));
        assert_eq!(*n.0.fragment(), "(test)");
    }
}
