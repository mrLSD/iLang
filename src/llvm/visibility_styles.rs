//! # Visibility Styles
//!
//! All Global Variables and Functions have one of the following
//! visibility styles
//!
//! A symbol with internal or private linkage must have default visibility.
//!
//! https://llvm.org/docs/LangRef.html#id1246

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum VisibilityStyles {
    Default,
    Hidden,
    Protected,
}

impl std::fmt::Display for VisibilityStyles {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            VisibilityStyles::Default => "default",
            VisibilityStyles::Hidden => "hidden",
            VisibilityStyles::Protected => "protected",
        };

        write!(f, "{}", s)
    }
}
