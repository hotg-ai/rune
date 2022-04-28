#![doc= include_str!("../README.md")]

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[macro_use]
mod macros;

pub mod codegen;
mod config;
pub mod diagnostics;
mod filesystem;
pub mod lowering;
pub mod parse;
pub mod proc_blocks;
mod text;
pub mod type_check;

use crate::diagnostics::{AsDiagnostic, DiagnosticMetadata};
pub use crate::{
    config::{BuildConfig, Environment, EnvironmentStorage},
    filesystem::{FileSystem, FileSystemError, FileSystemOperation},
    text::Text,
};

/// Get the [`DiagnosticMetadata`] for all known diagnostics.
pub fn all_diagnostics() -> Vec<DiagnosticMetadata> {
    vec![
        crate::lowering::diagnostics::DuplicateName::meta(),
        crate::lowering::diagnostics::NotAResource::meta(),
        crate::lowering::diagnostics::PathAndInlineNotAllowed::meta(),
        crate::lowering::diagnostics::ResourceUsedAsInput::meta(),
        crate::lowering::diagnostics::UnknownAbi::meta(),
        crate::lowering::diagnostics::UnknownInput::meta(),
        crate::lowering::diagnostics::UnknownResource::meta(),
        crate::parse::ParseFailed::meta(),
    ]
}
