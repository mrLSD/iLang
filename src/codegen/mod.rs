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

#[derive(Debug, Clone)]
pub struct Codegen<'a> {
    ctx: Context,
    global_let_values: Vec<String>,
    ast: &'a Main<'a>,
}

impl<'a> Codegen<'a> {
    #[allow(clippy::ptr_arg)]
    fn new(ast: &'a Main) -> Self {
        Self {
            ctx: Context::new(),
            global_let_values: vec![],
            ast,
        }
    }

    #[allow(clippy::ptr_arg)]
    pub fn expression(&self) -> Result {
        println!(" # expression");
        let src = "".to_string();
        Ok(src)
    }

    pub fn type_expression(&mut self, te: &TypeExpression) -> (String, Option<String>) {
        println!(" # type_expression: TypeExpression = {:#?}", te.expr);
        match te.expr {
            BasicTypeExpression::Number(n) => {
                let a = alloca!(Integer32 self.ctx.get());
                let result_val = self.ctx.val();
                let s = store!(Integer32 n, result_val);
                self.ctx.inc();
                (bf!(a s), Some(result_val))
            }
            _ => unimplemented!(),
        }
    }

    pub fn value_expression(&mut self, vle: &ValueExpression) -> (String, Option<String>) {
        println!(" # value_expression: ValueExpression");
        match vle {
            ValueExpression::ParameterValue(pv) => {
                println!("ParameterValue: {:#?}", pv);
                ("".to_string(), None)
            }
            ValueExpression::TypeExpression(te) => self.type_expression(te),
        }
    }

    pub fn function_value(&mut self, fv: &FunctionValue) -> (String, Option<String>) {
        println!(" # function_value: FunctionValue");
        match fv {
            FunctionValue::ValueList(vl) => vl.iter().fold(("".to_string(), None), |s, vle| {
                println!("FunctionValue::ValueList");
                let (val_list, res_value) = self.value_expression(vle);
                println!(" #= FunctionValue::ValueList res_value: {:?}", res_value);
                (bf!(= s.0 val_list), res_value)
            }),
            FunctionValue::Expression(expr) => {
                println!("FunctionValue::Expression: {:#?}", expr);
                ("".to_string(), None)
            }
        }
    }

    pub fn function_value_call(
        &mut self,
        efvc: &ExpressionFunctionValueCall,
    ) -> (String, Option<String>) {
        println!(" # function_value_call: ExpressionFunctionValueCall");
        match efvc {
            ExpressionFunctionValueCall::FunctionValue(ref fv) => self.function_value(fv),
            ExpressionFunctionValueCall::FunctionCall(ref fc) => {
                println!("ExpressionFunctionValueCall::FunctionCall: {:#?}", fc);
                ("".into(), None)
            }
        }
    }

    pub fn function_call(&self, fc: &FunctionCall) -> String {
        println!(" # function_call: FunctionCall = {:#?}", fc);
        unimplemented!()
    }

    pub fn fn_body_statement(&mut self, fbs: &FunctionBodyStatement) -> (String, Option<String>) {
        println!(" # fn_body_statement: FunctionBodyStatement = {:#?}", fbs);
        match fbs {
            FunctionBodyStatement::Expression(e) => {
                println!("FunctionBodyStatement::Expression");
                let res = self.function_value_call(&e.function_statement);
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
                (self.function_call(fc), None)
            }
            FunctionBodyStatement::LetBinding(lb) => {
                println!("FunctionBodyStatement::FunctionCall: {:#?}", lb);
                ("".into(), None)
            }
        }
    }

    pub fn fn_parameter_value_type(
        &self,
        acc: Vec<(String, Option<String>)>,
        pvt: &ParameterValueType,
    ) -> Vec<(String, Option<String>)> {
        println!(" # fn_parameter_value_type: ParameterValueType");
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
        println!(" # fn_parameter_value_list: ParameterValueList");
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

    #[allow(clippy::ptr_arg)]
    pub fn fn_body(
        &mut self,
        ast: &FunctionBody,
        let_value_name: &Vec<(String, Option<String>)>,
    ) -> Result {
        println!(" # fn_body: FunctionBody");
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
                    let s = store!(Integer32 self.ctx.val(), name);
                    self.ctx.inc();
                    bf!(fb l s)
                }
            } else {
                fb
            };
            bf!(= s fb)
        });
        let src = bf!(= src body_src);
        println!("  #= fn_body: {}", src);
        Ok(src)
    }

    #[allow(clippy::ptr_arg)]
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

    #[allow(clippy::ptr_arg)]
    pub fn fn_global_let(&mut self) -> Result {
        println!(" # fn_global_let");
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
        let globals = let_values.iter().fold("".to_string(), |s, l| {
            self.global_let_values.push(l.0.clone());
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

    #[allow(clippy::ptr_arg)]
    pub fn fn_main(ast: &'a Main) -> Result {
        println!(" # fn_main");
        let mut codegen = Self::new(&ast);
        let module = codegen.fn_module()?;
        let global_let = codegen.fn_global_let()?;
        let attrs = codegen.fn_attr_group();
        let src = module!(module global_let attrs);
        println!("\n{}", src);
        Ok(src)
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
}
