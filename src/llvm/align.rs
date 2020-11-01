//! # Alignments

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Alignment(u32);

impl std::fmt::Display for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "align {}", self.0)
    }
}
