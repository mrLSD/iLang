//! # iLang - functional language
//!
//! Currently it's Parser stage.
//!
#![warn(clippy::all)]
use crate::codegen::Codegen;
use clap::{
    App,
    Arg,
};

pub mod compiler;
#[macro_use]
pub mod llvm;
pub mod codegen;
pub mod parser;
mod tests;

fn read_source(file: &str) -> String {
    std::fs::read_to_string(file).unwrap_or_else(|_| panic!("input file {} not found", file))
}

pub fn main() {
    let matches = App::new("iLang")
        .version("v0.1")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();
    let source_file = matches.value_of("INPUT").unwrap();
    println!("# Using input file: {}", source_file);
    let src = read_source(source_file);
    println!("# Source code: {}", src);
    let llvm_code = Codegen::build(&src).unwrap_or_else(|err| panic!("Error: {:?}", err));
    compiler::builder("app".into(), llvm_code)
        .unwrap_or_else(|err| panic!("Failed build: {}", err));
}
