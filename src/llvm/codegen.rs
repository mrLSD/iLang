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
        GetElementPtr,
        Load,
        Store,
    },
    instructions::other_operations::Call,
    instructions::terminator::FunctionArg,
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
    decl!(d.argument_list arg!(ty1, ...));
    decl!(d.preemption_specifier @DsoLocal);

    let gty = Array(ArrayType(10, b!(Integer8)));
    let mut g = global!(Constant gty ".str");
    global!(g.linkage @Private);
    global!(g.unnamed_addr @UnnamedAddr);
    global!(g.initializer_constant @r#"c"Hello: %d\00""#.to_string());

    let sf = source_file!(1.il);
    let tt = target_triple!(TARGET_X86_64_UNKNOWN_LINUX_GNU);

    let a1 = alloca!(Integer32 3);
    let store1 = store!(Integer32 "33", "%3");
    let load1 = load!(Integer32 "4", "%3");
    let gty = Array(ArrayType(10, b!(Integer8)));
    let ge = getelementptr!(gty inbounds "el", "@.str" => [Integer64 0, Integer64 0]);

    let ty2 = Type::pointer1(Integer8);
    let ty3 = Type::pointer1(Integer8);
    let call1 = Call {
        ret_val: "5".to_string(),
        tail: None,
        fast_math_flags: None,
        cconv: None,
        ret_attr: None,
        addrspace: None,
        ty: Type::Integer32,
        fnty: arg!(ty2, ...),
        fnptrval: (false, "printf".to_string()),
        function_args: vec![
            FunctionArg(ty3, "%el".to_string()),
            FunctionArg(Integer32, "%4".to_string()),
        ],
        function_attrs: None,
        operand_bundles: None,
    };
    let ret1 = ret!(Integer32 @0);
    let body = format!(
        "{{\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n}}",
        a1, store1, load1, ge, call1, ret1
    );

    println!("==================");
    println!("{}\n{}\n{}\n{} {}\n{}", sf, tt, g, f, body, d);
    println!("==================");
}

#[cfg(test)]
mod test {
    use crate::llvm::codegen::main_fn;

    #[test]
    fn test_main_fn() {
        main_fn();
    }
}
