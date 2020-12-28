//! # Functions
//!
//! LLVM function definitions consist of the “define”
//! keyword, and an optional fields.
//!
//! LLVM function declarations consist of the “declare”
//! keyword, an optional fields.
//!
//! A function definition contains a list of basic blocks, forming the
//! CFG (Control Flow Graph) for the function. Each basic block may
//! optionally start with a label (giving the basic block a symbol table
//! entry), contains a list of instructions, and ends with a terminator
//! instruction (such as a branch or function return). If an explicit
//! label name is not provided, a block is assigned an implicit numbered
//! label, using the next value from the same counter as used for unnamed
//! temporaries (see above). For example, if a function entry block does
//! not have an explicit label, it will be assigned label “%0”, then the
//! first unnamed temporary in that block will be “%1”, etc. If a numeric
//! label is explicitly specified, it must match the numeric label that
//! would be used implicitly.
//!
//! The first basic block in a function is special in two ways: it is
//! immediately executed on entrance to the function, and it is not
//! allowed to have predecessor basic blocks (i.e. there can not be
//! any branches to the entry block of a function). Because the block
//! can have no predecessors, it also cannot have any PHI nodes.
//!
//! Syntax:
//! ```html
//! define [linkage] [PreemptionSpecifier] [visibility] [DLLStorageClass]
//!        [cconv] [ret attrs]
//!        <ResultType> @<FunctionName> ([argument list])
//!        [(unnamed_addr|local_unnamed_addr)] [AddrSpace] [fn Attrs]
//!        [section "name"] [comdat [($name)]] [align N] [gc] [prefix Constant]
//!        [prologue Constant] [personality Constant] (!name !N)* { ... }
//! ```
//!
//! The argument list is a comma separated sequence of arguments where
//! each argument is of the following form:
//!
//! Syntax:
//! ```html
//! <type> [parameter Attrs] [name]
//! ```
//! https://llvm.org/docs/LangRef.html#functions

use crate::llvm::{
    addrspace::AddrSpace,
    align::Alignment,
    attribute_groups::Personality,
    calling_convention::CallingConvention,
    comdat::ComDat,
    dll_storage_classes::DLLStorageClasses,
    function_attributes::FunctionAttributes,
    gc_stratagy_name::GCStrategyName,
    global_variables::UnnamedAddr,
    linkage_types::LinkageTypes,
    parameter_attributes::ParameterAttributes,
    prefix::Prefix,
    runtime_preemption::RuntimePreemptionSpecifier,
    types::Type,
    visibility_styles::VisibilityStyles,
};

/// The argument list is a comma separated sequence of arguments where
/// each argument is of the following form:
///
/// Syntax:
/// ```html
/// <type> [parameter Attrs] [name]
/// ```
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ArgumentList {
    pub parameter_type: Option<Type>,
    pub attributes: Option<ParameterAttributes>,
    pub name: Option<String>,
    pub variable_argument: bool,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum FunctionDefinitionType {
    Declare,
    Define,
}

/// Most commin unction specification
/// NOTE: prologue, metadata is simple strings without specific Rust types
/// personality field is not clear is it right or not
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Function {
    pub definition_type: FunctionDefinitionType,
    pub linkage: Option<LinkageTypes>,
    pub preemption_specifier: Option<RuntimePreemptionSpecifier>,
    pub visibility: Option<VisibilityStyles>,
    pub dll_storage_class: Option<DLLStorageClasses>,
    pub cconv: Option<CallingConvention>,
    pub ret_attrs: Option<ParameterAttributes>,
    pub result_type: Type,
    pub function_name: String,
    pub argument_list: Vec<ArgumentList>,
    pub unnamed_addr: Option<UnnamedAddr>,
    pub addr_sapce: Option<AddrSpace>,
    pub fn_attrs: Vec<FunctionAttributes>,
    pub attr_group: Vec<u64>,
    pub section_name: Option<String>,
    pub comdat: Option<ComDat>,
    pub align: Option<Alignment>,
    pub gc: Option<GCStrategyName>,
    pub prefix: Option<Prefix>,
    pub prologue: Option<String>,
    pub personality: Option<Personality>,
    pub metadata: Option<String>,
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = match self.definition_type {
            FunctionDefinitionType::Declare => "declare".to_string(),
            FunctionDefinitionType::Define => "define".to_string(),
        };
        if let Some(x) = &self.linkage {
            s = format!("{} {}", s, x);
        }
        if let Some(x) = &self.preemption_specifier {
            s = format!("{} {}", s, x);
        }
        if let Some(x) = &self.visibility {
            s = format!("{} {}", s, x);
        }
        if let Some(x) = &self.dll_storage_class {
            s = format!("{} {}", s, x);
        }
        if let Some(x) = &self.cconv {
            s = format!("{} {}", s, x);
        }
        if let Some(x) = &self.ret_attrs {
            s = format!("{} {}", s, x);
        }
        s = format!("{} {}", s, self.result_type);
        s = format!("{} @{}", s, self.function_name);
        let arg = self
            .argument_list
            .iter()
            .enumerate()
            .fold("".to_string(), |s, (i, x)| {
                if i == 0 {
                    format!("{}", x)
                } else {
                    format!("{}, {}", s, x)
                }
            });
        s = format!("{} ({})", s, arg);

        if let Some(x) = &self.unnamed_addr {
            s = format!("{} {}", s, x);
        }

        if let Some(x) = &self.addr_sapce {
            s = format!("{} {}", s, x);
        }

        let fn_attrs = self
            .fn_attrs
            .iter()
            .fold("".to_string(), |s, x| format!("{} {}", s, x));
        s = format!("{} {}", s, fn_attrs);

        let attr_group = self
            .attr_group
            .iter()
            .fold("".to_string(), |s, x| format!("{} #{}", s, x));
        s = format!("{} {}", s, attr_group);

        if let Some(x) = &self.section_name {
            s = format!("{} section \"{}\"", s, x);
        }
        if let Some(x) = &self.comdat {
            s = format!("{} comdat {}", s, x);
        }
        if let Some(x) = &self.align {
            s = format!("{} {}", s, x);
        }
        if let Some(x) = &self.gc {
            s = format!("{} {}", s, x);
        }
        if let Some(x) = &self.prefix {
            s = format!("{} {}", s, x);
        }
        if let Some(x) = &self.prologue {
            s = format!("{} prologue {}", s, x);
        }
        if let Some(x) = &self.personality {
            s = format!("{} personality {}", s, x);
        }
        if let Some(x) = &self.metadata {
            s = format!("{} {}", s, x);
        }

        write!(f, "{}", s)
    }
}

impl std::fmt::Display for ArgumentList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = "".to_string();
        if let Some(t) = &self.parameter_type {
            s = format!("{}", t);
        }
        if let Some(x) = &self.attributes {
            s = format!("{} {}", s, x);
        }
        if let Some(x) = &self.name {
            s = format!("{} {}", s, x);
        }
        if self.variable_argument {
            s = "...".to_string();
        }
        write!(f, "{}", s)
    }
}
