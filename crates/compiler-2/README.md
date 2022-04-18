# The Rune Compiler

A compiler that compiles your data processing pipeline into a portable
WebAssembly binary.

## Architecture

The Rune compiler is base around [Salsa][salsa], a library for incremental
computation.  This lets us phrase the compilation process as a series of queries
(essentially, pure functions) which can be aggressively cached based on
dependency analysis.

- [`parse`] - Runefile parsing
- [`lowering`] - Convert a Runefile's AST into a high-level intermediate
  representation that is more amenable to analysis

[salsa]: https://github.com/salsa-rs/salsa
