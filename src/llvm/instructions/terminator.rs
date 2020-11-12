//! # Terminator Instructions
//!
//! Every basic block in a program ends with a “Terminator” instruction,
//! which indicates which block should be executed after the current block
//! is finished. These terminator instructions typically yield a ‘void’
//! value: they produce control flow, not values (the one exception being
//! the ‘invoke’ instruction).
//!
//! https://llvm.org/docs/LangRef.html#terminator-instructions
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
    pub choices: Vec<(String, String)>,
}

/// The ‘indirectbr’ instruction implements an indirect branch to a label
/// within the current function, whose address is specified by “address”.
/// Address must be derived from a blockaddress constant.
/// Syntax:
/// ```html
/// indirectbr <somety>* <address>, [ label <dest1>, label <dest2>, ... ]
/// ```
/// https://llvm.org/docs/LangRef.html#indirectbr-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct IndirectBr {
    pub ty: String,
    pub address: String,
    pub choices: Vec<(String, String)>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Invoke();

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CallBr();

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Resume();

/// The ‘catchswitch’ instruction is used by LLVM’s exception handling
/// system to describe the set of possible catch handlers that may be
/// executed by the EH personality routine.
///
/// The parent argument is the token of the funclet that contains the
/// catchswitch instruction. If the catchswitch is not inside a
/// funclet, this operand may be the token none.
///
/// The default argument is the label of another basic block beginning
/// with either a cleanuppad or catchswitch instruction. This unwind
/// destination must be a legal target with respect to the parent
/// links, as described in the exception handling documentation.
///
/// The handlers are a nonempty list of successor blocks that each
/// begin with a catchpad instruction.
///
/// https://llvm.org/docs/LangRef.html#catchswitch-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CatchSwitch {
    pub result_val: String,
    pub parent: String,
    pub handler_labels: Vec<String>,
    pub default_label: Option<String>,
}

/// The ‘catchret’ instruction is a terminator instruction that has a
/// single successor.
///
/// The first argument to a ‘catchret’ indicates which catchpad it
/// exits. It must be a catchpad. The second argument to a ‘catchret’
/// specifies where control will transfer to next.
///
/// https://llvm.org/docs/LangRef.html#catchret-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CatchRet {
    pub catch: String,
    pub continue_label: String,
}

/// The ‘cleanupret’ instruction is a terminator instruction that has
/// an optional successor.
///
/// The ‘cleanupret’ instruction requires one argument, which indicates
/// which cleanuppad it exits, and must be a cleanuppad. If the specified
/// cleanuppad is not the most-recently-entered not-yet-exited funclet
/// pad (as described in the EH documentation), the cleanupret’s
/// behavior is undefined.
///
/// https://llvm.org/docs/LangRef.html#cleanupret-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CleanupRet {
    pub values: String,
    pub continue_label: Option<String>,
}

/// The ‘unreachable’ instruction has no defined semantics. This
/// instruction is used to inform the optimizer that a particular
/// portion of the code is not reachable. This can be used to indicate
/// that the code after a no-return function cannot be reached, and
/// other facts.
///
/// https://llvm.org/docs/LangRef.html#unreachable-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Unreachable();

impl<T: std::fmt::Display> std::fmt::Display for Ret<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(v) = &self.1 {
            write!(f, "ret {} {}", self.0, v)
        } else {
            write!(f, "ret void")
        }
    }
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

impl std::fmt::Display for IndirectBr {
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

impl std::fmt::Display for Invoke {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = "";
        write!(f, "switch {}", s)
    }
}

impl std::fmt::Display for CallBr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = "";
        write!(f, "switch {}", s)
    }
}

impl std::fmt::Display for Resume {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = "";
        write!(f, "switch {}", s)
    }
}

impl std::fmt::Display for CatchSwitch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let labels = self
            .handler_labels
            .iter()
            .enumerate()
            .fold("".to_string(), |s, (i, v)| {
                if i > 1 {
                    format!("{}, label %{}", s, v)
                } else {
                    format!("label %{}", v)
                }
            });
        let default = if let Some(x) = &self.default_label {
            format!("unwind label %{}", x)
        } else {
            "unwind to caller".to_string()
        };
        write!(
            f,
            "%{} = catchswitch within %{} [{}] {}",
            self.result_val, self.parent, labels, default
        )
    }
}

impl std::fmt::Display for CatchRet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "catchret from %{} label %{}",
            self.catch, self.continue_label
        )
    }
}

impl std::fmt::Display for CleanupRet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = if let Some(v) = &self.continue_label {
            format!("%{} unwind label %{}", self.values, v)
        } else {
            format!("%{} unwind to caller", self.values)
        };
        write!(f, "cleanupret from {}", s)
    }
}

impl std::fmt::Display for Unreachable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "unreachable")
    }
}
