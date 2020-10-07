//! # Numbered address space

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct AddrSpace(u32);

impl std::fmt::Display for AddrSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "addrspace({})", self.0)
    }
}
