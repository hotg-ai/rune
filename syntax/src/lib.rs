//! Parsing and analysis of Runefiles.

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub mod ast;
pub mod parse;

pub use parse::parse;
