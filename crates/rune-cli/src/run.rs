use std::{fs::File, path::PathBuf, str::FromStr};
use anyhow::{Context, Error};
use hound::WavReader;
use log;
use rune_core::capabilities;
use rune_runtime::{
    common_capabilities::{Accelerometer, Image, Random, Raw, Sound},
};
use rune_wasmer_runtime::Runtime;
use runicos_base_runtime::BaseImage;

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
    accelerometer: Option<Vec<PathBuf>>,
    #[structopt(
        long,
        parse(from_os_str),
        help = "A WAV file containing samples returned by the SOUND capability"
    )]
    sound: Option<Vec<PathBuf>>,
    #[structopt(
        long,
        aliases = &["img"],
        parse(from_os_str),
        help = "An image to be returned by the IMAGE capability"
    )]
    image: Option<Vec<PathBuf>>,
    #[structopt(
        long,
        parse(from_os_str),
        help = "A file who's bytes will be returned as-is by the RAW capability"
    )]
    raw: Option<Vec<PathBuf>>,
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

        let capabilities = self.all_capabilities();
        let env = initialize_image(&capabilities)
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

    fn all_capabilities(&self) -> Vec<Capability> {
        let Run {
            capabilities,
            accelerometer,
            sound,
            image,
            raw,
            random,
            rune: _,
            repeats: _,
        } = self;
        let mut caps = capabilities.clone();

        if let Some(accel) = accelerometer {
            extend_caps(&mut caps, accel, |p| Capability::accel(p));
        }
        if let Some(sound) = sound {
            extend_caps(&mut caps, sound, |p| Capability::sound(p));
        }
        if let Some(raw) = raw {
            extend_caps(&mut caps, raw, |p| Capability::raw(p));
        }
        if let Some(image) = image {
            extend_caps(&mut caps, image, |p| Capability::image(p));
        }
        if let Some(random) = random {
            caps.push(Capability::RandomSeed { seed: *random });
        }

        caps
    }
}

fn extend_caps<'a, I, F, T>(
    capabilities: &mut Vec<Capability>,
    items: I,
    mut map: F,
) where
    I: IntoIterator<Item = &'a T> + 'a,
    T: 'a,
    F: FnMut(&T) -> Capability,
{
    for item in items {
        capabilities.push(map(item));
    }
}

fn initialize_image(capabilities: &[Capability]) -> Result<BaseImage, Error> {
    let mut env = BaseImage::with_defaults();

    for cap in capabilities {
        match cap {
            Capability::RandomSeed { seed } => {
                log::debug!("Setting the RNG's seed to {}", seed);
                let seed = *seed;
                env.register_capability(capabilities::RAND, move || {
                    Ok(Box::new(Random::seeded(seed))
                        as Box<dyn rune_runtime::Capability>)
                });
            },
            Capability::RandomData { filename } => {
                log::debug!(
                    "Loading some \"random\" data from \"{}\"",
                    filename.display()
                );
                let random_bytes =
                    std::fs::read(filename).with_context(|| {
                        format!(
                            "Unable to load random data from \"{}\"",
                            filename.display()
                        )
                    })?;
                anyhow::ensure!(
                    !random_bytes.is_empty(),
                    "The random data file was empty"
                );
                env.register_capability(capabilities::RAND, move || {
                    Ok(Box::new(Random::with_repeated_data(
                        random_bytes.clone(),
                    ))
                        as Box<dyn rune_runtime::Capability>)
                });
            },
            Capability::Accelerometer { filename } => {
                log::debug!(
                    "Loading accelerator samples from \"{}\"",
                    filename.display()
                );
                let csv =
                    std::fs::read_to_string(filename).with_context(|| {
                        format!("Unable to read \"{}\"", filename.display())
                    })?;

                let acc = Accelerometer::from_csv(&csv)
                    .context("Unable to parse the samples")?;
                log::debug!(
                    "Loaded {} accelerometer samples from \"{}\"",
                    acc.samples().len(),
                    filename.display()
                );
                env.register_capability(capabilities::ACCEL, move || {
                    Ok(Box::new(acc.clone())
                        as Box<dyn rune_runtime::Capability>)
                });
            },
            Capability::Image { filename } => {
                log::debug!("Loading an image from \"{}\"", filename.display());
                let img = image::open(filename).with_context(|| {
                    format!("Unable to load \"{}\"", filename.display())
                })?;
                env.register_capability(capabilities::IMAGE, move || {
                    Ok(Box::new(Image::new(img.clone()))
                        as Box<dyn rune_runtime::Capability>)
                });
            },
            Capability::Sound { filename } => {
                let f = File::open(filename).with_context(|| {
                    format!(
                        "Unable to open \"{}\" for reading",
                        filename.display()
                    )
                })?;
                let reader = WavReader::new(f)
                    .context("Unable to read the WAV file's header")?;

                let samples = reader
                    .into_samples::<i16>()
                    .collect::<Result<Vec<_>, _>>()
                    .context("Unable to parse the WAV file's samples")?;

                env.register_capability(capabilities::SOUND, move || {
                    Ok(Box::new(Sound::new(samples.clone()))
                        as Box<dyn rune_runtime::Capability>)
                });
            },
            Capability::Raw { filename } => {
                let bytes = std::fs::read(filename).with_context(|| {
                    format!(
                        "Unable to open \"{}\" for reading",
                        filename.display()
                    )
                })?;
                env.register_capability(capabilities::RAW, move || {
                    Ok(Box::new(Raw::new(bytes.clone()))
                        as Box<dyn rune_runtime::Capability>)
                });
            },
        }
    }

    Ok(env)
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

impl Capability {
    fn accel(filename: impl Into<PathBuf>) -> Self {
        Capability::Accelerometer {
            filename: filename.into(),
        }
    }

    fn image(filename: impl Into<PathBuf>) -> Self {
        Capability::Image {
            filename: filename.into(),
        }
    }

    fn sound(filename: impl Into<PathBuf>) -> Self {
        Capability::Sound {
            filename: filename.into(),
        }
    }

    fn raw(filename: impl Into<PathBuf>) -> Self {
        Capability::Raw {
            filename: filename.into(),
        }
    }
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
