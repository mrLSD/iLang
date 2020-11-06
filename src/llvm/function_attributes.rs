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
    NoInlineLineTables
}

/*
alignstack(i32)
allocsize(Vec<i32>)
alwaysinline
builtin
cold
convergent
inaccessiblememonly
inaccessiblemem_or_argmemonly
inlinehint
jumptable
minsize
naked
"no-inline-line-tables"

no-jump-tables
nobuiltin
noduplicate
nofree
noimplicitfloat
noinline
nomerge
nonlazybind
noredzone
indirect-tls-seg-refs
noreturn
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
                let Vec = x.iter()
                    .enumerate()
                    .fold("".to_string(), |x, (i, v)|{
                        
                    })
                format!("allocsize({})", x)
            },
            FunctionAttributesType::AlwaysInline => "",
            FunctionAttributesType::Builtin => "",
            FunctionAttributesType::Cold => "",
            FunctionAttributesType::Convergent => "",
            FunctionAttributesType::InaccessibleMemOnly => "",
            FunctionAttributesType::InaccessibleMemOrArgMemOnly => "",
            FunctionAttributesType::InlineHint => "",
            FunctionAttributesType::JumpTable => "",
            FunctionAttributesType::MinSize => "",

alwaysinline
builtin
cold
convergent
inaccessiblememonly
inaccessiblemem_or_argmemonly
inlinehint
jumptable
minsize
            
            FunctionAttributesType::Naked => "naked",
            FunctionAttributesType::NoInlineLineTables => "\"no-inline-line-tables\"",
            _ => "",
        };
        write!(f, "\"{}\"", s)
    }
}
