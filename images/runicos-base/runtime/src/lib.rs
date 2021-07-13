mod image;

cfg_if::cfg_if! {
    // We can only compile the tflite crate on certain platflorms due to
    // https://github.com/boncheolgu/tflite-rs/issues/49
    if #[cfg(all(
            feature = "tflite",
            not(any(target_os = "android", target_os = "ios")),
        ))] {
        mod tflite_inference;
        pub(crate) use tflite_inference::initialize_model;
    } else {
        mod default_inference;
        pub(crate) use default_inference::initialize_model;
    }
}

use anyhow::Error;
pub use image::BaseImage;

pub trait Model: Send + Sync + 'static {
    fn infer(&mut self, input: &[u8], output: &mut [u8]) -> Result<(), Error>;
}
