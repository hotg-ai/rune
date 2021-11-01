//! The runtime component of the `runicos/base` image.
//!
//! # Feature Flags
//!
//! This crate has the following cargo feature flags:
//!
//! - `tensorflow-lite` - Implement inference of TensorFlow Lite models through
//!   the [`hotg_runecoral`] crate
//! - `wasm3-runtime` - Allow [`BaseImage`] to be used with the WASM3 runtime
//! - `wasmer-runtime` - Allow [`BaseImage`] to be used with the Wasmer runtime

#![cfg_attr(feature = "unstable_doc_cfg", feature(doc_cfg))]

#[cfg(feature = "tensorflow-lite")]
pub mod tensorflow_lite;
#[cfg(feature = "wasm3-runtime")]
mod wasm3_impl;
#[cfg(feature = "wasmer-runtime")]
mod wasmer_impl;


mod image;
mod random;

pub use crate::{
    image::{
        BaseImage, Model, ModelFactory, CapabilityFactory, OutputFactory,
        ResourceFactory,
    },
    random::Random,
};
