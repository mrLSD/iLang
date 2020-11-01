//! # Linkage Types
//!
//! All Global Variables and Functions have one of the following
//! types of linkage.
//!
//! It is illegal for a global variable or function declaration to
//! have any linkage type other than external or extern_weak.
//!
//! https://llvm.org/docs/LangRef.html#linkage-types

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum LinkageTypes {
    Private,
    Internal,
    AvailableExternally,
    LinkOnce,
    Weak,
    Common,
    Appending,
    ExternWeak,
    LinkonceOdr,
    WeakOdr,
    External,
}

impl std::fmt::Display for LinkageTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            LinkageTypes::Private => "private",
            LinkageTypes::Internal => "internal",
            LinkageTypes::AvailableExternally => "available_externally",
            LinkageTypes::LinkOnce => "linkonce",
            LinkageTypes::Weak => "weak",
            LinkageTypes::Common => "common",
            LinkageTypes::Appending => "appending",
            LinkageTypes::ExternWeak => "extern_weak",
            LinkageTypes::LinkonceOdr => "linkonce_odr",
            LinkageTypes::WeakOdr => "weak_odr",
            LinkageTypes::External => "external",
        };

        write!(f, "{}", s)
    }
}
