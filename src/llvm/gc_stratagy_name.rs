//! # Garbage Collector Strategy Names
//!
//! Each function may specify a garbage collector strategy name, which
//! is simply a string.
//!
//! The supported values of name includes those built in to LLVM and any
//! provided by loaded plugins. Specifying a GC strategy will cause the
//! compiler to alter its output in order to support the named garbage
//! collection algorithm. Note that LLVM itself does not contain a
//! garbage collector, this functionality is restricted to generating
//! machine code which can interoperate with a collector provided
//! externally.
//!
//! https://llvm.org/docs/LangRef.html#garbage-collector-strategy-names

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct GcStrategyName(String);

impl std::fmt::Display for GcStrategyName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = format!("gc {}", self.0);
        write!(f, "{}", s)
    }
}
