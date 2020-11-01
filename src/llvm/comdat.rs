//! # Comdats
//!
//! Comdat IR provides access to COFF and ELF object file COMDAT
//! functionality.
//!
//! Comdats have a name which represents the COMDAT key. All global
//! objects that specify this key will only end up in the final object
//! file if the linker chooses that key over some other key. Aliases are
//! placed in the same COMDAT that their aliasee computes to, if any.
//!
//! Comdats have a selection kind to provide input on how the linker should
//! choose between keys in two different object files.
//!
//! Syntax:
//! ```html
//! $<Name> = comdat SelectionKind
//! ```
//! https://llvm.org/docs/LangRef.html#comdats

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ComDat {
    name: String,
    selection_kind: SelectionKind,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SelectionKind {
    Any,
    ExactMatch,
    Largest,
    NoDuplicates,
    SameSize,
}

impl std::fmt::Display for ComDat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self.selection_kind {
            SelectionKind::Any => "any",
            SelectionKind::ExactMatch => "exactmatch",
            SelectionKind::Largest => "largest",
            SelectionKind::NoDuplicates => "noduplicates",
            SelectionKind::SameSize => "samesize",
        };
        let s = format!("${} = {}", self.name, s);
        write!(f, "{}", s)
    }
}
