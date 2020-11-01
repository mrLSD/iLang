//! # IFuncs
//!
//! IFuncs, like as aliases, don’t create any new data or func. They are
//! just a new symbol that dynamic linker resolves at runtime by calling
//! a resolver function.
//!
//! IFuncs have a name and a resolver that is a function called by
//! dynamic linker that returns address of another function associated
//! with the name.
//!
//! Syntax:
//! ```html
//! @<Name> =
//!    [Linkage]
//!    [Visibility]
//!    ifunc <IFuncTy>, <ResolverTy>* @<Resolver>
//! ```
//! https://llvm.org/docs/LangRef.html#ifuncs

use super::{
    linkage_types::LinkageTypes,
    types::Type,
    visibility_styles::VisibilityStyles,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct IFunc<T> {
    name: String,
    linkage: Option<LinkageTypes>,
    visibility: Option<VisibilityStyles>,
    ifunc_type: Type,
    resolver_type: Vec<Type>,
    resolver: T,
}

impl<T: std::fmt::Display> std::fmt::Display for IFunc<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = format!("@{} =", self.name);
        if self.linkage.is_some() {
            s = format!("{} {}", s, self.linkage.as_ref().unwrap());
        }
        if self.visibility.is_some() {
            s = format!("{} {}", s, self.visibility.as_ref().unwrap());
        }
        s = format!("{} ifunc {},", s, self.ifunc_type);

        let s = self
            .resolver_type
            .iter()
            .fold(s, |x, ty| format!("{} {}", x, ty));

        let s = format!("{} @{}", s, self.resolver);
        write!(f, "{}", s)
    }
}
