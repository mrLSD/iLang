//! # Basic LLVM types

use super::type_system::{
    aggregate::*,
    single_value::*,
    *,
};

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
    Array(ArrayType),
    Structure(StructureType),
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
            Type::Array(x) => format!("{}", x),
            Type::Structure(x) => format!("{}", x),
        };
        write!(f, "{}", s)
    }
}

impl Type {
    pub fn pointer1(ty: Type) -> Self {
        Type::Pointer(PointerType(Box::new(ty)))
    }

    pub fn pointer2(ty: Type) -> Self {
        let ty1 = Type::Pointer(PointerType(Box::new(ty)));
        Type::Pointer(PointerType(Box::new(ty1)))
    }

    pub fn pointer3(ty: Type) -> Self {
        let ty1 = Type::Pointer(PointerType(Box::new(ty)));
        let ty2 = Type::Pointer(PointerType(Box::new(ty1)));
        Type::Pointer(PointerType(Box::new(ty2)))
    }

    pub fn raw_string(s: &str) -> String {
        format!(r#"c"{}\00""#, s)
    }
}
