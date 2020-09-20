use crate::string::parse_string;

#[test]
fn test_parser_string() {
    let res = parse_string(r#""tab:\tafter tab, newline:\nnew line, quote: \", emoji: 😂, newline:\nescaped whitespace: abc""#).unwrap();
    assert_eq!(res.1.0, String::from("tab:\tafter tab, newline:\nnew line, quote: \", emoji: 😂, newline:\nescaped whitespace: abc"));

    let res = parse_string(r#""test1" test2"#).unwrap();
    assert_eq!(res.0, String::from(" test2"));
    assert_eq!(res.1.0, String::from("test1"));
}
