//! # Bitwise Binary Operations
//!
//! Bitwise binary operators are used to do various forms of
//! bit-twiddling in a program. They are generally very efficient
//! instructions and can commonly be strength reduced from other
//! instructions. They require two operands of the same type, execute
//! an operation on them, and produce a single value. The resulting
//! value is the same type as its operands.
//!
//! https://llvm.org/docs/LangRef.html#bitwise-binary-operations

use crate::llvm::types::Type;

/// The ‘shl’ instruction returns the first operand shifted to the
/// left a specified number of bits.
///
/// Both arguments to the ‘shl’ instruction must be the same integer
/// or vector of integer type. ‘op2’ is treated as an unsigned value.
///
/// https://llvm.org/docs/LangRef.html#shl-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Shl {
    pub result: String,
    pub nuw: Option<()>,
    pub nsw: Option<()>,
    pub ty: Type,
    pub op1: String,
    pub op2: String,
}

/// The ‘lshr’ instruction (logical shift right) returns the first
/// operand shifted to the right a specified number of bits with zero
/// fill.
///
/// Both arguments to the ‘lshr’ instruction must be the same integer
/// or vector of integer type. ‘op2’ is treated as an unsigned value.
///
/// https://llvm.org/docs/LangRef.html#lshr-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LShl {
    pub result: String,
    pub exact: Option<()>,
    pub ty: Type,
    pub op1: String,
    pub op2: String,
}

impl std::fmt::Display for Shl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = "shl".to_string();
        if self.nuw.is_some() {
            s = format!("{} nuw", s)
        }
        if self.nsw.is_some() {
            s = format!("{} nsw", s)
        }
        s = format!("{} {} {}, {}", s, self.ty, self.op1, self.op2);
        write!(f, "{}", s)
    }
}

impl std::fmt::Display for LShl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = "lshl".to_string();
        if self.exact.is_some() {
            s = format!("{} exact", s)
        }
        s = format!("{} {} {}, {}", s, self.ty, self.op1, self.op2);
        write!(f, "{}", s)
    }
}
