use std::path::PathBuf;
use anyhow::{Context, Error};
use log;
use hotg_rune_core::capabilities;
use crate::run::{
    Accelerometer, Image, Raw, Sound, accelerometer::Samples,
    image::ImageSource, multi::SourceBackedCapability, new_capability_switcher,
    runecoral_inference, sound::AudioClip,
};
use hotg_rune_wasmer_runtime::Runtime;
use hotg_runicos_base_runtime::{BaseImage, CapabilityFactory, Random};

#[derive(Debug, Clone, PartialEq, structopt::StructOpt)]
pub struct Run {
    /// The Rune to run.
    rune: PathBuf,
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
        env,
        help = "The librunecoral.so library to use for hardware acceleration"
    )]
    librunecoral: Option<PathBuf>,
}

impl Run {
    pub fn execute(self) -> Result<(), Error> {
        log::info!("Running rune: {}", self.rune.display());

        let rune = std::fs::read(&self.rune).with_context(|| {
            format!("Unable to read \"{}\"", self.rune.display())
        })?;

        let env = self
            .initialize_image()
            .context("Unable to initialize the environment")?;

        let mut runtime = Runtime::load(&rune, env)
            .context("Unable to initialize the virtual machine")?;

        for i in 0..self.repeats {
            if i > 0 {
                log::info!("Call {}", i + 1);
            }
            runtime.call().context("Call failed")?;
        }

        Ok(())
    }

    fn initialize_image(&self) -> Result<BaseImage, Error> {
        // Load the source files for each kind of Capability and create a
        // CapabilityFactory which will instantiate Capabilities depending on
        // the "source" index provided as a capability parameter.
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

        let mut img = BaseImage::with_defaults();

        img.register_capability(capabilities::ACCEL, accelerometer)
            .register_capability(capabilities::IMAGE, image)
            .register_capability(capabilities::RAW, raw)
            .register_capability(capabilities::SOUND, sound);

        runecoral_inference::override_model_handler(
            &mut img,
            self.librunecoral.as_deref(),
        )
        .context("Unable to register the librunecoral inference backend")?;

        if let Some(seed) = self.random {
            img.register_capability(capabilities::RAND, move || {
                Ok(Box::new(Random::seeded(seed))
                    as Box<dyn hotg_rune_runtime::Capability>)
            });
        }

        Ok(img)
    }
}

/// Create a new [`CapabilityFactory`] which uses the `"source"` parameter set
/// by a Rune at runtime to switch between inputs within the same type of
/// capability.
///
/// For example, imagine passing the path for 3 images to the `rune` CLI.
/// Inside the Rune, we'll instantiate a [`Capability`] object and set the
/// `"source"` parameter to `1`. This then tells the [`CapabilityFactory`] that
/// we want to read from the second image.
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
