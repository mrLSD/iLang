//! # Codegen
//!
//! COdegen based on syntax analyzer and LLVM code generation

use crate::llvm::attribute_groups::Attributes;
use crate::llvm::context::Context;
use crate::llvm::instructions::other_operations::Call;
use crate::llvm::linkage_types::LinkageTypes::Internal;
use crate::llvm::runtime_preemption::RuntimePreemptionSpecifier::DsoLocal;
use crate::llvm::types::Type::{
    Integer32,
    Void,
};
use crate::parser::ast::{
    BasicTypeExpression,
    ExpressionFunctionValueCall,
    FunctionBody,
    FunctionBodyStatement,
    FunctionCall,
    FunctionValue,
    Main,
    MainStatement,
    ParameterValueList,
    ParameterValueType,
    TypeExpression,
    ValueExpression,
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
pub fn expression(_ast: &Main) -> Result {
    let src = "".to_string();
    Ok(src)
}

pub fn type_expression(ctx: &mut Context, te: &TypeExpression) -> String {
    println!("TypeExpression: {:#?}", te.expr);
    match te.expr {
        BasicTypeExpression::Number(n) => {
            let a = alloca!(Integer32 ctx.get());
            let _n_val = ctx.val();
            let s = store!(Integer32 n, ctx.val());
            ctx.inc();
            bf!(a s)
        }
        _ => unimplemented!(),
    }
}

pub fn value_expression(ctx: &mut Context, vle: &ValueExpression) -> String {
    println!("ValueExpression");
    match vle {
        ValueExpression::ParameterValue(pv) => {
            println!("ParameterValue: {:#?}", pv);
            "".to_string()
        }
        ValueExpression::TypeExpression(te) => type_expression(ctx, te),
    }
}

pub fn function_value(ctx: &mut Context, fv: &FunctionValue) -> String {
    println!("FunctionValue");
    match fv {
        FunctionValue::ValueList(vl) => vl.iter().fold("".to_string(), |s, vle| {
            println!("FunctionValue::ValueList");
            let val_list = value_expression(ctx, vle);
            bf!(= s val_list)
        }),
        FunctionValue::Expression(expr) => {
            println!("FunctionValue::Expression: {:#?}", expr);
            "".to_string()
        }
    }
}

pub fn function_value_call(ctx: &mut Context, efvc: &ExpressionFunctionValueCall) -> String {
    println!("ExpressionFunctionValueCall");
    match efvc {
        ExpressionFunctionValueCall::FunctionValue(ref fv) => function_value(ctx, fv),
        ExpressionFunctionValueCall::FunctionCall(ref fc) => {
            println!("ExpressionFunctionValueCall::FunctionCall: {:#?}", fc);
            "".into()
        }
    }
}

pub fn function_call(_ctx: &mut Context, fc: &FunctionCall) -> String {
    println!("FunctionCall: {:#?}", fc);
    unimplemented!()
}

pub fn fn_body_statement(ctx: &mut Context, fbs: &FunctionBodyStatement) -> String {
    println!("FunctionBodyStatement: {:#?}", fbs);
    match fbs {
        FunctionBodyStatement::Expression(e) => {
            println!("FunctionBodyStatement::Expression");
            let res = function_value_call(ctx, &e.function_statement);
            if let Some(op) = &e.operation_statement {
                println!("operation_statement: {:?}", op);
                if let Some(ex) = &e.expression {
                    println!("expression: {:?}", ex);
                } else {
                    panic!("Expression doesn't exist")
                }
            }
            res
        }
        FunctionBodyStatement::FunctionCall(fc) => {
            println!("FunctionBodyStatement::FunctionCall");
            function_call(ctx, fc)
        }
        FunctionBodyStatement::LetBinding(lb) => {
            println!("FunctionBodyStatement::FunctionCall: {:#?}", lb);
            "".into()
        }
    }
}

pub fn fn_parameter_value_type(
    acc: Vec<(String, Option<String>)>,
    pvt: &ParameterValueType,
) -> Vec<(String, Option<String>)> {
    println!("ParameterValueType");
    match pvt {
        ParameterValueType::Value(v) => {
            let mut res = acc;
            res.push((v.fragment().to_string(), None));
            res
        }
        ParameterValueType::ValueType(v, ref t) => {
            let mut res = acc;
            res.push((v.fragment().to_string(), Some(t[0].fragment().to_string())));
            res
        }
    }
}

pub fn fn_parameter_value_list(
    acc: Vec<(String, Option<String>)>,
    pvl: &ParameterValueList,
) -> Vec<(String, Option<String>)> {
    println!("ParameterValueList");
    match pvl {
        ParameterValueList::ParameterValue(p) => {
            let mut res = acc;
            res.push((p.fragment().to_string(), None));
            res
        }
        ParameterValueList::ParameterList(pl) => pl.iter().fold(acc, fn_parameter_value_type),
    }
}

#[allow(clippy::ptr_arg)]
pub fn fn_body(ast: &FunctionBody) -> Result {
    println!("FunctionBody");
    let mut ctx = Context::new();
    let entry_ctx = Context::new();
    let src = entry!(entry_ctx.get());
    let body_src = ast.iter().fold("".to_string(), |s, b| {
        let fb = fn_body_statement(&mut ctx, b);
        bf!(= s fb)
    });
    let src = bf!(= src body_src);
    println!("{}", src);
    Ok(src)
}

#[allow(clippy::ptr_arg)]
pub fn fn_global_let(ast: &Main) -> Result {
    let mut global_let_statement = 0;
    let mut let_values: Vec<(String, Option<String>)> = vec![];
    let let_src = ast.iter().fold("".to_string(), |s, v| {
        if let MainStatement::LetBinding(l) = v {
            let name = format!("__global_let_init.{}", global_let_statement);
            let mut fn_def = def!(Void name);
            def!(fn_def.linkage @Internal);
            def!(fn_def.attr_group vec![0]);
            def!(fn_def.section_name @".text.startup".to_string());
            global_let_statement += 1;

            // Get Let-names & types
            let mut let_value: Vec<(String, Option<String>)> =
                l.value_list.iter().fold(vec![], fn_parameter_value_list);
            let_values.append(&mut let_value);

            let fn_body_part_src = fn_body(&l.function_body).unwrap();
            let ret = ret!();
            let body = body!(fn_body_part_src ret);
            let fn_body_src = fn_body!(fn_def body);

            merge!(s fn_body_src)
        } else {
            s
        }
    });
    let globals = let_values.iter().fold("".to_string(), |s, l| {
        let mut g = global!(Global Integer32 &l.0);
        global!(g.preemption_specifier @DsoLocal);
        global!(g.initializer_constant @"0".to_string());
        format!("{}{}\n", s, g)
    });

    let mut src = merge!(globals let_src);
    if global_let_statement > 0 {
        let global_ctors = "@llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 65535, void ()* @_GLOBAL_let_main, i8* null }]\n".to_string();
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
            call.push(call_fn.to_string());
        }
        call.push(ret!().to_string());

        let body = body!(@ call);
        let fn_body = fn_body!(fn_def body);
        src = merge!(src global_ctors fn_body);
    }
    Ok(src)
}

fn fn_attr_group() -> String {
    let attr0 = Attributes(0, vec!["noinline".to_string(), "uwtable".to_string()]);
    merge!(attr0)
}

pub fn fn_main(ast: Main) -> Result {
    let module = fn_module(&ast)?;
    let global_let = fn_global_let(&ast)?;
    let attrs = fn_attr_group();
    let src = module!(module global_let attrs);
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
