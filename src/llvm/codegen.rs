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
    instructions::terminator::Ret,
    linkage_types::LinkageTypes::{
        External,
        Private,
    },
    runtime_preemption::RuntimePreemptionSpecifier::DsoLocal,
    source_filename::SourceFileName,
    target_triple::TargetTriple,
    type_system::aggregate::ArrayType,
    types::Type,
    types::Type::*,
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

    let gty = Type::Array(ArrayType(10, b!(Integer8)));
    let mut g = global!(Constant gty ".str");
    global!(g.linkage @Private);
    global!(g.unnamed_addr @UnnamedAddr);
    global!(g.initializer_constant @r#"c"Hello: %d\00""#.to_string());

    let sf = source_file!(1.il);
    let tt = target_triple!(TARGET_X86_64_UNKNOWN_LINUX_GNU);

    let a1 = alloca!(Integer32 2);
    let store1 = store!(Integer32 "33", "%2");
    let load1 = load!(Integer32 "3", "%2");
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
    let s = ret!(Integer32 @"%3");
    println!("{}", s);
    let s = ret!();
    println!("{}", s);
}

#[cfg(test)]
mod test {
    use crate::llvm::codegen::main_fn;

    #[test]
    fn test_main_fn() {
        main_fn();
    }
}
