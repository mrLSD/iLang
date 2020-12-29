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
            let x = m
                .module_name
                .iter()
                .fold("".to_string(), |s, v| format!("{}.{}", s, v));
            src = format!("; {}.i", x);
        } else {
            return Err(CodegenError::ModuleNotFound);
        }
    } else {
        return Err(CodegenError::ModuleNotFound);
    }
    Ok(src)
}

#[allow(clippy::ptr_arg)]
pub fn fn_globa_let(ast: &Main) -> Result {
    let mut global_let_statement = 0;
    let _let_src = ast.iter().fold("".to_string(), |s, v| {
        if let MainStatement::LetBinding(l) = v {
            global_let_statement += 1;
            println!("# {:#?}", l);
        }
        s
    });
    let mut src = "".to_string();
    if global_let_statement > 0 {
        let global_ctors = "@llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 65535, void ()* @_GLOBAL_let_main, i8* null }]".to_string();
        let mut fn_def = def!(Void _GLOBAL_let_main);
        def!(fn_def.linkage @Internal);
        def!(fn_def.attr_group vec![0]);
        def!(fn_def.section_name @".text.startup".to_string());

        // Set body of function with @call instr
        let mut call = vec![];
        for i in 0..global_let_statement {
            let name = format!("__global_let_init.{}", i);
            let call_fn = call!(Void => @name vec![] => []);
            call.push(call_fn);
            //src = format!("{}\n\tcall void @__global_let_init.{}()", src, i);
        }
        //let  s = Call{}
        let body = body!(@ call);
        src = merge!(global_ctors fn_def body);
    }
    Ok(src)
}

pub fn fn_main(ast: Main) -> Result {
    //println!("{:#?}", ast);
    let _ = ast;
    let module = fn_module(&ast)?;
    let globa_let = fn_globa_let(&ast)?;
    let src = module!(module globa_let);
    println!("\n#main {}", src);
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

        let x = main(Span::new("module name1.name2\nlet (val1: i8) = 10")).unwrap();
        assert_eq!(x.0.fragment(), &"");
        let res = fn_main(x.1);
        assert!(res.is_ok());
    }
}
