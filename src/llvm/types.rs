//! # Basic LLVM types

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Type;

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // TODO: implement type definitions
        let s = "";
        write!(f, "{}", s)
    }
}
