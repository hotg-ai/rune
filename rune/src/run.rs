use std::{fs::File, path::PathBuf, str::FromStr};
use anyhow::{Context, Error};
use hound::WavReader;
use log;
use rune_runtime::common_capabilities::{Accelerometer, Image, Random, Raw, Sound};
use rune_wasmer_runtime::Runtime;
use runicos_base::BaseImage;

#[derive(Debug, Clone, PartialEq, structopt::StructOpt)]
pub struct Run {
    /// The Rune to run.
    rune: PathBuf,
    /// The number of times to execute this rune
    #[structopt(short, long, default_value = "1")]
    repeats: usize,
    /// Pass information to a capability as `key:value` pairs.
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
}

impl Run {
    pub fn execute(self) -> Result<(), Error> {
        log::info!("Running rune: {}", self.rune.display());

        let rune = std::fs::read(&self.rune).with_context(|| {
            format!("Unable to read \"{}\"", self.rune.display())
        })?;

        let env = self.env().context("Unable to initialize the environment")?;

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

    fn env(&self) -> Result<BaseImage, Error> {
        initialize_image(&self.capabilities)
    }
}

fn initialize_image(capabilities: &[Capability]) -> Result<BaseImage, Error> {
    let mut env = BaseImage::default();

    for cap in capabilities {
        match cap {
            Capability::RandomSeed { seed } => {
                log::debug!("Setting the RNG's seed to {}", seed);
                let seed = *seed;
                env.with_rand(move || Ok(Box::new(Random::seeded(seed))));
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
                env.with_rand(move || {
                    Ok(Box::new(Random::with_repeated_data(
                        random_bytes.clone(),
                    )))
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
                env.with_accelerometer(move || Ok(Box::new(acc.clone())));
            },
            Capability::Image { filename } => {
                log::debug!("Loading an image from \"{}\"", filename.display());
                let img = image::open(filename).with_context(|| {
                    format!("Unable to load \"{}\"", filename.display())
                })?;
                let img = img.to_rgb8();
                env.with_image(move || Ok(Box::new(Image::new(img.clone()))));
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

                env.with_sound(move || {
                    Ok(Box::new(Sound::new(samples.clone())))
                });
            },
            Capability::Raw { filename } => {
                let bytes = std::fs::read(filename).with_context(|| {
                    format!(
                        "Unable to open \"{}\" for reading",
                        filename.display()
                    )
                })?;
                env.with_raw(move || Ok(Box::new(Raw::new(bytes.clone()))));
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
