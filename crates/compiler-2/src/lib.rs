#![doc= include_str!("../README.md")]

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub mod codegen;
mod config;
mod filesystem;
pub mod parse;
pub mod proc_blocks;
mod text;
pub mod type_check;

pub use crate::{
    config::{BuildConfig, Environment, EnvironmentStorage},
    filesystem::{FileSystem, FileSystemError, FileSystemOperation},
    text::Text,
};
