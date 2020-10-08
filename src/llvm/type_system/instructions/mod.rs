use crate::llvm::types::Type;

/// The ‘ret’ instruction is used to return control flow (and optionally
/// a value) from a function back to the caller.
///
/// There are two forms of the ‘ret’ instruction: one that returns a value
/// and then causes control flow, and one that just causes control flow to
/// occur.
/// https://llvm.org/docs/LangRef.html#id1437
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Ret<T>(Type, Option<T>);

impl<T: std::fmt::Display> std::fmt::Display for Ret<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(v) = &self.1 {
            write!(f, "ret {} {}", self.0, v)
        } else {
            write!(f, "ret void")
        }
    }
}

/// The ‘br’ instruction is used to cause control flow to transfer to a
/// different basic block in the current function. There are two forms of
/// this instruction, corresponding to a conditional branch and an
/// unconditional branch.
/// https://llvm.org/docs/LangRef.html#br-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Br {
    Conditional(bool, String, String),
    Unconditional(String),
}

impl std::fmt::Display for Br {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Br::Conditional(cond, if_true, if_false) => {
                format!("%{} %{} %{}", cond, if_true, if_false)
            }
            Br::Unconditional(dest) => format!("%{}", dest),
        };
        write!(f, "br {}", s)
    }
}
