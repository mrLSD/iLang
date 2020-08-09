//! Parser tokens representation
//!
use nom::{
    character::complete::alpha1,
    error::ParseError,
    IResult,
    InputTakeAtPosition,
};
use nom_locate::LocatedSpan;

use crate::{
    ast,
    char::AsChar,
};
use nom::branch::alt;
use nom::bytes::complete::tag;

/// Span is basic lexical component
pub(crate) type Span<'a> = LocatedSpan<&'a str>;

/// Alphanum characters with underscores. Based on ASCII.
/// RULES: (alpha | number | '_')*
pub fn alphanum_and_underscore0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    let f = |c: &char| c.is_alphanumeric() || c.as_char() == '_';
    input.split_at_position_complete(|item| !item.is_a(f))
}

/// Get ident token
/// First always should be Alpha char.
/// RULES: (alpha+)(alpha | number | '_')*
pub fn ident(data: Span) -> IResult<Span, ast::Ident> {
    let _ = alpha1(data)?;
    let (i, o) = alphanum_and_underscore0(data)?;
    Ok((i, ast::Ident(o)))
}

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

/*pub fn funcdef<'a>(val: &str) -> IResult<Span, String> {
    let data = Span::new(val);
    let def = terminated(tag("def"), space1);
    let t = map(tuple((alpha1, alphanumeric0)), |(a1, a2)| format!("{:?}{:?}", a1, a2));
    let res = preceded(def, t)(data);
    println!("{:?}", res);
    res
}*/

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
