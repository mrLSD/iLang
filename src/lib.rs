//! # iLang - functional language
//!
//! Currently it's Parser stage.
//!
#![warn(clippy::all)]

pub mod ast;
pub mod llvm;
pub mod string;
pub mod token;

mod char;

#[cfg(test)]
mod char_test;
#[cfg(test)]
mod string_test;
mod tests;
#[cfg(test)]
mod token_test;
