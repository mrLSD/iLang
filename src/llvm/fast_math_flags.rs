/// # Fast-Math Flags
///
/// LLVM IR floating-point operations (fneg, fadd, fsub, fmul, fdiv, frem, fcmp), phi, select and call may use the following flags to enable otherwise unsafe floating-point transformations.
/// https://llvm.org/docs/LangRef.html#fast-math-flags

/// nnan - No NaNs - Allow optimizations to assume the arguments and result are not NaN. If an argument is a nan, or the result would be a nan, it produces a poison value instead.
/// ninf - No Infs - Allow optimizations to assume the arguments and result are not +/-Inf. If an argument is +/-Inf, or the result would be +/-Inf, it produces a poison value instead.
/// nsz - No Signed Zeros - Allow optimizations to treat the sign of a zero argument or result as insignificant. This does not imply that -0.0 is poison and/or guaranteed to not exist in the operation.
/// arcp - Allow Reciprocal - Allow optimizations to use the reciprocal of an argument rather than perform division.
/// contract - Allow floating-point contraction (e.g. fusing a multiply followed by an addition into a fused multiply-and-add). This does not enable reassociating to form arbitrary contractions. For example, (a*b) + (c*d) + e can not be transformed into (a*b) + ((c*d) + e) to create two fma operations.
/// afn - Approximate functions - Allow substitution of approximate calculations for functions (sin, log, sqrt, etc). See floating-point intrinsic definitions for places where this can apply to LLVM’s intrinsic math functions.
/// reassoc - Allow reassociation transformations for floating-point instructions. This may dramatically change results in floating-point.
/// fast - This flag implies all of the others.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum FastMathFlags {
    Nnan,
    Ninf,
    Nsz,
    Arcp,
    Contract,
    Afn,
    Reassoc,
    Fast,
}

impl std::fmt::Display for FastMathFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            FastMathFlags::Nnan => "nnan",
            FastMathFlags::Ninf => "ninf",
            FastMathFlags::Nsz => "nsz",
            FastMathFlags::Arcp => "arcp",
            FastMathFlags::Contract => "contract",
            FastMathFlags::Afn => "afn",
            FastMathFlags::Reassoc => "reassoc",
            FastMathFlags::Fast => "fast",
        };
        write!(f, "{}", s)
    }
}
