#![doc= include_str!("../README.md")]

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub mod codegen;
mod config;
pub mod filesystem;
pub mod im;
pub mod parse;
pub mod type_check;

pub use crate::{
    config::{BuildConfig, Environment, EnvironmentStorage, FeatureFlags},
    im::Text,
};

pub type Error = std::sync::Arc<dyn std::error::Error + Send + Sync + 'static>;
