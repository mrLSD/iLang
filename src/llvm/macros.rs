#[macro_export]
macro_rules! alloca {
    ($ty:ident $res:expr) => {
        Alloca {
            result: format!("%{}", stringify!($res)),
            alloc_ty: $ty,
            elements: None,
            align: None,
            addrspace: None,
        }
    };
    ($ty:ident $res:expr, $align:expr) => {
        Alloca {
            result: format!("%{}", stringify!($res)),
            alloc_ty: $ty,
            elements: None,
            align: Some(super::align::Alignment($align)),
            addrspace: None,
        }
    };
}

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

#[macro_export]
macro_rules! source_file {
    ($name:expr) => {
        SourceFileName(stringify!($name).to_string());
    };
}

#[macro_export]
macro_rules! target_triple {
    ($name:ident) => {
        TargetTriple(stringify!($name).to_string());
    };
}

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

#[macro_export]
macro_rules! load {
    ($var:ident.$attr:ident @ $val:expr) => {{
        $var.$attr = Some($val);
    }};
    ($ty:ident $res:expr, $ptrval:expr) => {{
        Load {
            result: format!("%{}", $res.to_string()),
            volatile: None,
            ty: $ty,
            ty_pointer: $ty,
            pointer: $ptrval.to_string(),
            align: None,
        }
    }};
}
