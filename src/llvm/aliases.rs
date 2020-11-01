//! # Aliases
//!
//! Aliases, unlike function or variables, don’t
//! create any new data. They are just a new symbol
//! and metadata for an existing position.
//!
//! Aliases have a name and an aliasee that is
//! either a global value or a constant expression.
//!
//! Syntax:
//! ```html
//! @<Name> =
//!     [Linkage]
//!     [PreemptionSpecifier]
//!     [Visibility]
//!     [DLLStorageClass]
//!     [ThreadLocal]
//!     [(unnamed_addr|local_unnamed_addr)]
//!     alias <AliaseeTy>, <AliaseeTy>* @<Aliasee>
//! ```
//! https://llvm.org/docs/LangRef.html#aliases

use super::{
    dll_storage_classes::DLLStorageClasses,
    global_variables::UnnamedAddr,
    linkage_types::LinkageTypes,
    runtime_preemption::RuntimePreemptionSpecifier,
    thread_local_storage::ThreadLocalStorage,
    types::Type,
    visibility_styles::VisibilityStyles,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Alias<T> {
    name: String,
    linkage: Option<LinkageTypes>,
    preemption_specifier: Option<RuntimePreemptionSpecifier>,
    visibility: Option<VisibilityStyles>,
    dll_storage_classes: Option<DLLStorageClasses>,
    thread_local: Option<ThreadLocalStorage>,
    unnamed_addr: Option<UnnamedAddr>,
    aliasee_type: Vec<Type>,
    aleasee: T,
}

impl<T: std::fmt::Display> std::fmt::Display for Alias<T> {
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
        s = format!("{} alias", s);

        let s = self
            .aliasee_type
            .iter()
            .enumerate()
            .fold(s, |x, (ctr, ty)| {
                // Calculation for comma for 1-th element
                if ctr == 0 {
                    format!("{} {},", x, ty)
                } else {
                    format!("{} {}", x, ty)
                }
            });
        let s = format!("{} @{}", s, self.aleasee);

        write!(f, "{}", s)
    }
}
