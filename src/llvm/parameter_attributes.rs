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

use nom::lib::std::fmt::Display;

#[allow(dead_code)]
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

impl<T> std::fmt::Display for ParameterAttributes<T> 
where
	T: Display
{
	fn fmt(&self, f: &mut std::fmt::Formatter) ->  std::fmt::Result {
		let s = match self {
			ParameterAttributes::ZeroExt => "zeroext".to_string(),
			ParameterAttributes::SignExt => "signext".to_string(),
			ParameterAttributes::InReg => "inreg".to_string(),
			ParameterAttributes::ByVal => "byval".to_string(),
			ParameterAttributes::ByRef(x) => format!("byref({})", x),
			ParameterAttributes::PreAllocated(_) => "".to_string(),
			ParameterAttributes::InAlloca => "".to_string(),
			ParameterAttributes::Sret => "".to_string(),
			ParameterAttributes::Allign(_) => "".to_string(),
			ParameterAttributes::NoAlias => "".to_string(),
			ParameterAttributes::NoCapture => "".to_string(),
			ParameterAttributes::NoFree => "".to_string(),
			ParameterAttributes::Nest => "".to_string(),
			ParameterAttributes::Returned => "".to_string(),
			ParameterAttributes::NonNull => "".to_string(),
			ParameterAttributes::Dereferenceable(_) => "".to_string(),
			ParameterAttributes::DereferenceableOrNull(_) => "".to_string(),
			ParameterAttributes::SwiftSelf => "".to_string(),
			ParameterAttributes::SwiftError => "".to_string(),
			ParameterAttributes::ImmArg => "".to_string(),
			ParameterAttributes::NoUndef => "".to_string(),
		};

		write!(f, "{}", s)
	}
}
