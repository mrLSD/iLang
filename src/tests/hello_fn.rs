#![allow(unused_imports)]
use super::read_source;
use crate::parser::{
    ast::Span,
    token::main,
};

#[test]
#[allow(dead_code)]
pub fn test_exampels_hello_fn() {
    let src = read_source("./examples/hello_fn.i");
    let _res = main(Span::new(src.as_str())).unwrap();
    //assert_eq!(res.0.fragment(), &"");
}
