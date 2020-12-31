//! # Codegen
//!
//! COdegen based on syntax analyzer and LLVM code generation

use crate::llvm::instructions::other_operations::Call;
use crate::llvm::linkage_types::LinkageTypes::Internal;
use crate::llvm::types::Type::Void;
use crate::parser::ast::{
    Main,
    MainStatement,
};

pub type Result = std::result::Result<String, CodegenError>;

#[derive(Debug, Eq, PartialEq)]
pub enum CodegenError {
    ModuleNotFound,
}

#[allow(clippy::ptr_arg)]
pub fn fn_module(ast: &Main) -> Result {
    let src: String;
    if !ast.is_empty() {
        if let MainStatement::Module(m) = &ast[0] {
            let s = m.module_name.iter().enumerate().fold(
                "; ModuleID = '".to_string(),
                |s, (i, name)| {
                    if i > 0 {
                        format!("{}.{}", s, name)
                    } else {
                        format!("{}{}", s, name)
                    }
                },
            );
            let s = format!("{}'", s);
            let source_file = format!("{}.i", m.module_name[m.module_name.len() - 1]);
            let sf = source_file!(source_file);
            let tt = target_triple!(TARGET_X86_64_UNKNOWN_LINUX_GNU);
            src = merge!(s sf tt);
        } else {
            return Err(CodegenError::ModuleNotFound);
        }
    } else {
        return Err(CodegenError::ModuleNotFound);
    }
    Ok(src)
}

#[allow(clippy::ptr_arg)]
pub fn fn_global_let(ast: &Main) -> Result {
    let mut global_let_statement = 0;
    let let_src = ast.iter().fold("".to_string(), |s, v| {
        if let MainStatement::LetBinding(_l) = v {
            let name = format!("__global_let_init.{}", global_let_statement);
            let mut fn_def = def!(Void name);
            def!(fn_def.linkage @Internal);
            def!(fn_def.attr_group vec![0]);
            def!(fn_def.section_name @".text.startup".to_string());
            global_let_statement += 1;

            let ret = ret!();
            let body = body!(ret);
            merge!(s fn_def body)
        } else {
            s
        }
    });
    let mut src = let_src;
    if global_let_statement > 0 {
        let global_ctors = "@llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 65535, void ()* @_GLOBAL_let_main, i8* null }]".to_string();
        let name = "_GLOBAL_let_main";
        let mut fn_def = def!(Void name);
        def!(fn_def.linkage @Internal);
        def!(fn_def.attr_group vec![0]);
        def!(fn_def.section_name @".text.startup".to_string());

        // Set body of function with @call instr
        let mut call = vec![];
        for i in 0..global_let_statement {
            let name = format!("__global_let_init.{}", i);
            let call_fn = call!(Void => @name vec![] => []);
            call.push(call_fn);
        }
        let body = body!(@ call);
        src = merge!(src global_ctors fn_def body);
    }
    Ok(src)
}

pub fn fn_main(ast: Main) -> Result {
    let module = fn_module(&ast)?;
    let global_let = fn_global_let(&ast)?;
    let src = module!(module global_let);
    println!("\n{}", src);
    Ok(src)
}

#[cfg(test)]
mod tests {
    use crate::codegen::{
        fn_main,
        CodegenError,
    };
    use crate::parser::{
        ast::Span,
        token::main,
    };

    #[test]
    fn test_codegen_main_func_complex() {
        let x = main(Span::new("let val1 = 10")).unwrap();
        assert_eq!(x.0.fragment(), &"");
        let res = fn_main(x.1);
        assert_eq!(res.unwrap_err(), CodegenError::ModuleNotFound);

        let x = main(Span::new(
            "module name1.name2\nlet (x: i8) = 10\nlet y = x*2",
        ))
        .unwrap();
        assert_eq!(x.0.fragment(), &"");
        let res = fn_main(x.1);
        assert!(res.is_ok());
    }
}
