//! # Aggregate Operations
//!
//! LLVM supports several instructions for working with aggregate values
//!
//! https://llvm.org/docs/LangRef.html#aggregate-operations

use crate::llvm::types::Type;

/// The ‘extractvalue’ instruction extracts the value of a member
/// field from an aggregate value.
///
/// The first operand of an ‘extractvalue’ instruction is a value of
/// struct or array type. The other operands are constant indices to
/// specify which value to extract in a similar manner as indices in
/// a ‘getelementptr’ instruction.
/// The major differences to getelementptr indexing are:
/// * Since the value being indexed is not a pointer, the first index
/// is omitted and assumed to be zero.
/// * At least one index must be specified.
/// * Not only struct indices but also array indices must be in bounds.
///
/// The result is the value at the position in the aggregate specified
/// by the index operands.
///
/// https://llvm.org/docs/LangRef.html#extractvalue-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Extractvalue {
    pub result: String,
    pub aggregate_type: String,
    pub val: String,
    pub idx: Vec<u64>,
}

/// The ‘insertvalue’ instruction inserts a value into a member field
/// in an aggregate value.
///
/// The first operand of an ‘insertvalue’ instruction is a value of
/// struct or array type. The second operand is a first-class value
/// to insert. The following operands are constant indices indicating
/// the position at which to insert the value in a similar manner as
/// indices in a ‘extractvalue’ instruction. The value to insert must
/// have the same type as the value identified by the indices.
///
/// The result is an aggregate of the same type as val. Its value is
/// that of val except that the value at the position specified by
/// the indices is that of elt.
///
/// https://llvm.org/docs/LangRef.html#insertvalue-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Insertvalue {
    pub result: String,
    pub aggregate_type: String,
    pub val: String,
    pub ty: Type,
    pub elt: String,
    pub idx: Vec<u64>,
}

impl std::fmt::Display for Extractvalue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let idx = self
            .idx
            .iter()
            .fold("".to_string(), |s, v| format!("{}, {}", s, v));
        let s = format!(
            "{} = extractvalue {} {} {}",
            self.result, self.aggregate_type, self.val, idx
        );
        write!(f, "{}", s)
    }
}

impl std::fmt::Display for Insertvalue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let idx = self
            .idx
            .iter()
            .fold("".to_string(), |s, v| format!("{}, {}", s, v));
        let s = format!(
            "{} = insertvalue {} {}, {} {} {}",
            self.result, self.aggregate_type, self.val, self.ty, self.elt, idx
        );
        write!(f, "{}", s)
    }
}
