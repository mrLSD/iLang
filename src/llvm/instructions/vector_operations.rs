//! # Vector Operations
//!
//! LLVM supports several instructions to represent vector operations in a
//! target-independent manner. These instructions cover the element-access
//! and vector-specific operations needed to process vectors effectively.
//! While LLVM does directly support these vector operations, many
//! sophisticated algorithms will want to use target-specific intrinsics
//! to take full advantage of a specific target.
//!
//! https://llvm.org/docs/LangRef.html#vector-operations
