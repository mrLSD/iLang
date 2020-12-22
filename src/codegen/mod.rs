//! # Codegen
//!
//! COdegen based on syntax analyzer and LLVM code generation

use crate::parser::ast::{
    Main,
    MainStatement,
};

pub type Result = std::result::Result<(), CodegenError>;

#[derive(Debug, Eq, PartialEq)]
pub enum CodegenError {
    ModuleNotFound,
}

pub fn fn_module(ast: Main) -> Result {
    if !ast.is_empty() {
        if let MainStatement::Module(m) = &ast[0] {
            let _ = m;
        } else {
            return Err(CodegenError::ModuleNotFound);
        }
    }
    Ok(())
}

pub fn fn_main(ast: Main) -> Result {
    println!("{:#?}", ast);
    let _ = ast;
    fn_module(ast)
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

        let x = main(Span::new("module name1.name2\nlet val1: i8 = 10")).unwrap();
        assert_eq!(x.0.fragment(), &"");
        let res = fn_main(x.1);
        assert!(res.is_ok());
    }
}
