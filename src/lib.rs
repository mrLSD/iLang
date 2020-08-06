use nom_locate::LocatedSpan;
use nom::{
    combinator::{map, verify, cut},
    bytes::complete::{tag, take_while, take_while1},
    sequence::{tuple, terminated, preceded, delimited},
    branch::alt,
    character::complete::{anychar, char, space1},
    IResult,
};
use nom::bytes::complete::take_until;
use nom::character::complete::{alpha1, alphanumeric0};


pub(crate) type Span<'a> = LocatedSpan<&'a str>;

pub fn name<'a>(val: &str) -> IResult<Span, Span> {
    let data = Span::new(val);
    let _ = alpha1(data)?;
    alphanumeric0(data)
}

pub fn funcdef<'a>(val: &str) -> IResult<Span, String> {
    let data = Span::new(val); 
    let def = terminated(tag("def"), space1);
    let t = map(tuple((alpha1, alphanumeric0)), |(a1, a2)| format!("{:?}{:?}", a1, a2));
    let res = preceded(def, t)(data);
    println!("{:#?}", res);
    res
}


#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_name() {
        assert!(name("test").is_ok());
        assert!(name("123test").is_err());
        assert!(name("test123").is_ok());
        assert!(name("test123test").is_ok());
        let n = name("test123 test");
        assert!(т.is_ok());
    } 
}