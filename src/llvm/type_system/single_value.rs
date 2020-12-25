//! # Single Value Types
//!
//! These are the types that are valid in registers from CodeGen’s perspective.

use crate::llvm::types::Type;

/// The integer type is a very simple type that simply specifies an
/// arbitrary bit width for the integer type desired. Any bit width from 1
/// bit to 223-1 (about 8 million) can be specified.
/// https://llvm.org/docs/LangRef.html#integer-type
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct IntegerType<N>(N);

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Integer1Type;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Integer8Type;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Integer16Type;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Integer32Type;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Integer64Type;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Integer128Type;

/// The binary format of half, float, double, and fp128 correspond to
/// the IEEE-754-2008 specifications for binary16, binary32, binary64, and
/// binary128 respectively.
/// https://llvm.org/docs/LangRef.html#floating-point-types
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum FloatingPointType {
    Half,
    Bfloat,
    Float,
    Double,
    Fp128,
    X86fp80,
    PpcFp128,
}

/// The pointer type is used to specify memory locations. Pointers are
/// commonly used to reference objects in memory.
///
/// Pointer types may have an optional address space attribute defining
/// the numbered address space where the pointed-to object resides. The
/// default address space is number zero. The semantics of non-zero address
/// spaces are target-specific.
///
/// Note that LLVM does not permit pointers to void (void*) nor does it
/// permit pointers to labels (label*). Use i8* instead.
/// https://llvm.org/docs/LangRef.html#pointer-type
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PointerType(pub Box<Type>);

/// A vector type is a simple derived type that represents a vector of
/// elements. Vector types are used when multiple primitive data are
/// operated in parallel using a single instruction (SIMD). A vector type
/// requires a size (number of elements), an underlying primitive data
/// type, and a scalable property to represent vectors where the exact
/// hardware vector length is unknown at compile time. Vector types are
/// considered first class.
/// https://llvm.org/docs/LangRef.html#vector-type
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct VectorType {
    pub elemetns: i64,
    pub element_type: Box<Type>,
    pub vscale: bool,
}

impl<N: std::fmt::Display> std::fmt::Display for IntegerType<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "i{}", self.0)
    }
}

impl std::fmt::Display for Integer1Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", IntegerType(1))
    }
}

impl std::fmt::Display for Integer8Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", IntegerType(8))
    }
}

impl std::fmt::Display for Integer16Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", IntegerType(16))
    }
}

impl std::fmt::Display for Integer32Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", IntegerType(32))
    }
}

impl std::fmt::Display for Integer64Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", IntegerType(64))
    }
}

impl std::fmt::Display for Integer128Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", IntegerType(128))
    }
}

impl std::fmt::Display for FloatingPointType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            FloatingPointType::Half => "half",
            FloatingPointType::Bfloat => "bfloat",
            FloatingPointType::Float => "float",
            FloatingPointType::Double => "double",
            FloatingPointType::Fp128 => "fp128",
            FloatingPointType::X86fp80 => "x86_fp80",
            FloatingPointType::PpcFp128 => "ppc_fp80",
        };
        write!(f, "{}", s)
    }
}

impl std::fmt::Display for PointerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}*", self.0)
    }
}

impl std::fmt::Display for VectorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = if self.vscale {
            format!("<vscale x {} x {}>", self.elemetns, self.element_type)
        } else {
            format!("<{} x {}>", self.elemetns, self.element_type)
        };
        write!(f, "{}", s)
    }
}
