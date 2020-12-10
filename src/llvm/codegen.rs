//! # Codegen

use crate::llvm::global_variables::UnnamedAddr::UnnamedAddr;
use crate::llvm::global_variables::{
    GlobalVariable,
    GlobalVariableKind,
};
use crate::llvm::source_filename::SourceFileName;
use crate::llvm::target_triple::{
    TargetTriple,
    TARGET_X86_64_UNKNOWN_LINUX_GNU,
};
use crate::llvm::type_system::aggregate::ArrayType;
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
        linkage: None,
        preemption_specifier: None,
        visibility: None,
        dll_storage_classes: None,
        thread_local: None,
        unnamed_addr: Some(UnnamedAddr),
        addrspace: None,
        global_variable_kind: GlobalVariableKind::Constant,
        value_type: Type::Array(ArrayType(10, Box::new(Type::Integer8))),
        initializer_constant: Some(r#"c"Hello: %d\00""#.to_string()),
        section: None,
        comdat: None,
        alignment: None,
        metadata: None,
    };

    let sf = SourceFileName("main.il".to_string());
    let tt = TargetTriple(TARGET_X86_64_UNKNOWN_LINUX_GNU.to_string());

    let f1 = Function {
        definition_type: FunctionDefinitionType::Declare,
        linkage: None,
        preemption_specifier: Some(RuntimePreemptionSpecifier::DsoLocal),
        visibility: None,
        dll_storage_class: None,
        cconv: None,
        ret_attrs: None,
        result_type: Type::Integer32,
        function_name: "printf".to_string(),
        argument_list: vec![
            ArgumentList {
                parameter_type: Type::pointer1(Type::Integer8),
                attributes: None,
                name: None,
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

    println!("{}", sf);
    println!("{}", tt);
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
