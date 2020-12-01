//! Memory Access and Addressing Operations
//!
//! A key design point of an SSA-based representation is how it represents
//! memory. In LLVM, no memory locations are in SSA form, which makes
//! things very simple. This section describes how to read, write, and
//! allocate memory in LLVM.
//!
//! https://llvm.org/docs/LangRef.html#memory-access-and-addressing-operations

use crate::llvm::addrspace::AddrSpace;
use crate::llvm::{
    align::Alignment,
    types::Type,
};

/// The ‘alloca’ instruction allocates memory on the stack frame of the
/// currently executing function, to be automatically released when
/// this function returns to its caller. The object is always allocated
/// in the address space for allocas indicated in the datalayout.
///
/// The ‘alloca’ instruction allocates sizeof(<type>)*NumElements
/// bytes of memory on the runtime stack, returning a pointer of the
/// appropriate type to the program. If “NumElements” is specified, it
/// is the number of elements allocated, otherwise “NumElements” is
/// defaulted to be one. If a constant alignment is specified, the
/// value result of the allocation is guaranteed to be aligned to at
/// least that boundary. The alignment may not be greater than 1 << 29.
/// If not specified, or if zero, the target can choose to align the
/// allocation on any convenient boundary compatible with the type.
/// ‘type’ may be any sized type.
///
/// Memory is allocated; a pointer is returned. The allocated memory
/// is uninitialized, and loading from uninitialized memory produces
/// an undefined value. The operation itself is undefined if there is
/// insufficient stack space for the allocation.’alloca’d memory is
/// automatically released when the function returns. The ‘alloca’
/// instruction is commonly used to represent automatic variables that
/// must have an address available. When the function returns (either
/// with the ret or resume instructions), the memory is reclaimed.
/// Allocating zero bytes is legal, but the returned pointer may not
/// be unique. The order in which memory is allocated (ie., which way
/// the stack grows) is not specified.
///
/// https://llvm.org/docs/LangRef.html#alloca-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Alloca {
    pub result: String,
    pub alloc_ty: Type,
    pub elements: Option<Vec<(Type, i64)>>,
    pub align: Option<Alignment>,
    pub addrspace: Option<AddrSpace>,
}

impl std::fmt::Display for Alloca {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = format!("{} = alloca {}", self.result, self.alloc_ty);
        if let Some(el) = &self.elements {
            let els = el
                .iter()
                .fold("".to_string(), |s, v| format!("{}, {} {}", s, v.0, v.1));
            s = format!("{} {}", s, els);
        }
        if let Some(v) = &self.align {
            s = format!("{}, {}", s, v);
        }
        if let Some(v) = &self.addrspace {
            s = format!("{}, {}", s, v);
        }
        write!(f, "{}", s)
    }
}
