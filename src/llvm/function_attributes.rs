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
}

/*
norecurse
willreturn
nosync
nounwind
null_pointer_is_valid
optforfuzzing
optnone
optsize
patchable-function
probe-stack
readnone
readonly
stack-probe-size
no-stack-arg-probe
writeonly
argmemonly
returns_twice
safestack
sanitize_address
sanitize_memory
sanitize_thread
sanitize_hwaddress
sanitize_memtag
speculative_load_hardening
speculatable
ssp
sspreq
sspstrong
strictfp
denormal-fp-math-f32
thunk
uwtable
nocf_check
shadowcallstack
*/
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
