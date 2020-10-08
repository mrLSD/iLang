//! # Sections definition
//!
//! Specific section to store data.
//!
//! More details: https://llvm.org/docs/LangRef.html#global-variables

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Section(String);

impl std::fmt::Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "section \"#{}\"", self.0)
    }
}
