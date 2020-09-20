use crate::ast::{
    BasicTypeExpression,
    Span,
};
use crate::string::parse_string;

#[test]
fn test_parser_string() {
    let res = parse_string(Span::new(r#""tab:\tafter tab, newline:\nnew line, quote: \", emoji: 😂, newline:\nescaped whitespace: abc""#)).unwrap();
    let x = if let BasicTypeExpression::String(v) = res.1 {
        v
    } else {
        unimplemented!()
    };
    assert_eq!(x, String::from("tab:\tafter tab, newline:\nnew line, quote: \", emoji: 😂, newline:\nescaped whitespace: abc"));

    let res = parse_string(Span::new(r#""test1" test2"#)).unwrap();
    let x = if let BasicTypeExpression::String(v) = res.1 {
        v
    } else {
        unimplemented!()
    };
    assert_eq!(res.0.fragment(), &" test2");
    assert_eq!(x, String::from("test1"));
}
