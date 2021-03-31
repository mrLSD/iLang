//! # DLL Storage Classes
//!
//! All Global Variables, Functions and Aliases can have one of the
//! following DLL storage class
//!
//! https://llvm.org/docs/LangRef.html#dll-storage-classes

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum DllStorageClasses {
    DllImport,
    DllExport,
}

impl std::fmt::Display for DllStorageClasses {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            DllStorageClasses::DllImport => "dllimport",
            DllStorageClasses::DllExport => "dllexport",
        };

        write!(f, "{}", s)
    }
}
