use std::fmt::{
    Debug,
    Display,
};

#[macro_use]
pub mod macros;

pub mod addrspace;
pub mod aliases;
pub mod align;
pub mod attribute_groups;
pub mod calling_convention;
pub mod codegen;
pub mod comdat;
pub mod context;
pub mod data_layout;
pub mod dll_storage_classes;
pub mod fast_math_flags;
pub mod function_attributes;
pub mod functions;
pub mod gc_stratagy_name;
pub mod global_variables;
pub mod ifunc;
pub mod instructions;
pub mod linkage_types;
pub mod module_inline_asm;
pub mod parameter_attributes;
pub mod prefix;
pub mod runtime_preemption;
pub mod section;
pub mod source_filename;
pub mod target_triple;
pub mod thread_local_storage;
pub mod type_system;
pub mod types;
pub mod visibility_styles;

pub trait InstructionSet: Debug + Display {
    /// Set context of input values (it mean increment flow of
    /// context values)
    fn set_context(&mut self, ctx: u64);
    /// Is context flow read only. So it mean previos context
    /// should not be changed/incremented
    fn is_read_only_context(&self) -> bool {
        false
    }
    /// For current instruction applicable assignment for value
    fn is_assignment(&self) -> bool {
        false
    }
    /// Is it global value
    fn is_global(&self) -> bool {
        false
    }
}
