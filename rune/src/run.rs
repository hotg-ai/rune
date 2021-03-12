use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    str::FromStr,
};
use anyhow::{Context, Error};
use hound::WavReader;
use log;
use rune_runtime::{DefaultEnvironment, Runtime};

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
            log::info!("Call {}", i);
            runtime.call().context("Call failed")?;
        }

        Ok(())
    }

    fn env(&self) -> Result<DefaultEnvironment, Error> {
        let mut env = DefaultEnvironment::default();

        if let Some(name) = self.name() {
            env.set_name(name);
        }

        for cap in &self.capabilities {
            match cap {
                Capability::RandomSeed { seed } => {
                    log::debug!("Setting the RNG's seed to {}", seed);
                    env.seed_rng(*seed);
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
                    env.set_random_data(random_bytes);
                },
                Capability::Accelerometer { filename } => {
                    log::debug!(
                        "Loading accelerator samples from \"{}\"",
                        filename.display()
                    );
                    let samples = load_accelerometer_data(filename)
                        .with_context(|| format!("Unable to load the accelerometer data from \"{}\"", filename.display()))?;

                    log::debug!(
                        "Loaded {} accelerometer samples from \"{}\"",
                        samples.len(),
                        filename.display()
                    );
                    env.set_accelerometer_data(samples);
                },
                Capability::Image { filename } => {
                    log::debug!(
                        "Loading an image from \"{}\"",
                        filename.display()
                    );
                    let img = image::open(filename).with_context(|| {
                        format!("Unable to load \"{}\"", filename.display())
                    })?;
                    env.set_image(img.to_rgb8());
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

                    env.set_sound(samples);
                },
            }
        }

        Ok(env)
    }

    fn name(&self) -> Option<&str> { self.rune.file_stem()?.to_str() }
}

#[derive(Debug, Clone, PartialEq)]
enum Capability {
    RandomSeed { seed: u64 },
    RandomData { filename: PathBuf },
    Accelerometer { filename: PathBuf },
    Image { filename: PathBuf },
    Sound { filename: PathBuf },
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

fn load_accelerometer_data(
    path: impl AsRef<Path>,
) -> Result<Vec<[f32; 3]>, Error> {
    let path = path.as_ref();

    let f = File::open(path)?;
    let reader = BufReader::new(f);

    let mut samples = Vec::new();
    let mut line_no = 1;

    let p = |word: &str, line: usize| {
        word.trim().parse::<f32>().with_context(|| {
            format!("Unable to parse \"{}\" as a float on line {}", word, line)
        })
    };

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let words: Vec<_> = line.split(",").collect();

        match words.as_slice() {
            [first, second, third] => {
                samples.push([
                    p(*first, line_no)?,
                    p(*second, line_no)?,
                    p(*third, line_no)?,
                ]);
            },
            _ => anyhow::bail!(
                "Expected a CSV with 3 columns, but line {} has {}",
                line_no,
                words.len(),
            ),
        }

        line_no += 1;
    }

    Ok(samples)
}
