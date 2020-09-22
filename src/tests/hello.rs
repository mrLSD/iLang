use crate::ast::Span;
use crate::token::main;

fn read_source(file: &str) -> String {
    std::fs::read_to_string(file).expect("file not found")
}

#[test]
fn test_exampels_hello_world() {
    let src = read_source("./examples/hello.i");
    let res = main(Span::new(src.as_str())).unwrap();
    assert_eq!(res.0.fragment(), &"");
    println!("{:#?}", res);
}
