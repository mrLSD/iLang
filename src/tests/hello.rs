#![allow(unused_imports)]
use crate::ast::Span;
use crate::token::main;

fn read_source(file: &str) -> String {
    std::fs::read_to_string(file).expect("file not found")
}

#[test]
#[allow(dead_code)]
pub fn test_exampels_hello_world() {
    let _src = read_source("./examples/hello.i");
    let _x = main;
    //let _res = main(Span::new(src.as_str())).unwrap();
    // assert_eq!(res.0.fragment(), &"");
    // println!("{:#?}", res);
}
