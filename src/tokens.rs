//! Parser tokens representation
//! 
use crate::char::AsChar;
use nom::{
    character::complete::alpha1,
    error::ParseError,
    IResult,
    InputTakeAtPosition,
};
use nom_locate::LocatedSpan;

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

/// Get NAME parameter
/// First always should be Alpha char.
/// RULES: (alpha+)(alpha | number | '_')*
pub fn name(val: &str) -> IResult<Span, Span> {
    let data = Span::new(val);
    let _ = alpha1(data)?;
    alphanum_and_underscore0(data)
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
    use crate::tokens::*;

    #[test]
    fn test_name() {
        assert!(name("test").is_ok());
        assert!(name("123test").is_err());
        assert!(name("test123").is_ok());
        assert!(name("test123test").is_ok());

        let n = name("test123 test");
        assert!(n.is_ok());
        let n = n.unwrap();
        assert_eq!(*n.1.fragment(), "test123");
        assert_eq!(*n.0.fragment(), " test");

        let n = name("test_123a(test)");
        assert!(n.is_ok());
        let n = n.unwrap();
        assert_eq!(*n.1.fragment(), "test_123a");
        assert_eq!(*n.0.fragment(), "(test)");
    }
}
