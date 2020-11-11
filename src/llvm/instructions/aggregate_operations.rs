//! # Aggregate Operations
//!
//! LLVM supports several instructions for working with aggregate values
//!
//! https://llvm.org/docs/LangRef.html#aggregate-operations

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
    pub aggregate_type: String,
    pub val: String,
    pub idx: Vec<u64>,
}

impl std::fmt::Display for Extractvalue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let idx = self
            .idx
            .iter()
            .fold("".to_string(), |s, v| format!("{}, {}", s, v));
        let s = format!("extractvalue {} {} {}", self.aggregate_type, self.val, idx);
        write!(f, "{}", s)
    }
}
