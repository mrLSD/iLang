//! # Binary Operations
//!
//! Binary operators are used to do most of the computation in a program.
//! They require two operands of the same type, execute an operation on
//! them, and produce a single value. The operands might represent multiple
//! data, as is the case with the vector data type. The result value has
//! the same type as its operands.
//!
//! https://llvm.org/docs/LangRef.html#binary-operations

use crate::llvm::fast_math_flags::FastMathFlags;
use crate::llvm::types::Type;

/// The ‘add’ instruction returns the sum of its two operands.
///
/// The two arguments to the ‘add’ instruction must be integer or
/// vector of integer values. Both arguments must have identical
/// types.
///
/// The value produced is the integer sum of the two operands.
///
/// https://llvm.org/docs/LangRef.html#add-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Add {
    pub result: String,
    pub nuw: Option<()>,
    pub nsw: Option<()>,
    pub ty: Type,
    pub op1: String,
    pub op2: String,
}

/// The ‘fadd’ instruction returns the sum of its two operands.
///
/// The two arguments to the ‘fadd’ instruction must be floating-point or vector of floating-point values. Both arguments must have identical types.
///
/// https://llvm.org/docs/LangRef.html#fadd-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FAdd {
    pub result: String,
    pub fast_math_flags: Option<FastMathFlags>,
    pub ty: Type,
    pub op1: String,
    pub op2: String,
}

/// https://llvm.org/docs/LangRef.html#sub-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Sub();

impl std::fmt::Display for Add {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = "add".to_string();
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

impl std::fmt::Display for FAdd {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = "fadd".to_string();
        if let Some(v) = &self.fast_math_flags {
            s = format!("{} {}", s, v)
        }
        s = format!("{} {} {}, {}", s, self.ty, self.op1, self.op2);
        write!(f, "{}", s)
    }
}
