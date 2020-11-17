//! # Other Operations
//!
//! The instructions in this category are the “miscellaneous” instructions,
//! which defy better classification.
//!
//! https://llvm.org/docs/LangRef.html#other-operations

use crate::llvm::fast_math_flags::FastMathFlags;
use crate::llvm::types::Type;

/// The ‘icmp’ instruction returns a boolean value or a vector of
/// boolean values based on comparison of its two integer, integer
/// vector, pointer, or pointer vector operands.
///
/// The ‘icmp’ instruction takes three operands. The first operand is
/// the condition code indicating the kind of comparison to perform.
/// It is not a value, just a keyword. The possible condition codes
/// are:
///     eq: equal
///     ne: not equal
///     ugt: unsigned greater than
///     uge: unsigned greater or equal
///     ult: unsigned less than
///     ule: unsigned less or equal
///     sgt: signed greater than
///     sge: signed greater or equal
///     slt: signed less than
///     sle: signed less or equal
///
/// The remaining two arguments must be integer or pointer or integer
/// vector typed. They must also be identical types.
///
/// The ‘icmp’ compares op1 and op2 according to the condition code
/// given as cond. The comparison performed always yields either an
/// i1 or vector of i1 resul
///
/// https://llvm.org/docs/LangRef.html#icmp-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Icmp {
    pub res_val: String,
    pub cond: IcmpCondition,
    pub ty: Type,
    pub op1: String,
    pub op2: String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum IcmpCondition {
    Eq,
    Ne,
    Ugt,
    Uge,
    Ult,
    Ule,
    Sgt,
    Sge,
    Slt,
    Sle,
}

/// The ‘fcmp’ instruction returns a boolean value or vector of
/// boolean values based on comparison of its operands.
///
/// If the operands are floating-point scalars, then the result type
/// is a boolean (i1).
///
/// If the operands are floating-point vectors, then the result type
/// is a vector of boolean with the same number of elements as the
/// operands being compared.
///
/// The ‘fcmp’ instruction takes three operands. The first operand is
/// the condition code indicating the kind of comparison to perform.
/// It is not a value, just a keyword.
///
/// Ordered means that neither operand is a QNAN while unordered
/// means that either operand may be a QNAN.
///
/// Each of val1 and val2 arguments must be either a floating-point
/// type or a vector of floating-point type. They must have
/// identical types.
///
/// https://llvm.org/docs/LangRef.html#fcmp-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Fcmp {
    pub res_val: String,
    pub cond: FcmpCondition,
    pub fast_math_flags: Option<FastMathFlags>,
    pub ty: Type,
    pub op1: String,
    pub op2: String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum FcmpCondition {
    False,
    Oeq,
    Ogt,
    Oge,
    Olt,
    Ole,
    One,
    Ord,
    Ueq,
    Ugt,
    Uge,
    Ult,
    Ule,
    Une,
    Uno,
    True,
}

impl std::fmt::Display for Icmp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = format!(
            "%{} = icmp {} {} {}, {}",
            self.res_val, self.cond, self.ty, self.op1, self.op2
        );
        write!(f, "{}", s)
    }
}

impl std::fmt::Display for Fcmp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let fast_math = if let Some(v) = &self.fast_math_flags {
            format!("{}", v)
        } else {
            "".to_string()
        };
        let s = format!(
            "%{} = fcmp {} {} {} {}, {}",
            self.res_val, fast_math, self.cond, self.ty, self.op1, self.op2
        );
        write!(f, "{}", s)
    }
}

impl std::fmt::Display for IcmpCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            IcmpCondition::Eq => "eq",
            IcmpCondition::Ne => "ne",
            IcmpCondition::Ugt => "ugt",
            IcmpCondition::Uge => "uge",
            IcmpCondition::Ult => "ult",
            IcmpCondition::Ule => "ule",
            IcmpCondition::Sgt => "sgt",
            IcmpCondition::Sge => "sge",
            IcmpCondition::Slt => "slt",
            IcmpCondition::Sle => "sle",
        };
        write!(f, "{}", s)
    }
}

impl std::fmt::Display for FcmpCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            FcmpCondition::False => "false",
            FcmpCondition::Oeq => "oeq",
            FcmpCondition::Ogt => "ogt",
            FcmpCondition::Oge => "oge",
            FcmpCondition::Olt => "olt",
            FcmpCondition::Ole => "ole",
            FcmpCondition::One => "one",
            FcmpCondition::Ord => "ord",
            FcmpCondition::Ueq => "ueq",
            FcmpCondition::Ugt => "ugt",
            FcmpCondition::Uge => "uge",
            FcmpCondition::Ult => "ult",
            FcmpCondition::Ule => "ule",
            FcmpCondition::Une => "une",
            FcmpCondition::Uno => "uno",
            FcmpCondition::True => "true",
        };
        write!(f, "{}", s)
    }
}
