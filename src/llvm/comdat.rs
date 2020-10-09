//! # COMDAT

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ComDat {
    name: String,
    comdat: Option<Box<ComDat>>,
}

impl std::fmt::Display for ComDat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // TODO: change it
        write!(f, "")
    }
}
