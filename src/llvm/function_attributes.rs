//! # Function Attributes
//!
//! Function attributes are set to communicate additional information
//! about a function. Function attributes are considered to be part of
//! the function, not of the function type, so functions with different
//! function attributes can have the same function type.
//!
//! Function attributes are simple keywords that follow the type
//! specified. If multiple attributes are needed, they are space
//! separated.
//!
//! https://llvm.org/docs/LangRef.html#function-attributes

// TODO: complete FunctionAttributes
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FunctionAttributes(Vec<FunctionAttributesType>);

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum FunctionAttributesType {
    AlignStack(i32),
    AllocSize(Vec<i32>),
    AlwaysInline,
    Builtin,
    Cold,
    Convergent,
    InaccessibleMemOnly,
    InaccessibleMemOrArgMemOnly,
    InlineHint,
    JumpTable,
    MinSize,
    Naked,
    NoInlineLineTables,

    NoJumpTables,
    NoBuiltin,
    NoDuplicate,
    NoFree,
    NoImplicitFloat,
    NoInline,
    NoMerge,
    NonLazyBind,
    NoRedZone,
    IndirectTlsSegRefs,
    NoReturn,
    NoRecurse,
    WillReturn,
    NoSync,
    NoUnwind,
    NullPointerIsValid,
    OptForFuzzing,
    OptNone,
    OptSize,
    PatchableFunction,
    ProbeStack,
    ReadNone,
    ReadOnly,
    StackProbeSize,
    NoStackArgProbe,
    WriteOnly,
    ArgMemOnly,
    ReturnsTwice,
    SafeStack,
    SanitizeAddress,
    SanitizeMemory,
    SanitizeThread,
    SanitizeNwAddress,
    SanitizeMemTag,
    SpeculativeLoadNardening,
    Speculatable,
    Ssp,
    SspReq,
    SspStrong,
    StrictFp,
    DenormalFpMathF32,
    Thunk,
    UwTable,
    NocfCheck,
    ShadowCallStack,
}

impl std::fmt::Display for FunctionAttributesType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            FunctionAttributesType::AlignStack(x) => format!("alignstack({})", x),
            FunctionAttributesType::AllocSize(x) => {
                let s = x.iter().enumerate().fold("".to_string(), |x, (i, v)| {
                    if i > 1 {
                        format!("{}, {}", x, v)
                    } else {
                        format!("{} {}", x, v)
                    }
                });
                format!("allocsize({})", s)
            }
            FunctionAttributesType::AlwaysInline => "alwaysinline".to_string(),
            FunctionAttributesType::Builtin => "builtin".to_string(),
            FunctionAttributesType::Cold => "cold".to_string(),
            FunctionAttributesType::Convergent => "convergent".to_string(),
            FunctionAttributesType::InaccessibleMemOnly => "inaccessiblememonly".to_string(),
            FunctionAttributesType::InaccessibleMemOrArgMemOnly => {
                "inaccessiblemem_or_argmemonly".to_string()
            }
            FunctionAttributesType::InlineHint => "inlinehint".to_string(),
            FunctionAttributesType::JumpTable => "jumptable".to_string(),
            FunctionAttributesType::MinSize => "minsize".to_string(),
            FunctionAttributesType::Naked => "naked".to_string(),
            FunctionAttributesType::NoInlineLineTables => "\"no-inline-line-tables\"".to_string(),
            FunctionAttributesType::NoJumpTables => "no-jump-tables".to_string(),
            FunctionAttributesType::NoBuiltin => "nobuiltin".to_string(),
            FunctionAttributesType::NoDuplicate => "noduplicate".to_string(),
            FunctionAttributesType::NoFree => "nofree".to_string(),
            FunctionAttributesType::NoImplicitFloat => "noimplicitfloat".to_string(),
            FunctionAttributesType::NoInline => "noinline".to_string(),
            FunctionAttributesType::NoMerge => "nomerge".to_string(),
            FunctionAttributesType::NonLazyBind => "nonlazybind".to_string(),
            FunctionAttributesType::NoRedZone => "noredzone".to_string(),
            FunctionAttributesType::IndirectTlsSegRefs => "indirect-tls-seg-refs".to_string(),
            FunctionAttributesType::NoReturn => "noreturn".to_string(),

            FunctionAttributesType::NoRecurse => "norecurse".to_string(),
            FunctionAttributesType::WillReturn => "willreturn".to_string(),
            FunctionAttributesType::NoSync => "nosync".to_string(),
            FunctionAttributesType::NoUnwind => "nounwind".to_string(),
            FunctionAttributesType::NullPointerIsValid => "null_pointer_is_valid".to_string(),
            FunctionAttributesType::OptForFuzzing => "optforfuzzing".to_string(),
            FunctionAttributesType::OptNone => "optnone".to_string(),
            FunctionAttributesType::OptSize => "optsize".to_string(),
            FunctionAttributesType::PatchableFunction => "patchable-function".to_string(),
            FunctionAttributesType::ProbeStack => "probe-stack".to_string(),
            FunctionAttributesType::ReadNone => "readnone".to_string(),
            FunctionAttributesType::ReadOnly => "readonly".to_string(),
            FunctionAttributesType::StackProbeSize => "stack-probe-size".to_string(),
            FunctionAttributesType::NoStackArgProbe => "no-stack-arg-probe".to_string(),
            FunctionAttributesType::WriteOnly => "writeonly".to_string(),
            FunctionAttributesType::ArgMemOnly => "argmemonly".to_string(),
            FunctionAttributesType::ReturnsTwice => "returns_twice".to_string(),
            FunctionAttributesType::SafeStack => "safestack".to_string(),
            FunctionAttributesType::SanitizeAddress => "sanitize_address".to_string(),
            FunctionAttributesType::SanitizeMemory => "sanitize_memory".to_string(),
            FunctionAttributesType::SanitizeThread => "sanitize_thread".to_string(),
            FunctionAttributesType::SanitizeNwAddress => "sanitize_hwaddress".to_string(),
            FunctionAttributesType::SanitizeMemTag => "sanitize_memtag".to_string(),
            FunctionAttributesType::SpeculativeLoadNardening => {
                "speculative_load_hardening".to_string()
            }
            FunctionAttributesType::Speculatable => "speculatable".to_string(),
            FunctionAttributesType::Ssp => "ssp".to_string(),
            FunctionAttributesType::SspReq => "sspreq".to_string(),
            FunctionAttributesType::SspStrong => "sspstrong".to_string(),
            FunctionAttributesType::StrictFp => "strictfp".to_string(),
            FunctionAttributesType::DenormalFpMathF32 => "denormal-fp-math-f32".to_string(),
            FunctionAttributesType::Thunk => "thunk".to_string(),
            FunctionAttributesType::UwTable => "uwtable".to_string(),
            FunctionAttributesType::NocfCheck => "nocf_check".to_string(),
            FunctionAttributesType::ShadowCallStack => "shadowcallstack".to_string(),
        };
        write!(f, "\"{}\"", s)
    }
}

impl std::fmt::Display for FunctionAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = self
            .0
            .iter()
            .fold("".to_string(), |s, v| format!("{} {}", s, v));
        write!(f, "\"{}\"", s)
    }
}
