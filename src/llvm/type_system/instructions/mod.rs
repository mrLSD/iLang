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
/// Syntax:
/// ```html
/// br i1 <cond>, label <iftrue>, label <iffalse>
/// br label <dest>
/// ```
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
                format!("i1 %{}, label %{}, label %{}", cond, if_true, if_false)
            }
            Br::Unconditional(dest) => format!("label %{}", dest),
        };
        write!(f, "br {}", s)
    }
}

/// The ‘switch’ instruction is used to transfer control flow to one of
/// several different places. It is a generalization of the ‘br’
/// instruction, allowing a branch to occur to one of many possible destinations.
/// Syntax:
/// ```html
/// switch <intty> <value>, label <defaultdest> [ <intty> <val>, label <dest> ... ]
/// ```
/// https://llvm.org/docs/LangRef.html#switch-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Switch {
    choices: Vec<(String, String)>,
}

impl std::fmt::Display for Switch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = self
            .choices
            .iter()
            .enumerate()
            .fold("".to_string(), |s, (i, (int_ty, label))| {
                // First item
                let s = if i < 1 {
                    format!("i32 {}, label %{} [\n", int_ty, label)
                } else {
                    format!("{} i32 {}, label %{}\n", s, int_ty, label)
                };
                if i + 1 == self.choices.len() {
                    format!("{} \n]", s)
                } else {
                    s
                }
            });
        write!(f, "switch {}", s)
    }
}
