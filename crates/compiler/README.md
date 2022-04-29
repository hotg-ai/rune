# The Rune Compiler

A compiler that compiles your data processing pipeline into a portable
WebAssembly binary.

## Architecture

The Rune compiler is base around [Salsa][salsa], a library for incremental
computation.  This lets us phrase the compilation process as a series of queries
(essentially, pure functions) which can be aggressively cached based on
dependency analysis.

These series of queries are broken up into a couple submodules,

- [`parse`] - Parse a Runefile written in the YAML format
- [`codegen`] - Generate the final Rune binary

[salsa]: https://github.com/salsa-rs/salsa
