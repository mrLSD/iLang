#[macro_export]
macro_rules! alloca {
    ($res:expr; $ty:expr) => {
        Alloca {
            result: stringify!($res).to_string(),
            alloc_ty: $ty,
            elements: None,
            align: None,
            addrspace: None,
        }
    };
    ($res:expr; $ty:expr; $align:expr) => {
        Alloca {
            result: stringify!($res).to_string(),
            alloc_ty: $ty,
            elements: None,
            align: Some(crate::llvm::align::Alignment($align)),
            addrspace: None,
        }
    };
}

// fn sad() {
// 	Alignment
// }
