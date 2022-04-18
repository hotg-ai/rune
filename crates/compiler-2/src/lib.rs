#![doc= include_str!("../README.md")]

#[macro_use]
mod macros;
pub mod parse;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
