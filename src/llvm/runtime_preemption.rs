//! # Runtime Preemption Specifiers
//!
//! Global variables, functions and aliases may have an optional
//! runtime preemption specifier. If a preemption specifier isn’t
//! given explicitly, then a symbol is assumed to be dso_preemptable.
//!
//! https://llvm.org/docs/LangRef.html#runtime-preemption-specifiers

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum RuntimePreemptionSpecifier {
    DsoPreemptable,
    DsoLocal,
}

impl std::fmt::Display for RuntimePreemptionSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            RuntimePreemptionSpecifier::DsoPreemptable => "dso_preemptable",
            RuntimePreemptionSpecifier::DsoLocal => "dso_local",
        };

        write!(f, "{}", s)
    }
}
