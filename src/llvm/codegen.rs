//! # Codegen

use crate::llvm::{
    functions::{
        ArgumentList,
        Function,
        FunctionDefinitionType,
    },
    global_variables::{
        GlobalVariable,
        GlobalVariableKind,
        UnnamedAddr::UnnamedAddr,
    },
    instructions::memory_access_addressing_operations::{
        Alloca,
        Load,
        Store,
    },
    instructions::other_operations::Call,
    linkage_types::LinkageTypes::External,
    runtime_preemption::RuntimePreemptionSpecifier,
    runtime_preemption::RuntimePreemptionSpecifier::DsoLocal,
    source_filename::SourceFileName,
    target_triple::{
        TargetTriple,
        TARGET_X86_64_UNKNOWN_LINUX_GNU,
    },
    type_system::aggregate::ArrayType,
    types::Type,
    types::Type::Integer32,
};

macro_rules! def {
    ($fnval:ident.$attr:ident $val:expr) => {{
        $fnval.$attr = $val;
        $fnval
    }};
    ($fnval:ident.$attr:ident @ $val:expr) => {{
        $fnval.$attr = Some($val);
        $fnval
    }};
    ($ty:ident $name:ident) => {{
        Function {
            definition_type: FunctionDefinitionType::Define,
            linkage: None,
            preemption_specifier: None,
            visibility: None,
            dll_storage_class: None,
            cconv: None,
            ret_attrs: None,
            result_type: Type::$ty,
            function_name: stringify!($name).to_string(),
            argument_list: vec![
                ArgumentList {
                    parameter_type: Some(Type::Integer32),
                    attributes: None,
                    name: Some("%0".to_string()),
                    variable_argument: false,
                },
                ArgumentList {
                    parameter_type: Some(Type::pointer2(Type::Integer32)),
                    attributes: None,
                    name: Some("%1".to_string()),
                    variable_argument: false,
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
        }
    }};
}

pub fn main_fn() {
    let f = Function {
        definition_type: FunctionDefinitionType::Define,
        linkage: Some(External),
        preemption_specifier: Some(RuntimePreemptionSpecifier::DsoLocal),
        visibility: None,
        dll_storage_class: None,
        cconv: None,
        ret_attrs: None,
        result_type: Type::Integer32,
        function_name: "main".to_string(),
        argument_list: vec![
            ArgumentList {
                parameter_type: Some(Type::Integer32),
                attributes: None,
                name: Some("%0".to_string()),
                variable_argument: false,
            },
            ArgumentList {
                parameter_type: Some(Type::pointer2(Type::Integer32)),
                attributes: None,
                name: Some("%1".to_string()),
                variable_argument: false,
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
                parameter_type: Some(Type::pointer1(Type::Integer8)),
                attributes: None,
                name: None,
                variable_argument: false,
            },
            ArgumentList {
                parameter_type: None,
                attributes: None,
                name: Some("%1".to_string()),
                variable_argument: true,
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

    let a1 = alloca!(Integer32 2);
    let store1 = Store {
        volatile: None,
        ty: Type::Integer32,
        value: "33".to_string(),
        ty_pointer: Type::Integer32,
        pointer: "%2".to_string(),
        align: None,
    };
    let load1 = Load {
        result: "%3".to_string(),
        volatile: None,
        ty: Type::Integer32,
        ty_pointer: Type::Integer32,
        pointer: "%2".to_string(),
        align: None,
    };
    let call1 = Call {
        ret_val: "%3".to_string(),
        tail: None,
        fast_math_flags: None,
        cconv: None,
        ret_attr: None,
        addrspace: None,
        ty: Type::Integer32,
        fnty: None,
        fnptrval: (false, "printf".to_string()),
        function_args: vec![],
        function_attrs: None,
        operand_bundles: None,
    };

    let body = format!("{{\n\t{}\n\t{}\n\t{}\n\t{}\n}}", a1, store1, load1, call1);

    println!("==================");
    println!("{}\n{}\n{}\n{} {}\n{}", sf, tt, g, f, body, f1);
    println!("==================");

    let a1 = alloca!(Integer32 1);
    println!("{}", a1);
    let a1 = alloca!(Integer32 v1, 4);
    println!("{}", a1);

    let mut f2 = def!(Integer32 main);
    let mut f2 = def!(f2.linkage @External);
    let f2 = def!(f2.preemption_specifier @DsoLocal);
    println!("\t#{}", f2);
}

#[cfg(test)]
mod test {
    use crate::llvm::codegen::main_fn;

    #[test]
    fn test_main_fn() {
        main_fn();
    }
}
