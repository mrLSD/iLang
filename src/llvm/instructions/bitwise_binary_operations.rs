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

/// The ‘ashr’ instruction (arithmetic shift right) returns the first
/// operand shifted to the right a specified number of bits with sign
/// extension.
///
/// Both arguments to the ‘ashr’ instruction must be the same integer
/// or vector of integer type. ‘op2’ is treated as an unsigned value.
///
/// This instruction always performs an arithmetic shift right
/// operation, The most significant bits of the result will be filled
/// with the sign bit of op1. If op2 is (statically or dynamically)
/// equal to or larger than the number of bits in op1, this instruction
/// returns a poison value. If the arguments are vectors, each vector
/// element of op1 is shifted by the corresponding shift amount in op2.
///
/// If the exact keyword is present, the result value of the ashr is a
/// poison value if any of the bits shifted out are non-zero.
///
/// https://llvm.org/docs/LangRef.html#ashr-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct AShr {
    pub result: String,
    pub exact: Option<()>,
    pub ty: Type,
    pub op1: String,
    pub op2: String,
}

/// The ‘and’ instruction returns the bitwise logical and of its two
/// operands.
///
/// The two arguments to the ‘and’ instruction must be integer or
/// vector of integer values. Both arguments must have identical
/// types.
///
/// https://llvm.org/docs/LangRef.html#and-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct And {
    pub result: String,
    pub ty: Type,
    pub op1: String,
    pub op2: String,
}

/// The ‘or’ instruction returns the bitwise logical inclusive or of
/// its two operands.
///
/// The two arguments to the ‘or’ instruction must be integer or
/// vector of integer values. Both arguments must have identical
/// types.
///
/// https://llvm.org/docs/LangRef.html#or-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Or {
    pub result: String,
    pub ty: Type,
    pub op1: String,
    pub op2: String,
}

/// The ‘xor’ instruction returns the bitwise logical exclusive or of
/// its two operands. The xor is used to implement the “one’s
/// complement” operation, which is the “~” operator in C.
///
/// The two arguments to the ‘xor’ instruction must be integer or
/// vector of integer values. Both arguments must have identical
/// types.
///
/// https://llvm.org/docs/LangRef.html#xor-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Xor {
    pub result: String,
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

impl std::fmt::Display for AShr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = "ashr".to_string();
        if self.exact.is_some() {
            s = format!("{} exact", s)
        }
        s = format!("{} {} {}, {}", s, self.ty, self.op1, self.op2);
        write!(f, "{}", s)
    }
}

impl std::fmt::Display for And {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = "and".to_string();
        s = format!("{} {} {}, {}", s, self.ty, self.op1, self.op2);
        write!(f, "{}", s)
    }
}

impl std::fmt::Display for Or {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = "or".to_string();
        s = format!("{} {} {}, {}", s, self.ty, self.op1, self.op2);
        write!(f, "{}", s)
    }
}

impl std::fmt::Display for Xor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = "xor".to_string();
        s = format!("{} {} {}, {}", s, self.ty, self.op1, self.op2);
        write!(f, "{}", s)
    }
}
