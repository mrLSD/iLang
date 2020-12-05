//! #Conversion Operations¶
//!
//! The instructions in this category are the conversion instructions
//! (casting) which all take a single operand and a type. They perform
//! various bit conversions on the operand.
//!
//! https://llvm.org/docs/LangRef.html#conversion-operations

use crate::llvm::types::Type;

/// The ‘trunc’ instruction truncates its operand to the type ty2.
///
/// The ‘trunc’ instruction takes a value to trunc, and a type to
/// trunc it to. Both types must be of integer types, or vectors of
/// the same number of integers. The bit size of the value must be
/// larger than the bit size of the destination type, ty2. Equal sized
/// types are not allowed.
/// The ‘trunc’ instruction truncates the high order bits in value and
/// converts the remaining bits to ty2. Since the source size must be
/// larger than the destination size, trunc cannot be a no-op cast.
/// It will always truncate bits.
///
/// https://llvm.org/docs/LangRef.html#trunc-to-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Trunc {
    pub result: String,
    pub ty: Type,
    pub value: String,
    pub ty2: Type,
}

impl std::fmt::Display for Trunc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = format!(
            "{} = trunc {} {} to {}",
            self.result, self.ty, self.value, self.ty2
        );
        write!(f, "{}", s)
    }
}
