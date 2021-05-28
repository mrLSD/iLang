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
    Integer1,
    Integer32,
    Void,
};
use crate::llvm::InstructionSet;
use crate::parser::ast::*;
use std::collections::{
    HashMap,
    HashSet,
};

pub type Result = std::result::Result<String, CodegenError>;

#[derive(Debug, Eq, PartialEq)]
pub enum CodegenError {
    ModuleNotFound,
    ParseSourceCode,
}

/// Codegen structure
#[derive(Debug, Clone)]
pub struct Codegen<'a> {
    ctx: Context,
    global_ctx: Context,
    let_values: HashSet<String>,
    global_let_values: HashMap<LetValueName, ValueType>,
    global_let_expressions: Vec<String>,
    ast: &'a Main<'a>,
}

pub struct TypeExpressionResult {
    pub value: String,
}

/// Instructions set stack for block
pub type VecInstructionSet = Vec<Box<dyn InstructionSet>>;

/// Build in types.
#[derive(Debug, Clone)]
pub enum BuildInTypes {
    String,
    Int,
    Bool,
    Custom(String),
}

type LetValueName = String;

/// Value and their type representation
#[derive(Debug, Clone)]
pub struct ValueType {
    pub value: LetValueName,
    pub value_type: Option<BuildInTypes>,
}

impl<'a> Codegen<'a> {
    #[allow(clippy::ptr_arg)]
    fn new(ast: &'a Main) -> Self {
        Self {
            ctx: Context::new(),
            global_ctx: Context::new(),
            let_values: HashSet::new(),
            global_let_values: HashMap::new(),
            global_let_expressions: vec![],
            ast,
        }
    }

    pub fn expression(&self) -> Result {
        println!("\t#[call] expression");
        let src = "".to_string();
        Ok(src)
    }

    pub fn type_expression(&mut self, te: &TypeExpression) -> VecInstructionSet {
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
                vec![Box::new(g)]
            }
            BasicTypeExpression::Bool(n) => {
                let mut v: Vec<Box<dyn InstructionSet>> = vec![];
                v.push(Box::new(alloca!(Integer1 0)));
                v.push(Box::new(store!(Integer1 n, 0)));
                v
            }
        }
    }

    pub fn value_expression(&mut self, vle: &ValueExpression) -> VecInstructionSet {
        println!("\t#[call] value_expression: ValueExpression");
        match vle {
            ValueExpression::ParameterValue(pv) => {
                // TODO: Get value form SOME key-value: parameter-value -> codegen-patam-alias
                println!(
                    "\t#[value_expression] ParameterValue [not-implemented]: {:#?}",
                    pv
                );
                vec![]
                //unimplemented!();
            }
            ValueExpression::TypeExpression(te) => {
                println!("\t#[value_expression] TypeExpression");
                self.type_expression(te)
            }
        }
    }

    pub fn function_value(&mut self, fv: &FunctionValue) -> VecInstructionSet {
        println!("\t#[call] function_value: FunctionValue");
        match fv {
            FunctionValue::ValueList(vl) => vl.iter().fold(vec![], |s, vle| {
                println!("\t#[function_value] ValueList");
                let mut res = self.value_expression(vle);
                println!("\t#[function_value] ValueList [{}]", res.len());
                let mut x = s;
                x.append(&mut res);
                x
            }),
            FunctionValue::Expression(expr) => {
                println!("\t#[function_value] Expression [not impl]: {:#?}", expr);
                vec![]
            }
        }
    }

    pub fn function_value_call(&mut self, efvc: &ExpressionFunctionValueCall) -> VecInstructionSet {
        println!("\t#[call] function_value_call: ExpressionFunctionValueCall");
        match efvc {
            ExpressionFunctionValueCall::FunctionValue(ref fv) => {
                println!("\t#[function_value_call] FunctionValue");
                self.function_value(fv)
            }
            ExpressionFunctionValueCall::FunctionCall(ref fc) => {
                println!("\t#[function_value_call] FunctionCall: {:#?}", fc);
                vec![]
            }
        }
    }

    pub fn function_call(&mut self, fc: &FunctionCall) -> VecInstructionSet {
        println!("\t#[call]: function_call: FunctionCall");
        if fc.function_call_name.is_empty() {
            return vec![];
        }
        let fn_name = fc.function_call_name[0].fragment();
        println!("\t#[function_call] fn_name: {}", fn_name);
        let params: Vec<String> = fc.function_value.iter().fold(vec![], |_s, v| {
            let x = self.function_value(v);
            //println!("\t#[function_call] fn_function_value: {:?}", x);
            println!("\t#[function_call] fn_function_value: [{}]", x.len());
            x.iter().fold(0, |_, v| {
                println!("\t#[function_call] elem: {:#?}", v);
                0
            });
            // let mut data = vec![];
            // data.push("".into());
            // data
            vec![]
        });
        println!("\t#[function_call] params: {:?}", params);
        eprintln!(";TODO: function_call");
        vec![]
    }

    pub fn fn_body_statement(&mut self, fbs: &FunctionBodyStatement) -> VecInstructionSet {
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
                self.function_call(fc)
            }
            FunctionBodyStatement::LetBinding(lb) => {
                println!("\t#[fn_body_statement] FunctionCall: {:#?}", lb);
                vec![]
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

    pub fn fn_parameter_value_list(&self, pvl: &ParameterValueList) -> Vec<ValueType> {
        println!("\t#[call] fn_parameter_value_list: ParameterValueList");
        vec![]
        /*match pvl {
            ParameterValueList::ParameterValue(p) => {
                ValueType {
                    value: p.fragment().to_string(),
                    value_type: None,
                }
            }
            ParameterValueList::ParameterList(pl) => pl
                .iter()
                .fold(acc, |acc, v| self.fn_parameter_value_type(acc, v)),
        }*/
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
            let statement = self.fn_body_statement(b);
            statement.iter().fold((), |_, v| println!("\t# {:#?}", v));
            /*let fb = if let Some(ref v) = res_val {
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
            */
            let (s, fb) = ("", "");
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

    fn global_init_fn_def(&self, global_let_statement: u64) -> String {
        let name = format!("__global_let_init.{}", global_let_statement);
        let mut fn_def = def!(Void name);
        def!(fn_def.linkage @Internal);
        def!(fn_def.attr_group vec![0]);
        def!(fn_def.section_name @".text.startup".to_string());
        fn_def.to_string()
    }

    fn set_let_value_types(&mut self, l: LetBinding) {
        for v in l.value_list.iter() {
            for vt   in self.fn_parameter_value_list(v) {
                self.global_let_values.insert(vt.value.clone(), vt.clone());
            }
        }
    }

    pub fn fn_global_let(&mut self) -> Result {
        println!("\t#[call] fn_global_let");
        let mut global_let_statement = 0;
        let mut let_values: Vec<(String, Option<String>)> = vec![];
        // Fetch AST tree and generate source code
        let let_src = self.ast.iter().fold("".to_string(), |src, v| {
            // Global let bindings
            if let MainStatement::LetBinding(l) = v {
                // Get Let-names & types
                /*let mut let_value: Vec<(String, Option<String>)> = l
                .value_list
                .iter()
                .fold(vec![], |acc, v| self.fn_parameter_value_list(acc, v));*/
                let mut let_value = vec![];
                let let_binding_val = let_value.clone();
                let_values.append(&mut let_value);

                // Function definition
                let fn_def = self.global_init_fn_def(global_let_statement);
                // Get function body
                let body = body!(self.fn_body(&l.function_body, &let_binding_val).unwrap() ret!());
                // Generate function
                let fn_body_src = fn_body!(fn_def body);
                global_let_statement += 1;
                // Merge generated code
                merge!(src fn_body_src)
            } else if let MainStatement::Function(_) = v {
                todo!("should be implemented global functions codegen")
            } else {
                src
            }
        });
        let globals_from_let = self
            .global_let_expressions
            .iter()
            .fold("".to_string(), |s, l| format!("{}{}\n", s, l));
        // TODO: remove
        let globals = let_values.iter().fold(globals_from_let, |s, l| {
            //self.global_let_values.insert(l.0.clone());
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
