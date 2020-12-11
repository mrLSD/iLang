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
