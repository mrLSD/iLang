#![allow(unused_imports)]
use crate::parser::{
	ast::Span,
	token::main,
};

fn read_source(file: &str) -> String {
	std::fs::read_to_string(file).expect("file not found")
}

#[test]
#[allow(dead_code)]
pub fn test_exampels_hello_fn() {
	let src = read_source("./examples/hello_fn.i");
	let res = main(Span::new(src.as_str())).unwrap();
	assert_eq!(res.0.fragment(), &"");
}
