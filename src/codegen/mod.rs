//! # Codegen
//!
//! COdegen based on syntax analyzer and LLVM code generation

use crate::parser::ast::{
    Main,
    MainStatement,
};
use crate::llvm::linkage_types::LinkageTypes::Internal;

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
    let let_src = ast.iter().fold("".to_string(), |s, v| {
        if let MainStatement::LetBinding(l) = v {
            global_let_statement += 1;
            println!("# {:#?}", l);
        }
        s
    });
    let mut src = "".to_string();
    if global_let_statement > 0 {
        src = "@llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 65535, void ()* @_GLOBAL_let_main, i8* null }]".to_string();
        let mut f = def!(Void _GLOBAL_let_main);
        def!(f.linkage @Internal);
        def!(f.fn_attrs @"#0".to_string());
        def!(f.section_name @".text.startup".to_string());
        src = format!(
            "{}\ndefine internal void @_GLOBAL_let_main() #0 section \".text.startup\" {{",
            src
        );
        // Set body of function with @call instr
        for i in 0..global_let_statement {
            src = format!("{}\n\tcall void @__global_let_init.{}()", src, i);
        }
        src = format!("{}\n}}\n{}", src, let_src);
    }
    Ok(src)
}

pub fn fn_main(ast: Main) -> Result {
    //println!("{:#?}", ast);
    let _ = ast;
    fn_module(&ast)?;
    fn_globa_let(&ast)
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
