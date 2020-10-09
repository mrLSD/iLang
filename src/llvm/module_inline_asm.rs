//! # Module-Level Inline Assembly
//!
//! Modules may contain “module-level inline asm” blocks, which
//! corresponds to the GCC “file scope inline asm” blocks. These blocks
//! are internally concatenated by LLVM and treated as a single unit,
//! but may be separated in the .ll file if desired.
//!
//! https://llvm.org/docs/LangRef.html#module-level-inline-assembly

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ModuleInlineAsm(String);

impl std::fmt::Display for ModuleInlineAsm {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "module asm \"{}\"", self.0)
    }
}
