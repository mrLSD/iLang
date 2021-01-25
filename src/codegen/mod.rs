//! # Codegen
//!
//! COdegen based on syntax analyzer and LLVM code generation

use crate::llvm::attribute_groups::Attributes;
use crate::llvm::context::Context;
use crate::llvm::global_variables::UnnamedAddr::UnnamedAddr;
use crate::llvm::instructions::memory_access_addressing_operations::GetElementPtr;
use crate::llvm::instructions::other_operations::Call;
use crate::llvm::linkage_types::LinkageTypes::{
    Internal,
    Private,
};
use crate::llvm::type_system::aggregate::ArrayType;
use crate::llvm::type_system::single_value::PointerType;
use crate::llvm::types::Type;
use crate::llvm::types::Type::{
    Integer1,
    Integer32,
    Integer64,
    Integer8,
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
    function_declarations: Vec<FunctionDeclaration>,
    ast: &'a Main<'a>,
}

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    name: String,
    declaration: String,
}

#[derive(Debug, Clone)]
pub struct FunctionParameter {
    name: String,
    global: bool,
}

impl InstructionSet for FunctionParameter {
    fn set_context(&mut self, _ctx: u64) {}
    fn get_value(&self) -> Option<String> {
        if self.global {
            Some(format!("@{}", self.name))
        } else {
            Some(format!("%{}", self.name))
        }
    }
}

impl std::fmt::Display for FunctionParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = if self.global {
            format!("@{}", self.name)
        } else {
            format!("%{}", self.name)
        };
        write!(f, "{}", s)
    }
}

pub struct TypeExpressionResult {
    pub value: String,
}

/// Instructions set stack for block
pub type VecInstructionSet = Vec<Box<dyn InstructionSet>>;

/// Build in types.
#[derive(Debug, Clone)]
pub enum BuildInTypes {
    Unknown,
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
            function_declarations: vec![],
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
                let value_key = pv.fragment();
                if let Some(x) = self.global_let_values.get(&value_key.to_string()) {
                    println!("\t#[value_expression] ParameterValue: {:#?}", x);
                    vec![Box::new(FunctionParameter {
                        name: x.value.clone(),
                        global: true,
                    })]
                } else {
                    println!(
                        "\t#[value_expression] ParameterValue [doesn't exist]: {}",
                        value_key
                    );
                    vec![]
                }
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

    #[allow(clippy::vec_init_then_push)]
    pub fn function_call(
        &mut self,
        ctx: &Context,
        fc: &FunctionCall,
    ) -> (Context, VecInstructionSet) {
        use crate::llvm::functions::ArgumentList;

        println!("\t#[call]: function_call: FunctionCall");
        if fc.function_call_name.is_empty() {
            return (ctx.clone(), vec![]);
        }
        let mut raw_ctx = ctx.clone().val();
        let fn_name = fc.function_call_name[0].fragment();
        println!("\t#[function_call] fn_name: {}", fn_name);
        let data: (Vec<String>, Vec<ArgumentList>) = fc
            .function_value
            .iter()
            .fold(vec![], |instr, v| {
                let mut x = instr;
                x.append(&mut self.function_value(v));
                x
            })
            .iter_mut()
            .fold((vec![], vec![]), |d, param| {
                let mut new_data = d;
                raw_ctx += 1;
                //params[i].set_context(raw_ctx);
                param.set_context(raw_ctx);
                print!("\t->{} [{}] ", raw_ctx, param);
                if let Some(p) = param.get_type() {
                    println!("type: {}", p);
                    let n1 = param.get_value().unwrap();
                    raw_ctx += 1;
                    let val_alias = raw_ctx.to_string();
                    let p1 = p.clone();
                    let ge1 =
                        getelementptr!(p inbounds val_alias, n1 => [Integer64 0, Integer64 0]);
                    println!("\t->{}: {} = {}", val_alias, p1, ge1);
                    let arg = ArgumentList {
                        parameter_type: Some(p1),
                        attributes: None,
                        name: Some(val_alias),
                        variable_argument: false,
                    };
                    new_data.0.push(ge1.to_string());
                    new_data.1.push(arg);
                } else {
                    let ty1 = Type::Pointer(PointerType(Box::new(Integer8)));
                    let n1 = param.get_value().unwrap();
                    println!("no-type: {}: {}", n1, ty1);
                }
                new_data
            });
        // Declare function
        let ty1 = Type::Pointer(PointerType(Box::new(Integer8)));
        let mut fn_decl = decl!(Integer32 fn_name);
        let args = arg!(ty1, ...);
        let args_decl = args.clone();
        decl!(fn_decl.argument_list args);
        println!("\t->{}", fn_decl);
        let fn_call = call!(Integer32 => @fn_name args_decl => []);
        println!("\t->{}", fn_call);
        println!("\t->{:?}", data);

        /*.iter()
        .enumerate()
        .for_each(|(idx, param)| {
            let mut param = param.clone();
            raw_ctx += 1;
            &param.set_context(raw_ctx);
            print!("\t->{} ", idx);
            if let Some(p) = param.get_type() {
                println!("type: {}", p);
            } else {
                println!("no-type");
            }
        });*/
        /*println!("\t#[function_call]: [{:?}]", params[0].get_type());
        if let Some(p) = params[0].get_type() {
            // Get param for function
            //if params[0].is_global() {
            let n1 = params[0].get_value().unwrap();
            let n2 = "1".to_string();
            let n2_val = format!("%{}", n2);
            println!("{:?}", n1);
            let ge1 = getelementptr!(p inbounds n2, n1 => [Integer64 0, Integer64 0]);
            let n3 = params[1].get_value().unwrap();

            // Call function
            //let n4 = params[1].get_value().unwrap();
            let fn_call = call!(Integer32 => @fn_name arg!(ty2, ...) => [ty3 n2_val, Integer32 n3]);
            println!(
                "\t#[function_call]:\n\t-> {}\n\t-> {}\n\t-> {}",
                ge1, fn_decl, fn_call
            );
        }
        */

        /*
        let a1 = load!(Integer32 "1", "2");
        let gty = Type::Array(ArrayType((s.len() + 1) as i32, Box::new(Type::Integer8)));
        let ge1 = getelementptr!(Integer64 inbounds "el", "@.str" => [Integer64 0, Integer64 0]);

        // Set function declaration
        let ty1 = Type::Pointer(PointerType(Box::new(Integer8)));
        let ty2 = ty1.clone();
        let ty3 = ty1.clone();
        let mut fn_decl = decl!(Integer32 fn_name);
        decl!(fn_decl.argument_list arg!(ty1, ...));
        self.function_declarations.push(FunctionDeclaration {
            name: fn_name.to_string(),
            declaration: fn_decl.to_string(),
        });

        let fn_call = call!(Integer32 => @fn_name arg!(ty2, ...) => [ty3 "%el".to_string(), Integer32 "%4".to_string()]);

        // let name = format!("__global_let_init.{}", i);
        // let call_fn = call!(Void => @name vec![] => []);
        println!(
            "\t#[function_call] params: {:?} #{} |> {} |> {:?} |> {} |> {} ",
            params.len(),
            fn_call,
            fn_decl,
            fn_call.function_args,
            a1,
            ge1,
        );*/
        ((*ctx).clone(), vec![])
    }

    pub fn fn_body_statement(
        &mut self,
        ctx: &Context,
        fbs: &FunctionBodyStatement,
    ) -> (Context, VecInstructionSet) {
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
                (ctx.clone(), res)
            }
            FunctionBodyStatement::FunctionCall(fc) => {
                println!("\t#[fn_body_statement] FunctionCall");
                self.function_call(&ctx, fc)
            }
            FunctionBodyStatement::LetBinding(lb) => {
                println!("\t#[fn_body_statement] LetBinding: {:#?}", lb);
                (ctx.clone(), vec![])
            }
        }
    }

    pub fn fn_parameter_value_type(&self, pvt: &ParameterValueType) -> ValueType {
        println!("\t#[call] fn_parameter_value_type: ParameterValueType");
        match pvt {
            ParameterValueType::Value(v) => ValueType {
                value: v.fragment().to_string(),
                value_type: None,
            },
            ParameterValueType::ValueType(v, ref t) => {
                // TODO: detect type
                let _ty = t[0].fragment().to_string();
                ValueType {
                    value: v.fragment().to_string(),
                    value_type: Some(BuildInTypes::Unknown),
                }
            }
        }
    }

    pub fn fn_parameter_value_list(&self, pvl: &ParameterValueList) -> Vec<ValueType> {
        println!("\t#[call] fn_parameter_value_list: ParameterValueList");
        match pvl {
            ParameterValueList::ParameterValue(p) => {
                vec![ValueType {
                    value: p.fragment().to_string(),
                    value_type: None,
                }]
            }
            ParameterValueList::ParameterList(pl) => pl.iter().fold(vec![], |mut acc, v| {
                acc.push(self.fn_parameter_value_type(v));
                acc
            }),
        }
    }

    pub fn fn_body(&mut self, ast: &FunctionBody) -> Result {
        println!("\t#[call] fn_body: FunctionBody");
        let mut entry_ctx = Context::new();
        let src = entry!(entry_ctx.get());
        let body_src = ast.iter().fold("".to_string(), |_, b| {
            let (ctx, statement) = self.fn_body_statement(&entry_ctx, b);
            entry_ctx = ctx;
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

    // Very simplified representation
    fn init_fn_def(&self, fn_name: &str) -> String {
        let mut fn_def = def!(Integer32 fn_name);
        def!(fn_def.linkage @Internal);
        def!(fn_def.attr_group vec![0]);
        fn_def.to_string()
    }

    fn set_let_value_types(&mut self, l: &LetBinding) {
        for v in l.value_list.iter() {
            for vt in self.fn_parameter_value_list(v) {
                self.global_let_values.insert(vt.value.clone(), vt);
            }
        }
    }

    pub fn fn_global_let(&mut self) -> Result {
        println!("\t#[call] fn_global_let");
        let mut global_let_statement = 0;
        // Fetch AST tree and generate source code
        let let_src = self.ast.iter().fold("".to_string(), |src, v| {
            // Global let bindings
            println!("{:#?}", v);
            match v {
                MainStatement::LetBinding(l) => {
                    // Get Let-names & types
                    self.set_let_value_types(l);
                    // Function definition
                    let fn_def = self.global_init_fn_def(global_let_statement);
                    // Get function body
                    let body = body!(self.fn_body(&l.function_body).unwrap() ret!());
                    // Generate function
                    let fn_body_src = fn_body!(fn_def body);
                    global_let_statement += 1;
                    // Merge generated code
                    merge!(src fn_body_src)
                }
                MainStatement::Function(f) => {
                    let fn_def = self.init_fn_def(&f.function_name);
                    // Get function body
                    let body = body!(self.fn_body(&f.function_body).unwrap() ret!());
                    // Generate function
                    let fn_body_src = fn_body!(fn_def body);
                    // Merge generated code
                    merge!(src fn_body_src)
                }
                _ => src,
            }
        });
        // TODO: remove
        self.global_let_values
            .iter()
            .for_each(|(n, _)| println!("\t# [glv] {}", n));

        let mut src = let_src;
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
        let mut codegen = Self::new(ast);
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
            "module name1.name2\nlet x1 = 10\nlet main () = printfn \"Res: %A\" x1",
        ))
        .unwrap();
        assert_eq!(x.0.fragment(), &"");
        let res = Codegen::fn_main(&x.1);
        assert!(res.is_ok());
    }
}
