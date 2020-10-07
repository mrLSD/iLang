//! # Global Variables
//!
//! Global variables define regions of memory allocated at compilation
//! time instead of run-time.
//!
//! Global variable definitions must be initialized.
//!
//! https://llvm.org/docs/LangRef.html#global-variables

use super::{
    dll_storage_classes::DLLStorageClasses,
    linkage_types::LinkageTypes,
    runtime_preemption::RuntimePreemptionSpecifier,
    thread_local_storage::ThreadLocalStorage,
    visibility_styles::VisibilityStyles,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum UnnamedAddr {
    UnnamedAddr,
    LocalUnnamedAddr,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct GlobalVariable {
    name: String,
    linkage: Option<LinkageTypes>,
    preemption_specifier: Option<RuntimePreemptionSpecifier>,
    visibility: Option<VisibilityStyles>,
    dll_storage_classes: Option<DLLStorageClasses>,
    thread_local: Option<ThreadLocalStorage>,
    unnamed_addr: Option<UnnamedAddr>,
}

impl std::fmt::Display for UnnamedAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            UnnamedAddr::UnnamedAddr => "unnamed_addr",
            UnnamedAddr::LocalUnnamedAddr => "local_unnamed_addr",
        };

        write!(f, "{}", s)
    }
}

impl std::fmt::Display for GlobalVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = "";
        write!(f, "{}", s)
    }
}
