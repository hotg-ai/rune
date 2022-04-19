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
- [`lowering`] - Convert a Runefile's AST into a high-level intermediate
  representation that is more amenable to analysis
- [`type_check`] - Construct a fully-defined pipeline based on the high-level
  intermediate representation

## Error Handling

Users are guaranteed to write buggy code, so the Rune compiler will try as hard
as it can to record these errors and continue on.

All queries that may encounter an error should return a value and a set of
[`diagnostics::Diagnostics`] as a tuple.

[salsa]: https://github.com/salsa-rs/salsa
