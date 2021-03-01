use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    str::FromStr,
};
use anyhow::{Context, Error};
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
    /// Supported keys are `random` for seeding the random number generator
    /// and `accelerometer` for providing accelerometer samples.
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
                Capability::Random { seed } => {
                    log::debug!("Setting the RNG's seed to {}", seed);
                    env.seed_rng(*seed);
                },
                Capability::Accelerometer { filename } => {
                    let samples = load_accelerometer_data(filename)
                        .with_context(|| format!("Unable to load the accelerometer data from \"{}\"", filename.display()))?;

                    log::debug!(
                        "Loaded {} accelerometer samples from \"{}\"",
                        samples.len(),
                        filename.display()
                    );
                    env.set_accelerometer_data(samples);
                },
            }
        }

        Ok(env)
    }

    fn name(&self) -> Option<&str> { self.rune.file_stem()?.to_str() }
}

#[derive(Debug, Clone, PartialEq)]
enum Capability {
    Random { seed: u64 },
    Accelerometer { filename: PathBuf },
}

impl FromStr for Capability {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let (key, value) = s
            .split_once(":")
            .context("Capabilities are in the form `key:value`")?;

        match key {
            "r" | "rand" | "random" => {
                let seed = value
                    .parse()
                    .context("Unable to parse the seed as an integer")?;
                Ok(Capability::Random { seed })
            },
            "a" | "acc" | "accel" | "accelerometer" => {
                Ok(Capability::Accelerometer {
                    filename: PathBuf::from(value),
                })
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
