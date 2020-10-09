//! # Data Layout
//!
//! A module may specify a target specific data layout string that
//! specifies how data is to be laid out in memory.
//!
//! https://llvm.org/docs/LangRef.html#data-layout

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DataLayout(String);

impl std::fmt::Display for DataLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "target datalayout = \"{}\"", self.0)
    }
}
