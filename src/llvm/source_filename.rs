//! # Source Filename
//!
//! The source filename string is set to the original module identifier,
//! which will be the name of the compiled source file when compiling
//! from source through the clang front end, for example. It is then
//! preserved through the IR and bitcode.
//!
//! This is currently necessary to generate a consistent unique global
//! identifier for local functions used in profile data, which prepends
//! the source file name to the local function name.
//!
//! https://llvm.org/docs/LangRef.html#source-filename

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SourceFileName(pub String);

impl std::fmt::Display for SourceFileName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "source_filename = \"{}\"", self.0)
    }
}
