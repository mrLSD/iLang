//! # Target Triple
//!
//! A module may specify a target triple string that describes the
//! target host.
//!
//! https://llvm.org/docs/LangRef.html#target-triple

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TargetTriple(String);

impl std::fmt::Display for TargetTriple {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "target triple = \"{}\"", self.0)
    }
}
