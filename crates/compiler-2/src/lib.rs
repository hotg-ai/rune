#![doc= include_str!("../README.md")]

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub mod diagnostics;
mod filesystem;
pub mod lowering;
pub mod parse;
mod text;
pub mod type_check;

pub use crate::{
    filesystem::{FileSystem, FileSystemError, FileSystemOperation},
    text::Text,
};
