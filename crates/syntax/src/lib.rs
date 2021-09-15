//! The Rune compiler.

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[macro_use]
mod macros;

mod build_context;
pub mod codegen;
mod diagnostics;
pub mod hir;
pub mod hooks;
pub mod lowering;
pub mod parse;
mod phases;
pub mod type_check;

pub use crate::{
    phases::{build, build_with_hooks, Phase},
    diagnostics::Diagnostics,
    build_context::{BuildContext, Verbosity},
};
