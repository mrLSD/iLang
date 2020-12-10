//! # Target Triple
//!
//! A module may specify a target triple string that describes the
//! target host.
//!
//! https://llvm.org/docs/LangRef.html#target-triple

pub const TARGET_X86_64_UNKNOWN_LINUX_GNU: &str = "x86_64-unknown-linux-gnu";

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TargetTriple(pub String);

impl std::fmt::Display for TargetTriple {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "target triple = \"{}\"", self.0)
    }
}
