//! # Calling Conventions
//!
//! LLVM functions, calls and invokes can all have an optional calling
//! convention specified for the call. The calling convention of any
//! pair of dynamic caller/callee must match, or the behavior of the
//! program is undefined. The following calling conventions are
//! supported by LLVM, and more may be added in the future.
//!
//! https://llvm.org/docs/LangRef.html#callingconv

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum CallingConvention {
    CCc,
    FastCc,
    Cc10,
    Cc11,
    WebkitJsCc,
    AnyRegCc,
    PreserveMostCc,
    PreserveAllCc,
    CxxFastYlsCc,
    SwiftCc,
    YailCc,
    CFGuardCheckCc,
}

impl std::fmt::Display for CallingConvention {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            CallingConvention::CCc => "ccc",
            CallingConvention::FastCc => "fastcc",
            CallingConvention::Cc10 => "cc 10",
            CallingConvention::Cc11 => "cc 11",
            CallingConvention::WebkitJsCc => "webkit_jscc",
            CallingConvention::AnyRegCc => "anyregcc",
            CallingConvention::PreserveMostCc => "preserve_mostcc",
            CallingConvention::PreserveAllCc => "preserve_allcc",
            CallingConvention::CxxFastYlsCc => "cxx_fast_tlscc",
            CallingConvention::SwiftCc => "swiftcc",
            CallingConvention::YailCc => "tailcc",
            CallingConvention::CFGuardCheckCc => "cfguard_checkcc",
        };

        write!(f, "{}", s)
    }
}
