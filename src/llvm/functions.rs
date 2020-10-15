//! # Functions
//!
//! LLVM function definitions consist of the “define”
//! keyword, and an optional fields.
//!
//! LLVM function declarations consist of the “declare”
//! keyword, an optional fields.
//!
//! A function definition contains a list of basic blocks, forming the
//! CFG (Control Flow Graph) for the function. Each basic block may
//! optionally start with a label (giving the basic block a symbol table
//! entry), contains a list of instructions, and ends with a terminator
//! instruction (such as a branch or function return). If an explicit
//! label name is not provided, a block is assigned an implicit numbered
//! label, using the next value from the same counter as used for unnamed
//! temporaries (see above). For example, if a function entry block does
//! not have an explicit label, it will be assigned label “%0”, then the
//! first unnamed temporary in that block will be “%1”, etc. If a numeric
//! label is explicitly specified, it must match the numeric label that
//! would be used implicitly.
//!
//! The first basic block in a function is special in two ways: it is
//! immediately executed on entrance to the function, and it is not
//! allowed to have predecessor basic blocks (i.e. there can not be
//! any branches to the entry block of a function). Because the block
//! can have no predecessors, it also cannot have any PHI nodes.
//!
//! Syntax:
//! ```html
//! define [linkage] [PreemptionSpecifier] [visibility] [DLLStorageClass]
//!        [cconv] [ret attrs]
//!        <ResultType> @<FunctionName> ([argument list])
//!        [(unnamed_addr|local_unnamed_addr)] [AddrSpace] [fn Attrs]
//!        [section "name"] [comdat [($name)]] [align N] [gc] [prefix Constant]
//!        [prologue Constant] [personality Constant] (!name !N)* { ... }
//! ```
//!
//! The argument list is a comma separated sequence of arguments where
//! each argument is of the following form:
//!
//! Syntax:
//! ```html
//! <type> [parameter Attrs] [name]
//! ```
//! https://llvm.org/docs/LangRef.html#functions

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Function;

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = "";
        write!(f, "{}", s)
    }
}
