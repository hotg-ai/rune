//! Parsing and analysis of Runefiles.

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

mod passes;
mod diagnostics;
pub mod hir;
mod utils;
pub mod yaml;

pub use crate::{passes::analyse, diagnostics::Diagnostics};
