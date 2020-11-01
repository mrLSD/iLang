//! # Aggregate Types
//!
//! Aggregate Types are a subset of derived types that can contain
//! multiple member types. Arrays and structs are aggregate types.
//! Vectors are not considered to be aggregate types.
//!
//! https://llvm.org/docs/LangRef.html#aggregate-types

use super::super::types::Type;

/// The array type is a very simple derived type that arranges elements
/// sequentially in memory. The array type requires a size (number of
/// elements) and an underlying data type.
///
/// Syntax:
/// ```html
/// [<# elements> x <elementtype>]
/// ```
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ArrayType(i32, Box<Type>);

/// The structure type is used to represent a collection of data members
/// together in memory. The elements of a structure may be any type that
/// has a size.
///
/// Structures in memory are accessed using ‘load’ and ‘store’ by getting
/// a pointer to a field with the ‘getelementptr’ instruction. Structures
/// in registers are accessed using the ‘extractvalue’ and ‘insertvalue’
/// instructions.
///
/// Structures may optionally be “packed” structures, which indicate
/// that the alignment of the struct is one byte, and that there is no
/// padding between the elements. In non-packed structs, padding between
/// field types is inserted as defined by the DataLayout string in the
/// module, which is required to match what the underlying code generator
/// expects.
///
/// Structures can either be “literal” or “identified”.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct StructureType {
    pub literal: bool,
    pub packed: bool,
    pub type_list: Vec<Type>,
}

impl std::fmt::Display for ArrayType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = format!("[{} x {}]", self.0, self.1);
        write!(f, "{}", s)
    }
}

impl std::fmt::Display for StructureType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = self
            .type_list
            .iter()
            .enumerate()
            .fold("".to_string(), |s, (i, ty)| {
                // Calculation for comma for 1-th element
                if i > 0 {
                    format!("{}, {}", s, ty)
                } else {
                    format!("{} {}", s, ty)
                }
            });
        let s = if self.literal {
            if self.packed {
                format!("<{{ {} }}>", s)
            } else {
                format!("{{ {} }}", s)
            }
        } else if self.packed {
            format!("type {{ {} }}", s)
        } else {
            format!("type <{{ {} }}>", s)
        };
        write!(f, "{}", s)
    }
}
