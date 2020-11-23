//! # Unary Operations
//!
//! Unary operators require a single operand, execute an operation on it,
//! and produce a single value. The operand might represent multiple data,
//! as is the case with the vector data type. The result value has the
//! same type as its operand.
//!
//! https://llvm.org/docs/LangRef.html#unary-operations

use crate::llvm::fast_math_flags::FastMathFlags;
use crate::llvm::types::Type;

/// The ‘fneg’ instruction returns the negation of its operand.
/// Syntax:
/// ```html
/// <result> = fneg [fast-math flags]* <ty> <op1>   ; yields ty:result
/// ```
/// https://llvm.org/docs/LangRef.html#fneg-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FNeg {
    pub fast_math_flags: Vec<FastMathFlags>,
    pub ty: Type,
    pub operand: String,
}

impl std::fmt::Display for FNeg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = self
            .fast_math_flags
            .iter()
            .fold("".to_string(), |s, x| format!("{} {}", s, x));
        write!(f, "fneg {} {} %{}", s, self.ty, self.operand)
    }
}
