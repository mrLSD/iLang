# iLang functional programming language 
[![CI Build Status](https://github.com/mrLSD/iLang/workflows/ci/badge.svg)](https://github.com/mrLSD/iLang/actions?query=workflow%3Aci)
[![Build Status](https://travis-ci.org/mrLSD/iLang.svg?branch=master)](https://travis-ci.org/mrLSD/iLang)
[![Coverage Status](https://coveralls.io/repos/github/mrLSD/iLang/badge.svg?branch=master)](https://coveralls.io/github/mrLSD/iLang?branch=master)

**iLang** is a functional programming language from scratch.
It is general purpose, strongly typed, multi-paradigm programming 
language that encompasses functional programming methods. 

**iLang** is a cross-platform and compiler based on LLVM. 

**iLang** corresponds as a member of the ML language family. 

Formal grammar based on EBNF, and it is described in the document [here](grammar.md).

Current main goals of the project
* [x] grammar creation
* [x] AST implementation
* [ ] compiler creation based on LLVM

Implementation based on *Rust language* 
to achieve the goals of reliability, efficiency, speed, memory safety.

Parsing based on [nom](https://crates.io/crates/nom) a parser combinators library. 

## Useful commands
* `cargo build` - build project
* `make` - run rust clippy tool - collection of lints to catch common mistakes.
* `make test` - run unit tests
* `make cover` - run coverage tests and generate test coverage report. It's used grcov. For installation run `cargo install grcov`
* `make fmt` - run code formatting.
* `make check` - run `cargo check`
 
## Licanse: MIT
