//! # Codegen
#![cfg(nightly)]
#![allow(clippy::vec_init_then_push)]

use crate::llvm::context::Context;
use crate::llvm::{
    functions::{
        ArgumentList,
        FunctionDefinitionType,
    },
    global_variables::UnnamedAddr::UnnamedAddr,
    instructions::memory_access_addressing_operations::{
        GetElementPtr,
        Load,
    },
    instructions::other_operations::Call,
    instructions::terminator::FunctionArg,
    linkage_types::LinkageTypes::{
        External,
        Private,
    },
    runtime_preemption::RuntimePreemptionSpecifier::DsoLocal,
    type_system::aggregate::ArrayType,
    types::Type,
    types::Type::*,
};

pub fn main_fn() {
    let mut ctx = Context::new();
    let ty1 = Type::pointer2(Integer8);

    let name = "main";
    let mut f = def!(Integer32 name);
    def!(f.linkage @External);
    def!(f.preemption_specifier @DsoLocal);
    def!(f.argument_list arg!(Integer32 ctx.get(), ty1 ctx.inc().get()));

    let ty1 = Type::pointer1(Integer8);
    let name = "printf";
    let mut d = decl!(Integer32 name);
    decl!(d.argument_list arg!(ty1, ...));
    decl!(d.preemption_specifier @DsoLocal);

    let gty = Array(ArrayType(11, b!(Integer8)));
    let mut g = global!(Constant gty ".str");
    global!(g.linkage @Private);
    global!(g.unnamed_addr @UnnamedAddr);
    global!(g.initializer_constant @r#"c"Hello: %d\0A\00""#.to_string());

    let sf = source_file!("1.il");
    let tt = target_triple!(TARGET_X86_64_UNKNOWN_LINUX_GNU);

    let a1 = alloca!(Integer32 ctx.inc().get());
    let v = format!("%{}", ctx.get());
    let store1 = store!(Integer32 "33", v);
    let vload = ctx.inc();
    let load1 = load!(Integer32 vload.get(), v);
    let gty = Array(ArrayType(11, b!(Integer8)));
    let valptr = format!("%{}", ctx.inc().get());
    let ge = getelementptr!(gty inbounds valptr.get(), "@.str" => [Integer64 0, Integer64 0]);

    let ty2 = Type::pointer1(Integer8);
    let ty3 = Type::pointer1(Integer8);
    let printf_call = "printf_call";
    let valptr = format!("%{}", valptr.get());
    let vload = format!("%{}", vload.get());
    let call1 = call!(Integer32 ctx.inc().get() => @printf_call arg!(ty2, ...) => [ty3 valptr, Integer32 vload]);
    let ret1 = ret!(Integer32 @0);
    let entry1 = entry!(0);
    let body = body!(entry1 a1 store1 load1 ge call1 ret1);

    println!("==================");
    let module = module!(sf tt g f body d);
    println!("{}", module);
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
