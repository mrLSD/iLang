//! # Codegen helper macros
//! Implemented most common codegen structures

/// `alloca` macros
///
/// ```ignore
/// // Allocate
/// let res = alloca!(Integer32 3);
/// // Allocate with align
/// let res = alloca!(Integer32 3, 8);
/// ```
#[macro_export]
macro_rules! alloca {
    ($ty:ident $res:expr) => {
        Alloca {
            result: stringify!($res).to_string(),
            alloc_ty: $ty,
            elements: None,
            align: None,
            addrspace: None,
        }
    };
    ($ty:ident $res:expr, $align:expr) => {
        Alloca {
            result: stringify!($res).to_string(),
            alloc_ty: $ty,
            elements: None,
            align: Some(super::align::Alignment($align)),
            addrspace: None,
        }
    };
}

/// `arg` macros
/// Arguments for `define` and `declare`
///
/// ```ignore
/// // Declare 2 arguments with different types
/// let res = arg!(Integer32 0, Integer8 1);
/// // Declare 2 arguments with different types, last argument variadic
/// let res = arg!(Integer32 0, Integer8 1, ...);
/// // Declare 2 argument without value
/// let res = arg!(Integer32, Integer8);
/// // Declare 1 argument without value, last argument variadic
/// let res = arg!(Integer32, ...);
/// ```
#[macro_export]
macro_rules! arg {
    ($($ty:ident $val:expr)? $(,$ty1:ident $val1:expr)*) => {{
        let mut v = vec![];
        $( v.push(ArgumentList {
            parameter_type: Some($ty),
            attributes: None,
            name: Some(format!("%{}", stringify!($val))),
            variable_argument: false,
        });)?
        $( v.push(ArgumentList {
            parameter_type: Some($ty1),
            attributes: None,
            name: Some(format!("%{}", stringify!($val1))),
            variable_argument: false,
        });)*
        v
    }};
    ($($ty:ident $val:expr)? $(,$ty1:ident $val1:expr)*, ...) => {{
        let mut v = vec![];
        $( v.push(ArgumentList {
            parameter_type: Some($ty),
            attributes: None,
            name: Some(format!("%{}", stringify!($val))),
            variable_argument: false,
        });)?
        $( v.push(ArgumentList {
            parameter_type: Some($ty1),
            attributes: None,
            name: Some(format!("%{}", stringify!($val1))),
            variable_argument: false,
        });)*
        v.push(ArgumentList {
            parameter_type: None,
            attributes: None,
            name: None,
            variable_argument: true,
        });
        v
    }};
    ($($ty:ident)? $(,$ty1:ident)*) => {{
        let mut v = vec![];
        $( v.push(ArgumentList {
            parameter_type: Some($ty),
            attributes: None,
            name: None,
            variable_argument: false,
        });)?
        $( v.push(ArgumentList {
            parameter_type: Some($ty1),
            attributes: None,
            name: None,
            variable_argument: false,
        });)*
        v
    }};
    ($($ty:ident)? $(,$ty1:ident)*, ...) => {{
        let mut v = vec![];
        $( v.push(ArgumentList {
            parameter_type: Some($ty),
            attributes: None,
            name: None,
            variable_argument: false,
        });)?
        $( v.push(ArgumentList {
            parameter_type: Some($ty1),
            attributes: None,
            name: None,
            variable_argument: false,
        });)*
        v.push(ArgumentList {
            parameter_type: None,
            attributes: None,
            name: None,
            variable_argument: true,
        });
        v
    }};
}

/// `def` macros
/// `define` instruction
///
/// ```ignore
/// // Basic `define` declaration
/// let mut f = def!(Integer32 main);
/// // Add optional `@` value for field `linkage`
/// def!(f.linkage @External);
/// // Extend non-optional value
/// def!(f.argument_list arg!(Integer32 0, ty1 1));
/// ```
#[macro_export]
macro_rules! def {
    ($fnval:ident.$attr:ident $val:expr) => {{
        $fnval.$attr = $val;
    }};
    ($fnval:ident.$attr:ident @ $val:expr) => {{
        $fnval.$attr = Some($val);
    }};
    ($ty:ident $name:ident) => {{
        Function {
            definition_type: FunctionDefinitionType::Define,
            linkage: None,
            preemption_specifier: None,
            visibility: None,
            dll_storage_class: None,
            cconv: None,
            ret_attrs: None,
            result_type: Type::$ty,
            function_name: stringify!($name).to_string(),
            argument_list: vec![],
            unnamed_addr: None,
            addr_sapce: None,
            fn_attrs: vec![],
            section_name: None,
            comdat: None,
            align: None,
            gc: None,
            prefix: None,
            prologue: None,
            personality: None,
            metadata: None,
        }
    }};
}

/// `decl` macros
/// `declare` instruction
///
/// ```ignore
/// // Basic declaration
/// let mut d = decl!(Integer32 printf);
/// // Extend non-optional value - arg list with variadic args
/// decl!(d.argument_list arg!(ty1, ...));
/// // Extend optional value `@` for `preemption_specifier` field
/// decl!(d.preemption_specifier @DsoLocal);
/// ```
#[macro_export]
macro_rules! decl {
    ($fnval:ident.$attr:ident $val:expr) => {{
        $fnval.$attr = $val;
    }};
    ($fnval:ident.$attr:ident @ $val:expr) => {{
        $fnval.$attr = Some($val);
    }};
    ($ty:ident $name:ident) => {{
        let mut f_decl = def!($ty $name);
        let d = FunctionDefinitionType::Declare;
        def!(f_decl.definition_type d);
        f_decl
    }};
}

/// `source_file` macros
///
/// ```ignore
/// // Described source file `1.ll`
/// let res = source_file!(1.il);
/// ```
#[macro_export]
macro_rules! source_file {
    ($name:expr) => {
        SourceFileName(stringify!($name).to_string());
    };
}

/// `target_triple` macros
///
/// ```ignore
/// // Described target constant parameter
/// let res = target_triple!(TARGET_X86_64_UNKNOWN_LINUX_GNU);
/// ```
#[macro_export]
macro_rules! target_triple {
    ($name:ident) => {
        TargetTriple(crate::llvm::target_triple::$name.to_string());
    };
}

/// `global` macros
///
/// ```ignore
/// // Basic `global` declaration
/// let mut g = global!(Constant gty ".str");
/// // Extend ofr non-optional field `name`
/// global!(g.name "nm".to_string());
/// // Extent with optional `@` value for field `linkage`
/// global!(g.linkage @Private);
/// // Extent with optional `@` value for field `unnamed_addr`
/// global!(g.unnamed_addr @UnnamedAddr);
/// // Extent with optional `@` value for field `initializer_constant`
/// global!(g.initializer_constant @r#"c"Hello: %d\00""#.to_string());
/// ```
#[macro_export]
macro_rules! global {
    ($var:ident.$attr:ident $val:expr) => {{
        $var.$attr = $val;
    }};
    ($var:ident.$attr:ident @ $val:expr) => {{
        $var.$attr = Some($val);
    }};
    ($kind:ident $ty:ident $name:expr) => {
        GlobalVariable {
            name: $name.to_string(),
            linkage: None,
            preemption_specifier: None,
            visibility: None,
            dll_storage_classes: None,
            thread_local: None,
            unnamed_addr: None,
            addrspace: None,
            global_variable_kind: GlobalVariableKind::$kind,
            value_type: $ty,
            initializer_constant: None,
            section: None,
            comdat: None,
            alignment: None,
            metadata: None,
        }
    };
}

/// `store` macros
///
/// ```igonre
/// // Store constant value to `%3` value: store i32 33, i32* %3
/// let res = store!(Integer32 "33", "%3");
/// // Store variable `%2` to `%3` value: store i32 %2, i32* %3
/// let res = store!(Integer32 "%2", "%3");
/// // Extend `store` instruction for optional field `volatile`
/// let res = store!(res.volatile @());
/// ```
#[macro_export]
macro_rules! store {
    ($var:ident.$attr:ident @ $val:expr) => {{
        $var.$attr = Some($val);
    }};
    ($ty:ident $val:expr, $ptrval:expr) => {{
        Store {
            volatile: None,
            ty: $ty,
            value: $val.to_string(),
            ty_pointer: $ty,
            pointer: $ptrval.to_string(),
            align: None,
        }
    }};
}

/// `load` macros
///
/// ```igonre
/// // load to `res` bu addr %3`:  %res = load i32, i32* %3
/// let res = load!(Integer32 "res", "%3");
/// // Extend `load` instruction for optional field `volatile`
/// let res = load!(res.volatile @());
/// ```
#[macro_export]
macro_rules! load {
    ($var:ident.$attr:ident @ $val:expr) => {{
        $var.$attr = Some($val);
    }};
    ($ty:ident $res:expr, $ptrval:expr) => {{
        Load {
            result: $res.to_string(),
            volatile: None,
            ty: $ty,
            ty_pointer: $ty,
            pointer: $ptrval.to_string(),
            align: None,
        }
    }};
}

/// `ret` macros
///
/// ```ignore
/// // return constant: ret i32 0
/// let res = ret!(Integer32 @0);
/// // return value: ret i32 %1
/// let res = ret!(Integer32 @"%1");
/// // return void: ret void
/// let res = ret!();
/// ```
#[macro_export]
macro_rules! ret {
    ($ty:ident @ $val:expr) => {{
        Ret(Some(($ty, $val.to_string())))
    }};
    () => {{
        Ret(None)
    }};
}

/// `b` macros
/// Boxing macros
///
/// ```ignore
/// // Boxing any parameter
/// let res = b!(Integer8);
/// ```
#[macro_export]
macro_rules! b {
    ($ty:expr) => {{
        Box::new($ty)
    }};
}

/// `getelementptr` macros
///
/// ```igonre
/// // Empty range values
/// let res = getelementptr!(Integer64 "el", "@.str" => []);
/// // With range values
/// let res = getelementptr!(Integer64 "el", "@.str" => [Integer64 0, Integer64 0]);
/// // With inbounds & range values
/// let res = getelementptr!(Integer64 inbounds "el", "@.str" => [Integer64 0, Integer64 0]);
/// // With inbounds & range values & `inrange` values
/// let res = getelementptr!(Integer64 inbounds "el", "@.str" => [Integer64 0, => Integer64 0]);
/// ```
#[macro_export]
macro_rules! getelementptr {
    ($ty:ident $res:expr, $ptrval:expr => [$($tyrng1:ident $rng1:expr)? $(=> $tyrng2:ident $rng2:expr)? $(,$tyrng3:ident $rng3:expr)* $(,=> $tyrng4:ident $rng4:expr)*]) => {{
        let mut v = vec![];
        $( v.push((None, $tyrng1, $rng1));)?
        $( v.push((Some(()), $tyrng2, $rng2));)?
        $( v.push((None, $tyrng3, $rng3));)*
        $( v.push((Some(()), $tyrng4, $rng4));)*
        GetElementPtr {
            result: $res.to_string(),
            inbounds: None,
            ty: $ty.clone(),
            ty_pointer: $ty,
            ptr_val: $ptrval.to_string(),
            range_val: v,
        }
    }};
    ($ty:ident inbounds $res:expr, $ptrval:expr => [$($tyrng1:ident $rng1:expr)? $(=> $tyrng2:ident $rng2:expr)? $(,$tyrng3:ident $rng3:expr)* $(,=> $tyrng4:ident $rng4:expr)*]) => {{
        let mut v = vec![];
        $( v.push((None, $tyrng1, $rng1));)?
        $( v.push((Some(()), $tyrng2, $rng2));)?
        $( v.push((None, $tyrng3, $rng3));)*
        $( v.push((Some(()), $tyrng4, $rng4));)*
        GetElementPtr {
            result: $res.to_string(),
            inbounds: Some(()),
            ty: $ty.clone(),
            ty_pointer: $ty,
            ptr_val: $ptrval.to_string(),
            range_val: v,
	}
    }};
}

/// `call` macros
///
/// ```ignore
/// // Basic `call` invokation:  %5 =  call  i32 (i8*, ...) @printf ( i8* %el, i32 %4)
/// let mut res = call!(Integer32 "5" => @printf arg!(ty2, ...) => [ty3 "%el".to_string(), Integer32 "%4".to_string()]);
/// // Extend with optional value for field `tail`
/// call!(res.tail @());
/// ```
#[macro_export]
macro_rules! call {
    ($var:ident.$attr:ident @ $val:expr) => {{
        $var.$attr = Some($val);
    }};
    ($ty:ident $res:expr => $(%$name1:ident)? $(@$name2:ident)? $declargs:expr => [$($argty1:ident $argval1:expr)? $(,$argty2:ident $argval2:expr)*]) => {{
    	#[allow(unused_assignments)]
    	let mut name = None;
    	$(
			name = Some((true, stringify!($name1).to_string()));
    	)?
    	$(
    		if name.is_some() {
    			panic!("can't init `name` twice!");
    		}
			name = Some((false, stringify!($name2).to_string()));
    	)?

    	let mut args: Vec<FunctionArg> = vec![];
    	$(
			args.push(FunctionArg($argty1, $argval1));
    	)?
    	$(
			args.push(FunctionArg($argty2, $argval2));
    	)*

        Call {
            ret_val: $res.to_string(),
            tail: None,
            fast_math_flags: None,
            cconv: None,
            ret_attr: None,
            addrspace: None,
            ty: $ty,
            fnty: $declargs,
            fnptrval: name.unwrap(),
            function_args: args,
            function_attrs: None,
            operand_bundles: None,
        }
    }};
}

/// `entry` macros
/// Label entry. Used for BR
///
/// ```ignore
/// // Basic declaration: entry1:
/// let res = entry!(1);
/// ```
#[macro_export]
macro_rules! entry {
    ($val:expr) => {{
        format!("entry{}:", $val.to_string())
    }};
}

/// `body` macros
/// Function body aggregator
///
/// ```ignore
/// let ret1 = ret!(Integer32 @0);
/// let entry1 = entry!(0);
/// // Function body with instructions inside
/// let body = body!(entry1 ret1);
/// ```
#[macro_export]
macro_rules! body {
    ($($el:expr)*) => {{
    	let s = "";
    	$(
    		let s = format!("{}\n\t{}", s, $el);
    	)*
        format!("{{{}\n}}", s)
    }};
}

/// `body` macros
/// Module declaration - contain main code for file
///
/// ```ignore
/// // Example of module part combinations
/// module!(fn body);
/// ```
#[macro_export]
macro_rules! module {
    ($($el:expr)*) => {{
    	let s = "";
    	$(
    		let s = format!("{}{}\n", s, $el);
    	)*
        format!("{}", s)
    }};
}
