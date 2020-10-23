//! # Type System
//! The LLVM type system is one of the most important features of the
//! intermediate representation. Being typed enables a number of
//! optimizations to be performed on the intermediate representation
//! directly, without having to do extra analyses on the side before the
//! transformation. A strong type system makes it easier to read the
//! generated code and enables novel analyses and transformations that
//! are not feasible to perform on normal three address code
//! representations.

use super::types::Type;

/// The void type does not represent any value and has no size.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct VoidType;

/// The function type can be thought of as a function signature. It
/// consists of a return type and a list of formal parameter types. The
/// return type of a function type is a void type or first class type —
/// except for label and metadata types.
/// Syntax:
/// ```html
/// <returntype> (<parameter list>)
/// ```
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FunctionType {
    return_type: Type,
    parameter_list: Vec<Type>,
    variable_argument: bool,
}

impl std::fmt::Display for VoidType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = "void";
        write!(f, "{}", s)
    }
}

impl std::fmt::Display for FunctionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = self
            .parameter_list
            .iter()
            .enumerate()
            .fold("".to_string(), |s, (i, t)| {
                // Check for comma
                if i > 0 {
                    format!("{}, {}", s, t)
                } else {
                    format!("{} {}", s, t)
                }
            });
        let s = if self.variable_argument {
            if s.is_empty() {
                "...".to_string()
            } else {
                format!("{}, ...", s)
            }
        } else {
            s
        };
        let s = format!("{} ({})", self.return_type, s);
        write!(f, "{}", s)
    }
}
