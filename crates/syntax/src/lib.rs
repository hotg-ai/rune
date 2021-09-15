//! The Rune compiler.
//!
//! # Phases
//!
//! The compilation process is split into several phases which build on each
//! other, with user-injectable [`hooks`] called after each phase finishes.
//!
//! The phases are:
//!
//! 1. [`parse`]
//! 2. [`lowering`]
//! 3. [`type_check`]
//! 4. [`codegen`]
//!
//! # Stability
//!
//! This crate contains the internal types used by the Rune compiler so they can
//! be used externally. While this can give you a lot of flexibility and let you
//! extract a lot of information about a Rune, the compiler is a continually
//! evolving codebase.
//!
//! **This API should be considered unstable and subject to change.**

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[macro_use]
mod macros;

mod build_context;
pub mod codegen;
mod diagnostics;
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
