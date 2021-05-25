//! # Codegen
//!
//! COdegen based on syntax analyzer and LLVM code generation

use crate::llvm::attribute_groups::Attributes;
use crate::llvm::context::Context;
use crate::llvm::global_variables::UnnamedAddr::UnnamedAddr;
use crate::llvm::instructions::other_operations::Call;
use crate::llvm::linkage_types::LinkageTypes::{
    Internal,
    Private,
};
use crate::llvm::runtime_preemption::RuntimePreemptionSpecifier::DsoLocal;
use crate::llvm::type_system::aggregate::ArrayType;
use crate::llvm::types::Type;
use crate::llvm::types::Type::{
    Integer32,
    Void,
};
use crate::llvm::InstructionSet;
use crate::parser::ast::*;
use std::collections::HashSet;

pub type Result = std::result::Result<String, CodegenError>;

#[derive(Debug, Eq, PartialEq)]
pub enum CodegenError {
    ModuleNotFound,
    ParseSourceCode,
}

#[derive(Debug, Clone)]
pub struct Codegen<'a> {
    ctx: Context,
    global_ctx: Context,
    let_values: HashSet<String>,
    global_let_values: HashSet<String>,
    global_let_expressions: Vec<String>,
    ast: &'a Main<'a>,
}

pub struct TypeExpressionResult {
    pub value: String,
}

impl<'a> Codegen<'a> {
    #[allow(clippy::ptr_arg)]
    fn new(ast: &'a Main) -> Self {
        Self {
            ctx: Context::new(),
            global_ctx: Context::new(),
            let_values: HashSet::new(),
            global_let_values: HashSet::new(),
            global_let_expressions: vec![],
            ast,
        }
    }

    pub fn expression(&self) -> Result {
        println!("\t#[call] expression");
        let src = "".to_string();
        Ok(src)
    }

    pub fn type_expression(&mut self, te: &TypeExpression) -> Vec<Box<dyn InstructionSet>> {
        println!("\t#[call] type_expression: TypeExpression = {:#?}", te.expr);
        match te.expr {
            BasicTypeExpression::Number(n) => {
                let mut v: Vec<Box<dyn InstructionSet>> = vec![];
                v.push(Box::new(alloca!(Integer32 0)));
                v.push(Box::new(store!(Integer32 n, 0)));
                v
            }
            BasicTypeExpression::String(ref s) => {
                let gty = Type::Array(ArrayType((s.len() + 1) as i32, Box::new(Type::Integer8)));
                let mut g = global!(Constant gty 0);
                global!(g.linkage @Private);
                global!(g.unnamed_addr @UnnamedAddr);
                global!(g.initializer_constant @Type::raw_string(s));
                self.global_ctx.inc();
                self.global_let_expressions.push(g.to_string());
                vec![]
            }
            _ => unimplemented!(),
        }
    }

    pub fn value_expression(&mut self, vle: &ValueExpression) -> (String, Option<String>) {
        println!("\t#[call] value_expression: ValueExpression");
        match vle {
            ValueExpression::ParameterValue(pv) => {
                println!("\t#[value_expression] ParameterValue: {:#?}", pv);
                (";TODO: value_expression".to_string(), None)
                //unimplemented!();
            }
            ValueExpression::TypeExpression(te) => {
                println!("\t#[value_expression] TypeExpression");
                let _ = self.type_expression(te);
                (";TODO: value_expression".to_string(), None)
            }
        }
    }

    pub fn function_value(&mut self, fv: &FunctionValue) -> (String, Option<String>) {
        println!("\t#[call] function_value: FunctionValue");
        match fv {
            FunctionValue::ValueList(vl) => vl.iter().fold(("".to_string(), None), |s, vle| {
                println!("\t#[function_value] FunctionValue::ValueList");
                let (val_list, res_value) = self.value_expression(vle);
                println!(
                    "\t#[function_value] res_value: {:?}\n\t{:?}",
                    val_list, res_value
                );
                (bf!(= s.0 val_list), res_value)
            }),
            FunctionValue::Expression(expr) => {
                println!("\t#[function_value] FunctionValue::Expression: {:#?}", expr);
                ("".to_string(), None)
            }
        }
    }

    pub fn function_value_call(
        &mut self,
        efvc: &ExpressionFunctionValueCall,
    ) -> (String, Option<String>) {
        println!("\t#[call] function_value_call: ExpressionFunctionValueCall");
        match efvc {
            ExpressionFunctionValueCall::FunctionValue(ref fv) => {
                println!("\t#[function_value_call] FunctionValue");
                self.function_value(fv)
            }
            ExpressionFunctionValueCall::FunctionCall(ref fc) => {
                println!("\t#[function_value_call] FunctionCall: {:#?}", fc);
                ("".into(), None)
            }
        }
    }

    pub fn function_call(&mut self, fc: &FunctionCall) -> String {
        println!("\t#[call]: function_call: FunctionCall = {:#?}", fc);
        if fc.function_call_name.is_empty() {
            return "".into();
        }
        let fn_name = fc.function_call_name[0].fragment();
        println!("\t#[function_call] fn_name: {}", fn_name);
        let params: Vec<String> = fc.function_value.iter().fold(vec![], |s, v| {
            let x = self.function_value(v);
            println!("\t#[function_call] fn_function_value: {:?}", x);
            let mut data = s;
            data.push("".into());
            data
        });
        println!("\t#[function_call] params: {:?}", params);
        eprintln!(";TODO: function_call");
        "".into()
    }

    pub fn fn_body_statement(&mut self, fbs: &FunctionBodyStatement) -> (String, Option<String>) {
        println!(
            "\t#[call] fn_body_statement: FunctionBodyStatement = {:#?}",
            fbs
        );
        match fbs {
            FunctionBodyStatement::Expression(e) => {
                println!("\t#[fn_body_statement] Expression");
                let res = self.function_value_call(&e.function_statement);
                if let Some(op) = &e.operation_statement {
                    println!("\t#[fn_body_statement] operation_statement: {:?}", op);
                    if let Some(ex) = &e.expression {
                        println!("\t#[fn_body_statement] expression: {:?}", ex);
                    } else {
                        panic!("\t#[fn_body_statement] Expression doesn't exist")
                    }
                }
                res
            }
            FunctionBodyStatement::FunctionCall(fc) => {
                println!("\t#[fn_body_statement] FunctionCall");
                (self.function_call(fc), None)
            }
            FunctionBodyStatement::LetBinding(lb) => {
                println!("\t#[fn_body_statement] FunctionCall: {:#?}", lb);
                ("".into(), None)
            }
        }
    }

    pub fn fn_parameter_value_type(
        &self,
        acc: Vec<(String, Option<String>)>,
        pvt: &ParameterValueType,
    ) -> Vec<(String, Option<String>)> {
        println!("\t#[call] fn_parameter_value_type: ParameterValueType");
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
        &self,
        acc: Vec<(String, Option<String>)>,
        pvl: &ParameterValueList,
    ) -> Vec<(String, Option<String>)> {
        println!("\t#[call] fn_parameter_value_list: ParameterValueList");
        match pvl {
            ParameterValueList::ParameterValue(p) => {
                let mut res = acc;
                res.push((p.fragment().to_string(), None));
                res
            }
            ParameterValueList::ParameterList(pl) => pl
                .iter()
                .fold(acc, |acc, v| self.fn_parameter_value_type(acc, v)),
        }
    }

    pub fn fn_body(
        &mut self,
        ast: &FunctionBody,
        let_value_name: &Vec<(String, Option<String>)>,
    ) -> Result {
        println!("\t#[call] fn_body: FunctionBody");
        let entry_ctx = Context::new();
        let src = entry!(entry_ctx.get());
        let body_src = ast.iter().fold("".to_string(), |s, b| {
            let (fb, res_val) = self.fn_body_statement(b);
            let fb = if let Some(ref v) = res_val {
                if let_value_name.is_empty() {
                    fb
                } else {
                    // TODO: extend for multi-values
                    // Also currently onlu Global variables
                    let name = format!("@{}", &let_value_name[0].0);
                    let l = load!(Integer32 self.ctx.get(), v);
                    let s = store!(Integer32 0, name);
                    self.ctx.inc();
                    println!("\t#[fn_body] 1> {:?}\n{:?}\n{:?}", fb, l, s);
                    bf!(=fb bf!(l s))
                }
            } else {
                println!("\t#[fn_body] 2> {:?}", fb);
                fb
            };
            println!("\t#[fn_body] 3> {:?} {:?}", s, fb);
            bf!(= s fb)
        });
        println!("\t#[fn_body] fn_body: {:?}", src);
        println!("\t#[fn_body] fn_body: {:?}", body_src);
        let src = bf!(= src body_src);

        println!("\t#[fn_body] fn_body: {:?}", src);
        Ok(src)
    }

    pub fn fn_module(&self) -> Result {
        let src: String;
        if !self.ast.is_empty() {
            if let MainStatement::Module(m) = &self.ast[0] {
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

    pub fn fn_global_let(&mut self) -> Result {
        println!("\t#[call] fn_global_let");
        let mut global_let_statement = 0;
        let mut let_values: Vec<(String, Option<String>)> = vec![];
        let let_src = self.ast.iter().fold("".to_string(), |s, v| {
            if let MainStatement::LetBinding(l) = v {
                let name = format!("__global_let_init.{}", global_let_statement);
                let mut fn_def = def!(Void name);
                def!(fn_def.linkage @Internal);
                def!(fn_def.attr_group vec![0]);
                def!(fn_def.section_name @".text.startup".to_string());
                global_let_statement += 1;

                // Get Let-names & types
                let mut let_value: Vec<(String, Option<String>)> = l
                    .value_list
                    .iter()
                    .fold(vec![], |acc, v| self.fn_parameter_value_list(acc, v));
                let let_binding_val = let_value.clone();
                let_values.append(&mut let_value);

                let fn_body_part_src = self.fn_body(&l.function_body, &let_binding_val).unwrap();
                let ret = ret!();
                let body = body!(fn_body_part_src ret);
                let fn_body_src = fn_body!(fn_def body);
                merge!(s fn_body_src)
            } else {
                s
            }
        });
        let globals_from_let = self
            .global_let_expressions
            .iter()
            .fold("".to_string(), |s, l| format!("{}{}\n", s, l));
        let globals = let_values.iter().fold(globals_from_let, |s, l| {
            self.global_let_values.insert(l.0.clone());
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

    fn fn_attr_group(&self) -> String {
        let attr0 = Attributes(0, vec!["noinline".to_string(), "uwtable".to_string()]);
        merge!(attr0)
    }

    pub fn fn_main(ast: &'a Main) -> Result {
        println!("\t#[call] fn_main");
        let mut codegen = Self::new(&ast);
        let module = codegen.fn_module()?;
        let global_let = codegen.fn_global_let()?;
        let attrs = codegen.fn_attr_group();
        let src = module!(module global_let attrs);
        println!("\n{}", src);
        Ok(src)
    }

    pub fn build(source: &str) -> Result {
        use crate::parser::token::main;

        let src = main(Span::new(source)).unwrap();
        if src.0.fragment().is_empty() {}
        Codegen::fn_main(&src.1)
    }
}

#[cfg(test)]
mod tests {
    use crate::codegen::{
        Codegen,
        CodegenError,
    };
    use crate::parser::{
        ast::Span,
        token::main,
    };

    #[test]
    fn test_codegen_main_module_not_found() {
        let x = main(Span::new("let val1 = 10")).unwrap();
        assert_eq!(x.0.fragment(), &"");
        let res = Codegen::fn_main(&x.1);
        assert_eq!(res.unwrap_err(), CodegenError::ModuleNotFound);
    }

    #[test]
    fn test_codegen_global_let_binding() {
        let x = main(Span::new("module name1.name2\nlet x1 = 10")).unwrap();
        assert_eq!(x.0.fragment(), &"");
        let res = Codegen::fn_main(&x.1);
        assert!(res.is_ok());
    }

    #[test]
    fn test_codegen_global_let_expression() {
        let x = main(Span::new("module name1.name2\nlet x2 = 10 * x1")).unwrap();
        assert_eq!(x.0.fragment(), &"");
        let res = Codegen::fn_main(&x.1);
        assert!(res.is_ok());
    }

    // Current test
    #[test]
    fn test_codegen_global_let_and_print() {
        let x = main(Span::new(
            "module name1.name2\nlet x1 = 10\nlet main = printfn \"Res: %A\" x1",
        ))
        .unwrap();
        assert_eq!(x.0.fragment(), &"");
        let res = Codegen::fn_main(&x.1);
        assert!(res.is_ok());
    }
}
