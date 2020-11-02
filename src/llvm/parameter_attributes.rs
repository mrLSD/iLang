//! # Parameter Attributes
//!
//! The return type and each parameter of a function type may have a set of
//! parameter attributes associated with them. Parameter attributes are
//! used to communicate additional information about the result or
//! parameters of a function. Parameter attributes are considered to
//! be part of the function, not of the function type, so functions
//! with different parameter attributes can have the same function
//! type.
//!
//! Parameter attributes are simple keywords that follow the type
//! specified. If multiple parameter attributes are needed, they are
//! space separated.
//!
//! ## Documentation
//! https://llvm.org/docs/LangRef.html#parameter-attributes
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ParameterAttributes<T> {
    ZeroExt,
    SignExt,
    InReg,
    ByVal,
    ByRef(T),
    PreAllocated(T),
    InAlloca,
    Sret,
    Allign(T),
    NoAlias,
    NoCapture,
    NoFree,
    Nest,
    Returned,
    NonNull,
    Dereferenceable(T),
    DereferenceableOrNull(T),
    SwiftSelf,
    SwiftError,
    ImmArg,
    NoUndef,
}

impl<T: std::fmt::Display> std::fmt::Display for ParameterAttributes<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            ParameterAttributes::ZeroExt => "zeroext".to_string(),
            ParameterAttributes::SignExt => "signext".to_string(),
            ParameterAttributes::InReg => "inreg".to_string(),
            ParameterAttributes::ByVal => "byval".to_string(),
            ParameterAttributes::ByRef(x) => format!("byref({})", x),
            ParameterAttributes::PreAllocated(x) => format!("preallocated({})", x),
            ParameterAttributes::InAlloca => "inalloca".to_string(),
            ParameterAttributes::Sret => "sret".to_string(),
            ParameterAttributes::Allign(x) => format!("align {}", x),
            ParameterAttributes::NoAlias => "noalias".to_string(),
            ParameterAttributes::NoCapture => "nocapture".to_string(),
            ParameterAttributes::NoFree => "nofree".to_string(),
            ParameterAttributes::Nest => "nest".to_string(),
            ParameterAttributes::Returned => "returned".to_string(),
            ParameterAttributes::NonNull => "nonnull".to_string(),
            ParameterAttributes::Dereferenceable(x) => format!("dereferenceable({})", x),
            ParameterAttributes::DereferenceableOrNull(x) => {
                format!("dereferenceable_or_null({})", x)
            }
            ParameterAttributes::SwiftSelf => "swiftself".to_string(),
            ParameterAttributes::SwiftError => "swifterror".to_string(),
            ParameterAttributes::ImmArg => "immarg".to_string(),
            ParameterAttributes::NoUndef => "noundef".to_string(),
        };

        write!(f, "{}", s)
    }
}
