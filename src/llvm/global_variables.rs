//! # Global Variables
//!
//! Global variables define regions of memory allocated at compilation
//! time instead of run-time.
//!
//! Global variable definitions must be initialized.
//!
//! Syntax:
//! ```html
//! @<GlobalVarName> = [Linkage] [PreemptionSpecifier] [Visibility]
//!                    [DLLStorageClass] [ThreadLocal]
//!                    [(unnamed_addr|local_unnamed_addr)] [AddrSpace]
//!                    [ExternallyInitialized]
//!                    <global | constant> <Type> [<InitializerConstant>]
//!                    [, section "name"] [, comdat [($name)]]
//!                    [, align <Alignment>] (, !name !N)*
//! ```
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
pub struct GlobalVariable {
    pub name: String,
    pub linkage: Option<LinkageTypes>,
    pub preemption_specifier: Option<RuntimePreemptionSpecifier>,
    pub visibility: Option<VisibilityStyles>,
    pub dll_storage_classes: Option<DLLStorageClasses>,
    pub thread_local: Option<ThreadLocalStorage>,
    pub unnamed_addr: Option<UnnamedAddr>,
    pub addrspace: Option<AddrSpace>,
    pub global_variable_kind: GlobalVariableKind,
    pub value_type: Type,
    pub initializer_constant: Option<String>,
    pub section: Option<Section>,
    pub comdat: Option<ComDat>,
    pub alignment: Option<Alignment>,
    pub metadata: Option<String>,
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

impl std::fmt::Display for GlobalVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = format!("@{} =", self.name);
        if self.linkage.is_some() {
            s = format!("{} {}", s, self.linkage.as_ref().unwrap());
        }
        if self.preemption_specifier.is_some() {
            s = format!("{} {}", s, self.preemption_specifier.as_ref().unwrap());
        }
        if self.visibility.is_some() {
            s = format!("{} {}", s, self.visibility.as_ref().unwrap());
        }
        if self.dll_storage_classes.is_some() {
            s = format!("{} {}", s, self.dll_storage_classes.as_ref().unwrap());
        }
        if self.thread_local.is_some() {
            s = format!("{} {}", s, self.thread_local.as_ref().unwrap());
        }
        if self.unnamed_addr.is_some() {
            s = format!("{} {}", s, self.unnamed_addr.as_ref().unwrap());
        }
        if self.addrspace.is_some() {
            s = format!("{} {}", s, self.addrspace.as_ref().unwrap());
        }
        s = format!("{} {} {}", s, self.global_variable_kind, self.value_type);
        if self.initializer_constant.is_some() {
            s = format!("{} {}", s, self.initializer_constant.as_ref().unwrap());
        }
        if self.section.is_some() {
            s = format!("{}, {}", s, self.section.as_ref().unwrap());
        }
        if self.comdat.is_some() {
            s = format!("{}, {}", s, self.comdat.as_ref().unwrap());
        }
        if self.alignment.is_some() {
            s = format!("{}, {}", s, self.alignment.as_ref().unwrap());
        }
        if self.metadata.is_some() {
            s = format!("{}, {}", s, self.metadata.as_ref().unwrap());
        }

        write!(f, "{}", s)
    }
}
