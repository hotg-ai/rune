//! Parsing and analysis of Runefiles.

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[macro_use]
mod macros;

pub mod hir;
mod diagnostics;
pub mod hooks;
mod passes;
pub mod yaml;

pub use crate::{passes::build, diagnostics::Diagnostics};
