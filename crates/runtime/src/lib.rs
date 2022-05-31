//! The Rune Runtime.
//!
//! # Feature Flags
//!
//! This crate makes use of cargo features to selectively enable or disable
//! functionality.
//!
//! The following cargo features are available:
//!
//! - `builtins` - (default) enable various builtin outputs and capabilities
#![cfg_attr(not(feature = "builtins"), doc = "(disabled)")]
//! - `tflite` - (default) enable support for TensorFlow Lite models
#![cfg_attr(not(feature = "tflite"), doc = "(disabled)")]
//! - `wasm3` - enable the [WASM3](https://github.com/wasm3/wasm3) engine
#![cfg_attr(not(feature = "wasm3"), doc = "(disabled)")]
//! - `wasmer` - enable the [wasmer](https://wasmer.io/) engine
#![cfg_attr(not(feature = "wasmer"), doc = "(disabled)")]
#![cfg_attr(feature = "unstable_doc_cfg", feature(doc_cfg))]

#[cfg(feature = "wasm3")]
pub extern crate wasm3;
#[cfg(feature = "wasmer")]
pub extern crate wasmer;

mod callbacks;
mod engine;
pub mod models;
mod runtime;
mod tensor;

#[cfg(feature = "builtins")]
pub mod builtins;
mod outputs;

pub mod zune;

pub use crate::{
    callbacks::{Model, ModelMetadata, NodeMetadata},
    engine::LoadError,
    outputs::OutputTensor,
    runtime::Runtime,
    tensor::{ElementType, Tensor, TensorElement}
};
