//! # Codegen

use crate::llvm::types::Type::Integer8;
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

pub fn main_fn() {
    let ty1 = Type::pointer2(Integer8);

    let mut f = def!(Integer32 main);
    def!(f.linkage @External);
    def!(f.preemption_specifier @DsoLocal);
    def!(f.argument_list arg!(Integer32 0, ty1 1));

    let ty1 = Type::pointer1(Integer8);
    let mut d = decl!(Integer32 printf);
    def!(d.argument_list arg!(ty1, ...));
    def!(d.preemption_specifier @DsoLocal);

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
    println!("{}\n{}\n{}\n{} {}\n{}", sf, tt, g, f, body, d);
    println!("==================");
    let sf = source_file!(1.il);
    let tt = target_triple!(TARGET_X86_64_UNKNOWN_LINUX_GNU);
    println!("{}", sf);
    println!("{}", tt);
}

#[cfg(test)]
mod test {
    use crate::llvm::codegen::main_fn;

    #[test]
    fn test_main_fn() {
        main_fn();
    }
}
