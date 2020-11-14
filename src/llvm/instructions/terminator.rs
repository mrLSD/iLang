//! # Terminator Instructions
//!
//! Every basic block in a program ends with a “Terminator” instruction,
//! which indicates which block should be executed after the current block
//! is finished. These terminator instructions typically yield a ‘void’
//! value: they produce control flow, not values (the one exception being
//! the ‘invoke’ instruction).
//!
//! https://llvm.org/docs/LangRef.html#terminator-instructions
use crate::llvm::addrspace::AddrSpace;
use crate::llvm::calling_convention::CallingConvention;
use crate::llvm::function_attributes::FunctionAttributes;
use crate::llvm::parameter_attributes::ParameterAttributes;
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

/// The ‘invoke’ instruction causes control to transfer to a specified
/// function, with the possibility of control flow transfer to either
/// the ‘normal’ label or the ‘exception’ label. If the callee
/// function returns with the “ret” instruction, control flow will
/// return to the “normal” label. If the callee (or any indirect
/// callees) returns via the “resume” instruction or other exception
/// handling mechanism, control is interrupted and continued at the
/// dynamically nearest “exception” label.
///
/// The ‘exception’ label is a landing pad for the exception. As
/// such, ‘exception’ label is required to have the “landingpad”
/// instruction, which contains the information about the behavior of the
/// program after unwinding happens, as its first non-PHI instruction.
/// The restrictions on the “landingpad” instruction’s tightly couples
/// it to the “invoke” instruction, so that the important information
/// contained within the “landingpad” instruction can’t be lost through
/// normal code motion.
///
/// This instruction requires several arguments:
///
/// 1.  The optional “cconv” marker indicates which calling convention
///     the call should use. If none is specified, the call defaults to
///     using C calling conventions.
/// 2.  The optional Parameter Attributes list for return values.
///     Only
///     ‘zeroext’, ‘signext’, and ‘inreg’ attributes are valid here.
/// 3.  The optional addrspace attribute can be used to indicate the
///     address space of the called function. If it is not specified,
///     the program address space from the datalayout string will be
///     used.
/// 4.  ‘ty’: the type of the call instruction itself which is also
///     the type of the return value. Functions that return no value
///     are marked void.
/// 5.  ‘fnty’: shall be the signature of the function being invoked.
///     The argument types must match the types implied by this
///     signature. This type can be omitted if the function is not
///     varargs.
/// 6.  ‘fnptrval’: An LLVM value containing a pointer to a function
///     to be invoked. In most cases, this is a direct function
///     invocation, but indirect invoke’s are just as possible,
///     calling an arbitrary pointer to function value.
/// 7.  ‘function args’: argument list whose types match the function
///     signature argument types and parameter attributes. All
///     arguments must be of first class type. If the function
///     signature indicates the function accepts a variable number of
///     arguments, the extra arguments can be specified.
/// 8.  ‘normal label’: the label reached when the called function
///     executes a ‘ret’ instruction.
/// 9.  ‘exception label’: the label reached when a callee returns via
///     the resume instruction or other exception handling mechanism.
/// 10. The optional function attributes list.
/// 12. The optional operand bundles list.
///
/// Syntax:
/// ```html
/// <result> = invoke [cconv] [ret attrs] [addrspace(<num>)] <ty>|<fnty> <fnptrval>(<function args>) [fn attrs]
/// [operand bundles] to label <normal label> unwind label <exception label>
/// ```
///
/// https://llvm.org/docs/LangRef.html#invoke-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Invoke<P> {
    pub ret_val: String,
    pub cconv: Option<CallingConvention>,
    pub ret_attr: Option<ParameterAttributes<P>>,
    pub addrspace: Option<AddrSpace>,
    pub ty: Type,
    pub fnty: Option<Type>,
    pub fnptrval: (bool, String), // first param indicate is it ptr
    pub function_args: Vec<FunctionArg>,
    pub function_attrs: Option<FunctionAttributes>,
    pub operand_bundles: String,
    pub normal_label: String,
    pub exception_label: String,
}

/// Fucntion argument contain type and their value
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FunctionArg(Type, String);

/// The ‘callbr’ instruction causes control to transfer to a specified
/// function, with the possibility of control flow transfer to either
/// the ‘fallthrough’ label or one of the ‘indirect’ labels.
///
/// This instruction should only be used to implement the “goto”
/// feature of gcc style inline assembly. Any other usage is an
/// error in the IR verifier.
///
/// This instruction requires several arguments:
///
/// 1.  The optional “cconv” marker indicates which calling convention
///     the call should use. If none is specified, the call defaults to
///     using C calling conventions.
/// 2.  The optional Parameter Attributes list for return values. Only
///     ‘zeroext’, ‘signext’, and ‘inreg’ attributes are valid here.
/// 3.  The optional addrspace attribute can be used to indicate the
///     address space of the called function. If it is not specified,
///     the program address space from the datalayout string will be used.
/// 4.  ‘ty’: the type of the call instruction itself which is also
///     the type of the return value. Functions that return no value
///     are marked void.
/// 5.  ‘fnty’: shall be the signature of the function being called.
///     The argument types must match the types implied by this
///     signature. This type can be omitted if the function is not
///     varargs.
/// 6.  ‘fnptrval’: An LLVM value containing a pointer to a function
///     to be called. In most cases, this is a direct function call,
///     but other callbr’s are just as possible, calling an arbitrary
///     pointer to function value.
/// 7.  ‘function args’: argument list whose types match the function
///     signature argument types and parameter attributes. All
///     arguments must be of first class type. If the function
///     signature indicates the function accepts a variable number of
///     arguments, the extra arguments can be specified.
/// 8.  ‘fallthrough label’: the label reached when the inline
///     assembly’s execution exits the bottom.
/// 9.  ‘indirect labels’: the labels reached when a callee transfers
///     control to a location other than the ‘fallthrough label’. The
///     blockaddress constant for these should also be in the list of
///     ‘function args’.
/// 10. The optional function attributes list.
/// 11. The optional operand bundles list.
///
/// https://llvm.org/docs/LangRef.html#callbr-instruction
/// TODO: implement Syntax.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CallBr();

/// The ‘resume’ instruction is a terminator instruction that has no
/// successors.
///
/// The ‘resume’ instruction requires one argument, which must have
/// the same type as the result of any ‘landingpad’ instruction in
/// the same function.
///
/// https://llvm.org/docs/LangRef.html#resume-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Resume {
    pub resume_type: Type,
    pub value: String,
}

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

impl<P: std::fmt::Display> std::fmt::Display for Invoke<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = format!("%{} = invoke", self.ret_val);
        if let Some(v) = &self.cconv {
            s = format!("{} {}", s, v)
        }
        write!(f, "{}", s)
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
        write!(f, "resume {} %{}", self.resume_type, self.value)
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
