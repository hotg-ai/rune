use hotg_rune_runtime::Output;
use regex::Regex;
use std::{path::PathBuf, str::FromStr};
use anyhow::{Context, Error};
use log;
use hotg_rune_core::{Shape, capabilities};
use once_cell::sync::Lazy;
use crate::run::{
    Accelerometer, Image, Raw, Sound, accelerometer::Samples,
    image::ImageSource, multi::SourceBackedCapability, new_capability_switcher,
    resources, sound::AudioClip,
};
use hotg_runicos_base_runtime::{BaseImage, CapabilityFactory, Random};

#[derive(Debug, Clone, PartialEq, structopt::StructOpt)]
pub struct Run {
    /// The number of times to execute this rune
    #[structopt(short, long, default_value = "1")]
    repeats: usize,
    #[structopt(
        long = "accelerometer",
        aliases = &["accel"],
        parse(from_os_str),
        help = "A CSV file containing [X, Y, Z] vectors to be returned by the ACCEL capability"
    )]
    accelerometer: Vec<PathBuf>,
    #[structopt(
        long,
        parse(from_os_str),
        help = "A WAV file containing samples returned by the SOUND capability"
    )]
    sound: Vec<PathBuf>,
    #[structopt(
        long,
        aliases = &["img"],
        parse(from_os_str),
        help = "An image to be returned by the IMAGE capability"
    )]
    image: Vec<PathBuf>,
    #[structopt(
        long,
        parse(from_os_str),
        help = "A file who's bytes will be returned as-is by the RAW capability"
    )]
    raw: Vec<PathBuf>,
    #[structopt(
        long,
        aliases = &["rand"],
        help = "Seed the runtime's Random Number Generator"
    )]
    random: Option<u64>,
    #[structopt(
        long,
        help = "Use the wasm3 WebAssembly engine instead of Wasmer"
    )]
    wasm3: bool,
    #[structopt(
        long = "file-resource",
        parse(try_from_str),
        help = "Load a named resource from a file"
    )]
    file_resources: Vec<FileResource>,
    #[structopt(
        long = "string-resource",
        parse(try_from_str),
        help = "Use the provided string as a resource"
    )]
    string_resources: Vec<StringResource>,
    #[structopt(help = "The Rune to run")]
    rune: PathBuf,
}

impl Run {
    pub fn execute(self) -> Result<(), Error> {
        log::info!("Running rune: {}", self.rune.display());

        let rune = std::fs::read(&self.rune).with_context(|| {
            format!("Unable to read \"{}\"", self.rune.display())
        })?;

        let img = self.initialize_image(&rune)?;
        let mut call = self.load_runtime(&rune, img)?;

        log::info!("The Rune was loaded successfully");

        if cfg!(target_os = "macos") {
            let ticket = "https://github.com/tensorflow/tensorflow/issues/52300";
            log::warn!("TensorFlow Lite has a bug where its MacOS CPU backend will occasionally segfault during inference. See {} for more.", ticket);
        }

        for i in 0..self.repeats {
            if i > 0 {
                log::info!("Call {}", i + 1);
            }

            call().context("Call failed")?;
        }

        Ok(())
    }

    fn load_runtime(
        &self,
        rune: &[u8],
        img: BaseImage,
    ) -> Result<Box<dyn FnMut() -> Result<(), Error>>, Error> {
        if self.wasm3 {
            let mut runtime =
                hotg_rune_wasm3_runtime::Runtime::load(rune, img)
                    .context("Unable to initialize the virtual machine")?;

            Ok(Box::new(move || runtime.call()))
        } else {
            let mut runtime =
                hotg_rune_wasmer_runtime::Runtime::load(rune, img)
                    .context("Unable to initialize the virtual machine")?;

            Ok(Box::new(move || runtime.call()))
        }
    }

    fn initialize_image(&self, rune: &[u8]) -> Result<BaseImage, Error> {
        let mut img = BaseImage::with_defaults();

        self.register_capabilities(&mut img)?;
        self.register_outputs(&mut img);

        resources::load_from_custom_sections(&mut img, rune)?;
        resources::load_from_files(&mut img, &self.file_resources);
        resources::load_from_strings(&mut img, &self.string_resources);

        Ok(img)
    }

    fn register_outputs(&self, img: &mut BaseImage) {
        img.register_output(hotg_rune_core::outputs::SERIAL, serial_output);
    }

    /// Load the source files for each kind of capability and create a
    /// [`CapabilityFactory`] which will instantiate capabilities depending on
    /// the "source" index provided as a capability parameter.
    fn register_capabilities(&self, img: &mut BaseImage) -> Result<(), Error> {
        let accelerometer = capability_switcher::<Accelerometer, _, _>(
            &self.accelerometer,
            |p| Samples::from_csv_file(p),
        )?;
        let image = capability_switcher::<Image, _, _>(&self.image, |p| {
            ImageSource::from_file(p)
        })?;
        let raw = capability_switcher::<Raw, _, _>(&self.raw, |p| {
            std::fs::read(p)
                .with_context(|| format!("Unable to read \"{}\"", p.display()))
        })?;
        let sound = capability_switcher::<Sound, _, _>(&self.sound, |p| {
            AudioClip::from_wav_file(p)
        })?;

        img.register_capability(capabilities::ACCEL, accelerometer)
            .register_capability(capabilities::IMAGE, image)
            .register_capability(capabilities::RAW, raw)
            .register_capability(capabilities::SOUND, sound);

        if let Some(seed) = self.random {
            img.register_capability(capabilities::RAND, move || {
                Ok(Box::new(Random::seeded(seed))
                    as Box<dyn hotg_rune_runtime::Capability>)
            });
        }

        Ok(())
    }
}

/// Create a new [`CapabilityFactory`] which uses the `"source"` parameter set
/// by a Rune at runtime to switch between inputs within the same type of
/// capability.
///
/// For example, imagine passing the path for 3 images to the `rune` CLI.
/// Inside the Rune, we'll instantiate a [`SourceBackedCapability`] object and
/// set the `"source"` parameter to `1`. This then tells the
/// [`CapabilityFactory`] that we want to read from the second image.
fn capability_switcher<C, T, F>(
    items: &[T],
    mut make_source: F,
) -> Result<impl CapabilityFactory, Error>
where
    C: SourceBackedCapability,
    F: FnMut(&T) -> Result<C::Source, Error>,
{
    let mut sources = Vec::new();

    for item in items {
        let source = make_source(item)?;
        sources.push(source);
    }

    Ok(new_capability_switcher::<C, _>(sources))
}

fn parse_key_value_pair(s: &str) -> Result<(&str, &str), Error> {
    static PATTERN: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"([a-zA-Z_][a-zA-Z0-9]*)=(.*)").unwrap());

    let captures = PATTERN
        .captures(s)
        .context("Expected a resource in the form \"NAME=value\"")?;
    let key = captures.get(0).unwrap().as_str();
    let value = captures.get(1).unwrap().as_str();

    Ok((key, value))
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FileResource {
    pub name: String,
    pub path: PathBuf,
}

impl FromStr for FileResource {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Error> {
        let (name, path) = parse_key_value_pair(value)?;

        Ok(FileResource {
            name: name.to_string(),
            path: PathBuf::from(path),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StringResource {
    pub name: String,
    pub value: String,
}

impl FromStr for StringResource {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Error> {
        let (name, value) = parse_key_value_pair(value)?;

        Ok(StringResource {
            name: name.to_string(),
            value: value.to_string(),
        })
    }
}

fn serial_output(_: Option<&[Shape<'_>]>) -> Result<Box<dyn Output>, Error> {
    Ok(Box::new(Serial::default()))
}

#[derive(Debug, Clone, Copy, Default)]
struct Serial;

impl Output for Serial {
    fn consume(&mut self, buffer: &[u8]) -> Result<(), Error> {
        let json = std::str::from_utf8(buffer)
            .context("Unable to parse the input as UTF-8")?;

        println!("{}", json);

        Ok(())
    }
}
