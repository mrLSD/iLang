//! # iLang - functional language
//!
//! Currently it's Parser stage.
//!
#![warn(clippy::all)]

pub mod compiler;
#[macro_use]
pub mod llvm;
pub mod codegen;
pub mod parser;
mod tests;
