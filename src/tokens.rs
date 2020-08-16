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
    multi::many1,
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

/// Parse Ident from brackets
/// ## RULE:
/// ```js
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
/// (ident | "(" ident ")")
/// ```
pub fn parameter_value(data: Span) -> IResult<Span, ast::ParameterValue> {
    let (i, o) = delimited(
        multispace0,
        alt((ident, get_ident_from_brackets)),
        multispace0,
    )(data)?;
    Ok((i, ast::ParameterValue(o)))
}

/// Parse parameter type. It can contain type sequence
/// ## RULES:
/// ```js
/// (parameter_value ["*"] | "(" parameter_value ["*"] ")")+
/// ```
pub fn parameter_type(data: Span) -> IResult<Span, ast::ParameterType> {
    let res_first_type = delimited(multispace0, parameter_value, multispace0)(data)?;

    let res = tuple((
        many1(terminated(
            parameter_value,
            delimited(multispace0, tag("*"), multispace0),
        )),
        parameter_value,
    ))(data);
    if res.is_err() {
        return Ok((res_first_type.0, ast::ParameterType(vec![res_first_type.1])));
    }
    let (i, (mut param1, param2)) = res.unwrap();
    param1.append(&mut vec![param2]);
    Ok((i, ast::ParameterType(param1)))
}

/// ## RULES:
/// ```js
/// (parameter-value ":" parameter-type | "(" parameter-value ":" parameter-type ")")
/// ```
pub fn parameter_value_type(data: Span) -> IResult<Span, ast::ParameterValue> {
    let (i, o) = alt((ident, get_ident_from_brackets))(data)?;
    Ok((i, ast::ParameterValue(o)))
}

#[cfg(test)]
mod test {
    use crate::ast::*;
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
        assert_eq!(n.1, Ident(Span::new("test123")));
        assert_eq!(n.0.fragment(), &" test");

        let n = ident(Span::new("test_123a(test)"));
        assert!(n.is_ok());
        let n = n.unwrap();
        assert_eq!(n.1.clone(), Ident(Span::new("test_123a")));
        assert_eq!(*n.0.fragment(), "(test)");
    }

    #[test]
    fn test_expression_operations() {
        assert_eq!(
            expression_operations(Span::new("+x")).unwrap().1,
            ExpressionOperation::Plus
        );
        assert_eq!(
            expression_operations(Span::new("-x")).unwrap().1,
            ExpressionOperation::Minus
        );

        assert_eq!(
            expression_operations(Span::new("*x")).unwrap().1,
            ExpressionOperation::Multiply
        );
        assert_eq!(
            expression_operations(Span::new("/x")).unwrap().1,
            ExpressionOperation::Divide
        );

        assert_eq!(
            expression_operations(Span::new("<<<x")).unwrap().1,
            ExpressionOperation::ShiftLeft
        );
        assert_eq!(
            expression_operations(Span::new(">>>x")).unwrap().1,
            ExpressionOperation::ShiftRight
        );
    }

    #[test]
    fn test_parameter_value() {
        assert_eq!(
            parameter_value(Span::new("test")).unwrap().1,
            ParameterValue(Ident(Span::new("test")))
        );

        let n = parameter_value(Span::new("asd123 test")).unwrap();
        let fragment = ((n.1).0).0.fragment();
        assert_eq!(fragment, &"asd123");

        let n = parameter_value(Span::new("(asd123) test")).unwrap();
        let fragment = ((n.1).0).0.fragment();
        assert_eq!(fragment, &"asd123");

        let n = parameter_value(Span::new(" ( asd123 ) test")).unwrap();
        let fragment = ((n.1).0).0.fragment();
        assert_eq!(fragment, &"asd123");

        assert!(parameter_value(Span::new("123test")).is_err());
    }

    #[test]
    fn test_get_ident_from_brackets() {
        let res = get_ident_from_brackets(Span::new("test123 test"));
        assert!(res.is_err());

        let n = get_ident_from_brackets(Span::new("(test123) test"));
        assert!(n.is_ok());
        let n = n.unwrap();
        assert_eq!((n.1).0.fragment(), &"test123");
        // Spaces removed
        assert_eq!(n.0.fragment(), &"test");

        let n = get_ident_from_brackets(Span::new(" ( test123 ) test"));
        assert!(n.is_ok());
        let n = n.unwrap();
        assert_eq!((n.1).0.fragment(), &"test123");
        assert_eq!(n.0.fragment(), &"test");
    }

    #[test]
    fn test_parameter_type() {
        //let n = parameter_type(Span::new("(asd123) test")).unwrap();
        //let fragment = n.1.fragment();
        //assert_eq!(fragment, &"asd123");

        //let n = parameter_type(Span::new(" ( asd123 ) test")).unwrap();
        //let fragment = n.1.fragment();
        //assert_eq!(fragment, &"asd123");

        //let n = parameter_type(Span::new(" ( asd123 ) * dsa123 * ")).unwrap();
        //let n = parameter_type(Span::new(" asd123 * dsa123 * ")).unwrap();
        //let n = parameter_type(Span::new("asd123")).unwrap();
        //let n = parameter_type(Span::new(" asd123 ")).unwrap();
        let n = parameter_type(Span::new("asd1 *  asd2 * asd3 * asd4")).unwrap();
        //let fragment = n.1.fragment();
        //assert_eq!(fragment, &"asd123");
    }
}
