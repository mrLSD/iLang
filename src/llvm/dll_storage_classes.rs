//! # DLL Storage Classes
//!
//! All Global Variables, Functions and Aliases can have one of the
//! following DLL storage class
//!
//! https://llvm.org/docs/LangRef.html#dll-storage-classes

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum DLLStorageClasses {
    DllImport,
    DllExport,
}

impl std::fmt::Display for DLLStorageClasses {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            DLLStorageClasses::DllImport => "dllimport",
            DLLStorageClasses::DllExport => "dllexport",
        };

        write!(f, "{}", s)
    }
}
