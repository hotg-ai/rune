//! Parsing and analysis of Runefiles.

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

mod analysis;
pub mod ast;
mod diagnostics;
pub mod hir;
pub mod parse;
mod type_inference;

pub use analysis::analyse;
pub use diagnostics::Diagnostics;
pub use parse::parse;
