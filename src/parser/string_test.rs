use super::{
    ast::{
        BasicTypeExpression,
        Span,
    },
    string::parse_string,
    token::delimited_space,
};
use nom::multi::many0;

#[test]
fn test_parser_string() {
    let res = parse_string(Span::new("\""));
    assert!(res.is_err());

    let res = parse_string(Span::new(r#""tab:\tafter tab, newline:\nnew line, quote: \", emoji: 😂, newline:\nescaped whitespace: abc \u{00AC}""#)).unwrap();
    let x = if let BasicTypeExpression::String(v) = res.1 {
        v
    } else {
        unimplemented!()
    };
    assert_eq!(x, String::from("tab:\tafter tab, newline:\nnew line, quote: \", emoji: 😂, newline:\nescaped whitespace: abc \u{00AC}"));

    let res = parse_string(Span::new(r#""test1" test2"#)).unwrap();
    let x = if let BasicTypeExpression::String(v) = res.1 {
        v
    } else {
        unimplemented!()
    };
    assert_eq!(res.0.fragment(), &" test2");
    assert_eq!(x, String::from("test1"));

    let res = parse_string(Span::new(r#""""#)).unwrap();
    let x = if let BasicTypeExpression::String(v) = res.1 {
        v
    } else {
        unimplemented!()
    };
    assert_eq!(res.0.fragment(), &"");
    assert_eq!(x, String::from(""));

    let res = many0(delimited_space(parse_string))(Span::new(r#" "test1" "test2" "#)).unwrap();
    assert_eq!(res.0.fragment(), &"");
    assert_eq!(res.1.len(), 2);
    if let BasicTypeExpression::String(v) = &res.1[0] {
        assert_eq!(v, &String::from("test1"));
    } else {
        unimplemented!()
    }
    if let BasicTypeExpression::String(v) = &res.1[1] {
        assert_eq!(v, &String::from("test2"));
    } else {
        unimplemented!()
    }
}
