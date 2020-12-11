//! # Other Operations
//!
//! The instructions in this category are the “miscellaneous” instructions,
//! which defy better classification.
//!
//! https://llvm.org/docs/LangRef.html#other-operations

use crate::llvm::{
    addrspace::AddrSpace,
    calling_convention::CallingConvention,
    fast_math_flags::FastMathFlags,
    function_attributes::FunctionAttributes,
    instructions::terminator::FunctionArg,
    parameter_attributes::ParameterAttributes,
    types::Type,
};

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

/// The ‘icmp’ compares op1 and op2 according to the condition code
/// given as cond. The comparison performed always yields either an
/// i1 or vector of i1 result, as follows:
///
/// eq: yields true if the operands are equal, false otherwise. No sign interpretation is necessary or performed.
/// ne: yields true if the operands are unequal, false otherwise. No sign interpretation is necessary or performed.
/// ugt: interprets the operands as unsigned values and yields true if op1 is greater than op2.
/// uge: interprets the operands as unsigned values and yields true if op1 is greater than or equal to op2.
/// ult: interprets the operands as unsigned values and yields true if op1 is less than op2.
/// ule: interprets the operands as unsigned values and yields true if op1 is less than or equal to op2.
/// sgt: interprets the operands as signed values and yields true if op1 is greater than op2.
/// sge: interprets the operands as signed values and yields true if op1 is greater than or equal to op2.
/// slt: interprets the operands as signed values and yields true if op1 is less than op2.
/// sle: interprets the operands as signed values and yields true if op1 is less than or equal to op2.
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

/// The ‘fcmp’ instruction compares op1 and op2 according to the
/// condition code given as cond. If the operands are vectors, then
/// the vectors are compared element by element. Each comparison
/// performed always yields an i1 result, as follows:
///
/// false: always yields false, regardless of operands.
/// oeq: yields true if both operands are not a QNAN and op1 is equal to op2.
/// ogt: yields true if both operands are not a QNAN and op1 is greater than op2.
/// oge: yields true if both operands are not a QNAN and op1 is greater than or equal to op2.
/// olt: yields true if both operands are not a QNAN and op1 is less than op2.
/// ole: yields true if both operands are not a QNAN and op1 is less than or equal to op2.
/// one: yields true if both operands are not a QNAN and op1 is not equal to op2.
/// ord: yields true if both operands are not a QNAN.
/// ueq: yields true if either operand is a QNAN or op1 is equal to op2.
/// ugt: yields true if either operand is a QNAN or op1 is greater than op2.
/// uge: yields true if either operand is a QNAN or op1 is greater than or equal to op2.
/// ult: yields true if either operand is a QNAN or op1 is less than op2.
/// ule: yields true if either operand is a QNAN or op1 is less than or equal to op2.
/// une: yields true if either operand is a QNAN or op1 is not equal to op2.
/// uno: yields true if either operand is a QNAN.
/// true: always yields true, regardless of operands.
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

/// The ‘phi’ instruction is used to implement the φ node in the SSA
/// graph representing the function.
///
/// The type of the incoming values is specified with the first type
/// field. After this, the ‘phi’ instruction takes a list of pairs as
/// arguments, with one pair for each predecessor basic block of the
/// current block. Only values of first class type may be used as the
/// value arguments to the PHI node. Only labels may be used as the
/// label arguments.
///
/// There must be no non-phi instructions between the start of a
/// basic block and the PHI instructions: i.e. PHI instructions must
/// be first in a basic block.
///
/// For the purposes of the SSA form, the use of each incoming value
/// is deemed to occur on the edge from the corresponding predecessor
/// block to the current block (but after any definition of an
/// ‘invoke’ instruction’s return value on the same edge).
///
/// The optional fast-math-flags marker indicates that the phi has
/// one or more fast-math-flags. These are optimization hints to
/// enable otherwise unsafe floating-point optimizations. Fast-math-flags
/// are only valid for phis that return a floating-point scalar or
/// vector type, or an array (nested to any depth) of floating-point
/// scalar or vector types.
///
/// https://llvm.org/docs/LangRef.html#phi-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Phi {
    pub res_val: String,
    pub fast_math_flags: Option<FastMathFlags>,
    pub ty: Type,
    pub params: Vec<(String, String)>,
}

/// The ‘select’ instruction is used to choose one value based on a
/// condition, without IR-level branching.
///
/// The ‘select’ instruction requires an ‘i1’ value or a vector of
/// ‘i1’ values indicating the condition, and two values of the same
/// first class type.
///
/// The optional fast-math flags marker indicates that the select
/// has one or more fast-math flags. These are optimization hints to
/// enable otherwise unsafe floating-point optimizations. Fast-math
/// flags are only valid for selects that return a floating-point
/// scalar or vector type, or an array (nested to any depth) of
/// floating-point scalar or vector types.
///
/// If the condition is an i1 and it evaluates to 1, the instruction
/// returns the first value argument; otherwise, it returns the second
/// value argument.
///
/// If the condition is a vector of i1, then the value arguments must
/// be vectors of the same size, and the selection is done element by
/// element.
///
/// If the condition is an i1 and the value arguments are vectors of
/// the same size, then an entire vector is selected.
///
/// https://llvm.org/docs/LangRef.html#select-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Select {
    pub res_val: String,
    pub fast_math_flags: Option<FastMathFlags>,
    pub selty: Type,
    pub cond: String,
    pub ty1: Type,
    pub val1: String,
    pub ty2: Type,
    pub val2: String,
}

///  https://llvm.org/docs/LangRef.html#call-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Call {
    pub ret_val: String,
    pub tail: Option<TailCall>,
    pub fast_math_flags: Option<FastMathFlags>,
    pub cconv: Option<CallingConvention>,
    pub ret_attr: Option<ParameterAttributes>,
    pub addrspace: Option<AddrSpace>,
    pub ty: Type,
    pub fnty: Option<String>,
    pub fnptrval: (bool, String),
    // first param indicate is it ptr
    pub function_args: Vec<FunctionArg>,
    pub function_attrs: Option<FunctionAttributes>,
    pub operand_bundles: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TailCall {
    Tail,
    MustTail,
    NoTail,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Landingpad {
    pub resultval: String,
    pub resultty: Type,
    pub cleanup: Option<()>,
    pub clause: Vec<Clause>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Clause {
    pub clause_type: Type,
    pub value: String,
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

impl std::fmt::Display for Phi {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let fast_math = if let Some(v) = &self.fast_math_flags {
            format!("{}", v)
        } else {
            "".to_string()
        };
        let params = self
            .params
            .iter()
            .enumerate()
            .fold("".to_string(), |s, (i, v)| {
                if i > 0 {
                    format!("{}, [{}, {}]", s, v.0, v.1)
                } else {
                    format!("{} [{}, {}]", s, v.0, v.1)
                }
            });

        let s = format!(
            "%{} = phi {} {} {}",
            self.res_val, fast_math, self.ty, params
        );
        write!(f, "{}", s)
    }
}

impl std::fmt::Display for Select {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let fast_math = if let Some(v) = &self.fast_math_flags {
            format!("{}", v)
        } else {
            "".to_string()
        };
        let s = format!(
            "%{} = select {} {} {}, {} {}, {} {}",
            self.res_val,
            fast_math,
            self.selty,
            self.cond,
            self.ty1,
            self.val1,
            self.ty2,
            self.val2,
        );
        write!(f, "{}", s)
    }
}

impl std::fmt::Display for TailCall {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            TailCall::Tail => "tail",
            TailCall::MustTail => "musttail",
            TailCall::NoTail => "notail",
        };
        write!(f, "{}", s)
    }
}

impl std::fmt::Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let tail = if let Some(v) = &self.tail {
            format!("{}", v)
        } else {
            "".to_string()
        };
        let fast_math = if let Some(v) = &self.fast_math_flags {
            format!("{}", v)
        } else {
            "".to_string()
        };

        let mut s = format!("%{} = {} call {}", self.ret_val, tail, fast_math);
        if let Some(v) = &self.cconv {
            s = format!("{} {}", s, v)
        }
        if let Some(v) = &self.ret_attr {
            s = format!("{} {}", s, v)
        }
        if let Some(v) = &self.addrspace {
            s = format!("{} {}", s, v)
        }
        s = format!("{} {}", s, &self.ty);
        if let Some(v) = &self.fnty {
            s = format!("{} {}", s, v)
        }
        // Check is it Ptr
        if self.fnptrval.0 {
            s = format!("{} %{}", s, self.fnptrval.1)
        } else {
            s = format!("{} @{}", s, self.fnptrval.1)
        }
        let args = self
            .function_args
            .iter()
            .enumerate()
            .fold("".to_string(), |s, (i, v)| {
                if i > 0 {
                    format!("{}, {} {}", s, v.0, v.1)
                } else {
                    format!("{} {} {}", s, v.0, v.1)
                }
            });
        s = format!("{} ({})", s, args);
        if let Some(v) = &self.function_attrs {
            s = format!("{} {}", s, v)
        }
        if let Some(v) = &self.operand_bundles {
            s = format!("{} {}", s, v)
        }

        write!(f, "{}", s)
    }
}

impl std::fmt::Display for Landingpad {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = format!("{} = landingpad {} ", self.resultval, self.resultty);
        if self.cleanup.is_some() {
            s = format!("{} cleanup", s);
        }
        let clause = self
            .clause
            .iter()
            .fold("".to_string(), |s, v| format!("{} {}", s, v));

        s = format!("{} {}", s, clause);

        write!(f, "{}", s)
    }
}

impl std::fmt::Display for Clause {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "catch {} {}", self.clause_type, self.value)
    }
}
