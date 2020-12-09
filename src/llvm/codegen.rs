//! # Codegen

use crate::llvm::global_variables::UnnamedAddr::UnnamedAddr;
use crate::llvm::global_variables::{
    GlobalVariable,
    GlobalVariableKind,
};
use crate::llvm::{
    functions::{
        ArgumentList,
        Function,
        FunctionDefinitionType,
    },
    linkage_types::LinkageTypes,
    runtime_preemption::RuntimePreemptionSpecifier,
    types::Type,
};

pub fn main_fn() {
    let f = Function {
        definition_type: FunctionDefinitionType::Define,
        linkage: Some(LinkageTypes::External),
        preemption_specifier: Some(RuntimePreemptionSpecifier::DsoLocal),
        visibility: None,
        dll_storage_class: None,
        cconv: None,
        ret_attrs: None,
        result_type: Type::Integer32,
        function_name: "main".to_string(),
        argument_list: vec![
            ArgumentList {
                parameter_type: Type::Integer32,
                attributes: None,
                name: Some("%0".to_string()),
            },
            ArgumentList {
                parameter_type: Type::pointer2(Type::Integer32),
                attributes: None,
                name: Some("%1".to_string()),
            },
        ],
        unnamed_addr: None,
        addr_sapce: None,
        fn_attrs: vec![],
        section_name: None,
        comdat: None,
        align: None,
        gc: None,
        prefix: None,
        prologue: None,
        personality: None,
        metadata: None,
    };

    let g = GlobalVariable {
        name: ".str".to_string(),
        linkage: Some(LinkageTypes::Private),
        preemption_specifier: None,
        visibility: None,
        dll_storage_classes: None,
        thread_local: None,
        unnamed_addr: Some(UnnamedAddr),
        addrspace: None,
        global_variable_kind: GlobalVariableKind::Constant,
        value_type: Type::Integer16,
        initializer_constant: Some(r#"c"Hello: %d\00""#.to_string()),
        section: None,
        comdat: None,
        alignment: None,
        metadata: None,
    };
    println!("{}", g);
    println!("\n{} {{\n}}\n", f)
}

#[cfg(test)]
mod test {
    use crate::llvm::codegen::main_fn;

    #[test]
    fn test_main_fn() {
        main_fn();
    }
}
