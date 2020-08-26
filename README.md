# iLang functional programming language 
[![CI Build Status](https://github.com/mrLSD/iLang/workflows/ci/badge.svg)](https://github.com/mrLSD/iLang/actions?query=workflow%3Aci)
[![Build Status](https://travis-ci.org/mrLSD/iLang.svg?branch=master)](https://travis-ci.org/mrLSD/iLang)
[![Coverage Status](https://coveralls.io/repos/github/mrLSD/iLang/badge.svg?branch=master)](https://coveralls.io/github/mrLSD/iLang?branch=master)

iLang is a functional programming language from scratch. 
In the current stage, it's a dialect from F#. It is not related to .NET or .NET Core platform.
Grammar based on EBNF, and described [here](grammar.md).

Current main goals of the project
* grammar creation
* AST implementation
* compiler creation based on LLVM

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
