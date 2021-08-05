use std::{path::PathBuf, str::FromStr};
use anyhow::{Context, Error};
use log;
use hotg_rune_core::capabilities;
use crate::run::{
    Accelerometer, Image, Raw, Sound, accelerometer::Samples,
    image::ImageSource, new_multiplexer, sound::AudioClip,
};
use hotg_rune_wasmer_runtime::Runtime;
use hotg_runicos_base_runtime::{BaseImage, Random};

#[derive(Debug, Clone, PartialEq, structopt::StructOpt)]
pub struct Run {
    /// The Rune to run.
    rune: PathBuf,
    /// The number of times to execute this rune
    #[structopt(short, long, default_value = "1")]
    repeats: usize,
    /// Initialize capabilities based on `key:value` pairs. Prefer to use
    /// aliases like "--image" and "--sound" when the capability is builtin.
    ///
    /// For example:
    ///
    /// - `random:42` seeds the random number generator with `42`
    ///
    /// - `random:random_bytes.bin` provides data to be returned by the random
    ///   number generator
    ///
    /// - `accel:samples.csv` is a CSV file containing `[X, Y, Z]` vectors to
    ///   be returned by the accelerometer
    ///
    /// - `sound:audio.wav` is a WAV file containing samples returned by the
    ///   sound capability
    ///
    /// - `image:person.png` is an image file that will be returned by the
    ///   image capability
    ///
    /// - `raw:data.bin` is a file who's bytes will be used as-is
    #[structopt(short, long = "capability")]
    capabilities: Vec<Capability>,
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
        macro_rules! chain_capabilities {
            ($collection:expr, $capabilities:expr, $pattern:pat => $value:ident $(,)?) => {
                $collection
                    .iter()
                    .chain($capabilities.iter().filter_map(|c| match c {
                        $pattern => Some($value),
                        _ => None,
                    }))
            };
        }

        let Run {
            capabilities,
            accelerometer,
            sound,
            image,
            raw,
            random,
            ..
        } = self;

        if !capabilities.is_empty() {
            log::warn!("The \"--capability\" flag has been deprecated in favour of the more auto-complete friendly variants.");
            log::warn!("For example, use \"--image person.png\" instead of \"--capability image:person.png\"");
        }

        let accelerometer = chain_capabilities!(
            accelerometer,
            capabilities,
            Capability::Accelerometer { filename } => filename,
        );
        let accelerometer = accelerometer
            .map(|f| Samples::from_csv_file(f))
            .collect::<Result<Vec<_>, Error>>()?;

        let sound = chain_capabilities!(
            sound,
            capabilities,
            Capability::Sound { filename } => filename,
        );
        let sound = sound
            .map(|f| AudioClip::from_wav_file(f))
            .collect::<Result<Vec<_>, Error>>()?;

        let image = chain_capabilities!(
            image,
            capabilities,
            Capability::Image { filename } => filename,
        );
        let image = image
            .map(|f| ImageSource::from_file(f))
            .collect::<Result<Vec<_>, Error>>()?;

        let raw = chain_capabilities!(
            raw,
            capabilities,
            Capability::Raw { filename } => filename,
        );
        let raw = raw
            .map(|f| {
                std::fs::read(&f).with_context(|| {
                    format!("Unable to read \"{}\"", f.display())
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;

        let mut img = BaseImage::with_defaults();

        img.register_capability(
            capabilities::IMAGE,
            new_multiplexer::<Image, _>(image),
        )
        .register_capability(
            capabilities::SOUND,
            new_multiplexer::<Sound, _>(sound),
        )
        .register_capability(
            capabilities::ACCEL,
            new_multiplexer::<Accelerometer, _>(accelerometer),
        )
        .register_capability(capabilities::RAW, new_multiplexer::<Raw, _>(raw));

        if let Some(seed) = *random {
            img.register_capability(capabilities::RAND, move || {
                Ok(Box::new(Random::seeded(seed))
                    as Box<dyn hotg_rune_runtime::Capability>)
            });
        }

        Ok(img)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Capability {
    RandomSeed { seed: u64 },
    RandomData { filename: PathBuf },
    Accelerometer { filename: PathBuf },
    Image { filename: PathBuf },
    Sound { filename: PathBuf },
    Raw { filename: PathBuf },
}

impl FromStr for Capability {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let (key, value) = s
            .split_once(":")
            .context("Capabilities are in the form `key:value`")?;

        match key {
            "r" | "rand" | "random" => {
                match value.parse() {
                // it might be an integer seed
                    Ok(seed) => Ok(Capability::RandomSeed { seed}),
                // otherwise it's pointing us to a file containing "random" data
                    Err(_) => Ok(Capability::RandomData { filename: PathBuf::from(value )})
                }
            },
            "a" | "acc" | "accel" | "accelerometer" => {
                Ok(Capability::Accelerometer {
                    filename: PathBuf::from(value),
                })
            },
            "i" | "img" | "image" => {
                Ok(Capability::Image { filename: PathBuf::from(value) })
            }
            "w" | "raw" => {
                Ok(Capability::Raw { filename: PathBuf::from(value) })
            }
            "s" | "sound" | "wav" => {
                Ok(Capability::Sound { filename: PathBuf::from(value) })
            },
            other => anyhow::bail!(
                "Supported capabilities are \"random\" and \"accelerometer\", found \"{}\"",
                other,
            ),
        }
    }
}
