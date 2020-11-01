//! # Basic LLVM types

use super::type_system::single_value::*;
use super::type_system::*;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Type {
    Void(VoidType),
    Function(FunctionType),
    Integer1(Integer1Type),
    Integer8(Integer8Type),
    Integer16(Integer16Type),
    Integer32(Integer32Type),
    Integer64(Integer64Type),
    Integer128(Integer128Type),
    FloatingPoint(FloatingPointType),
    Pointer(PointerType),
    Vector(VectorType),
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Type::Void(x) => format!("{}", x),
            Type::Function(x) => format!("{}", x),
            Type::Integer1(x) => format!("{}", x),
            Type::Integer8(x) => format!("{}", x),
            Type::Integer16(x) => format!("{}", x),
            Type::Integer32(x) => format!("{}", x),
            Type::Integer64(x) => format!("{}", x),
            Type::Integer128(x) => format!("{}", x),
            Type::FloatingPoint(x) => format!("{}", x),
            Type::Pointer(x) => format!("{}", x),
            Type::Vector(x) => format!("{}", x),
        };
        write!(f, "{}", s)
    }
}
