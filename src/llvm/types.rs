//! # Basic LLVM types

use super::type_system::single_value::*;
use super::type_system::*;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Type {
    Void,
    Function(FunctionType),
    Integer1,
    Integer8,
    Integer16,
    Integer32,
    Integer64,
    Integer128,
    FloatingPoint(FloatingPointType),
    Pointer(PointerType),
    Vector(VectorType),
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Type::Void => format!("{}", VoidType),
            Type::Function(x) => format!("{}", x),
            Type::Integer1 => format!("{}", Integer1Type),
            Type::Integer8 => format!("{}", Integer8Type),
            Type::Integer16 => format!("{}", Integer16Type),
            Type::Integer32 => format!("{}", Integer32Type),
            Type::Integer64 => format!("{}", Integer64Type),
            Type::Integer128 => format!("{}", Integer128Type),
            Type::FloatingPoint(x) => format!("{}", x),
            Type::Pointer(x) => format!("{}", x),
            Type::Vector(x) => format!("{}", x),
        };
        write!(f, "{}", s)
    }
}
