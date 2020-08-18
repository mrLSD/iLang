//! # iLang - functional language
//!
//! Currently it's Parser stage.
//!
#![warn(clippy::all)]

pub mod ast;
pub mod tokens;

mod char;
#[cfg(test)]
mod token_test;
