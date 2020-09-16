//! # iLang - functional language
//!
//! Currently it's Parser stage.
//!
#![warn(clippy::all)]

pub mod ast;
pub mod token;

mod char;

#[cfg(test)]
mod char_test;
mod tests;
#[cfg(test)]
mod token_test;
