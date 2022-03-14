//! Functions for handling various "well-known" model formats.

#[cfg(feature = "tflite")]
mod tflite;

use anyhow::Error;
pub use hotg_rune_core::{TFJS_MIMETYPE, TFLITE_MIMETYPE, TF_MIMETYPE};

#[cfg(feature = "tflite")]
pub use self::tflite::load_tflite;
use crate::callbacks::{Model, ModelMetadata};

/// A model handler which will try to load a model based on the feature flags
/// that have been set.
///
/// Supported formats are:
/// - TensorFlow Lite
#[cfg_attr(not(feature = "tflite"), doc("(not supported)"))]
pub fn default_model_handler(
    _id: u32,
    meta: &ModelMetadata<'_>,
    model: &[u8],
) -> Result<Box<dyn Model>, Error> {
    let ModelMetadata {
        mimetype,
        inputs,
        outputs,
        ..
    } = *meta;

    match mimetype {
        #[cfg(feature = "tflite")]
        TFLITE_MIMETYPE => load_tflite(model, inputs, outputs),
        _ => Err(UnsupportedModelFormat::new(mimetype).into()),
    }
}

/// The error returned when the model handler can't handle a particular model
/// format.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
#[error("The \"{}\" format isn't supported", mimetype)]
pub struct UnsupportedModelFormat {
    /// The model's "mimetype".
    pub mimetype: String,
}

impl UnsupportedModelFormat {
    pub fn new(mimetype: impl Into<String>) -> Self {
        Self {
            mimetype: mimetype.into(),
        }
    }
}
