#![doc= include_str!("../README.md")]

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[macro_use]
mod macros;

pub mod diagnostics;
mod filesystem;
pub mod lowering;
pub mod parse;
mod text;

pub use crate::{
    filesystem::{FileSystem, ReadError},
    text::Text,
};
