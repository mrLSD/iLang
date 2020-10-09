//! # Global Variables
//!
//! Global variables define regions of memory allocated at compilation
//! time instead of run-time.
//!
//! Global variable definitions must be initialized.
//!
//! https://llvm.org/docs/LangRef.html#global-variables

use super::{
    addrspace::AddrSpace,
    align::Alignment,
    comdat::ComDat,
    dll_storage_classes::DLLStorageClasses,
    linkage_types::LinkageTypes,
    runtime_preemption::RuntimePreemptionSpecifier,
    section::Section,
    thread_local_storage::ThreadLocalStorage,
    types::Type,
    visibility_styles::VisibilityStyles,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum UnnamedAddr {
    UnnamedAddr,
    LocalUnnamedAddr,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum GlobalVariableKind {
    Global,
    Constant,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct GlobalVariable<T> {
    name: String,
    linkage: Option<LinkageTypes>,
    preemption_specifier: Option<RuntimePreemptionSpecifier>,
    visibility: Option<VisibilityStyles>,
    dll_storage_classes: Option<DLLStorageClasses>,
    thread_local: Option<ThreadLocalStorage>,
    unnamed_addr: Option<UnnamedAddr>,
    addrspace: Option<AddrSpace>,
    global_variable_kind: GlobalVariableKind,
    value_type: Type,
    initializer_constant: Option<T>,
    section: Option<Section>,
    comdat: Option<ComDat>,
    alignment: Option<Alignment>,
    metadata: Option<String>,
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

impl std::fmt::Display for GlobalVariableKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            GlobalVariableKind::Global => "global",
            GlobalVariableKind::Constant => "constant",
        };

        write!(f, "{}", s)
    }
}

impl<T: std::fmt::Display> std::fmt::Display for GlobalVariable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = "";
        write!(f, "{}", s)
    }
}
