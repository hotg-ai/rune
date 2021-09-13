//! Parsing and analysis of Runefiles.

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[macro_use]
mod utils;

mod diagnostics;
pub mod hir;
mod passes;
pub mod yaml;

pub use crate::{passes::analyse, diagnostics::Diagnostics};
