//! # Other Operations
//!
//! The instructions in this category are the “miscellaneous” instructions,
//! which defy better classification.
//!
//! https://llvm.org/docs/LangRef.html#other-operations

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

impl std::fmt::Display for Icmp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = format!(
            "%{} = icmp {} {} {}, {}",
            self.res_val, self.cond, self.ty, self.op1, self.op2
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
