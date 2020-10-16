//! # Prefix Data
//!
//! Prefix data is data associated with a function which the code
//! generator will emit immediately before the function’s entrypoint.
//! The purpose of this feature is to allow frontends to associate
//! language-specific runtime metadata with specific functions and make
//! it available through the function pointer while still allowing the
//! function pointer to be called.
//!
//! To access the data for a given function, a program may bitcast the
//! function pointer to a pointer to the constant’s type and dereference
//! index -1. This implies that the IR symbol points just past the end of
//! the prefix data.
//!
//! https://llvm.org/docs/LangRef.html#prefix-data

use super::types::Type;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Prefix<T> {
    prefix_type: Type,
    value: T,
}

impl<T: std::fmt::Display> std::fmt::Display for Prefix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = format!("prefix {} {}", self.prefix_type, self.value);
        write!(f, "{}", s)
    }
}
