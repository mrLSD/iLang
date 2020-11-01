//! # Thread Local Storage Models
//!
//! A variable may be defined as thread_local, which means that it will
//! not be shared by threads (each thread will have a separated copy
//! of the variable). Not all targets support thread-local variables.
//! Optionally, a TLS model may be specified.
//!
//! If no explicit model is given, the “general dynamic” model is used.
//!
//! https://llvm.org/docs/LangRef.html#thread-local-storage-models

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ThreadLocalStorage {
    LocalDynamic,
    InitialExec,
    LocalExec,
}

impl std::fmt::Display for ThreadLocalStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            ThreadLocalStorage::LocalDynamic => "thread_local(localdynamic)",
            ThreadLocalStorage::InitialExec => "thread_local(initialexec)",
            ThreadLocalStorage::LocalExec => "thread_local(localexec)",
        };

        write!(f, "{}", s)
    }
}
