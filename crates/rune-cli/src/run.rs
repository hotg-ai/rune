use std::{collections::HashMap, path::PathBuf, str::FromStr};

use anyhow::{Context, Error};
use hotg_rune_runtime::{
    builtins::{self, Arguments, AudioClip},
    NodeMetadata, Runtime,
};
use once_cell::sync::Lazy;
use regex::Regex;
use structopt::StructOpt;
use strum::VariantNames;

#[derive(Debug, Clone, PartialEq, StructOpt)]
pub struct Run {
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
        help = "A file who's bytes will be returned as-is by the RAW \
                capability"
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
        help = "The WebAssembly engine to use",
        possible_values = Engine::VARIANTS,
        default_value = "wasmer",
    )]
    engine: Engine,
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

        let mut runtime: Runtime = self
            .load_runtime(&rune)
            .context("Unable to load the Runtime")?;

        let caps = runtime.capabilities().clone();
        *runtime.input_tensors() = self.load_inputs(caps)?;

        runtime.predict().context("Prediction failed")?;

        let outputs = runtime.output_tensors();
        dbg!(outputs);

        Ok(())
    }

    fn load_inputs(
        self,
        caps: HashMap<u32, NodeMetadata>,
    ) -> Result<HashMap<u32, hotg_rune_runtime::Tensor>, Error> {
        let mut inputs = HashMap::new();

        for (id, metadata) in caps {
            let NodeMetadata {
                kind, arguments, ..
            } = metadata;
            let args = Arguments(arguments);

            let tensor = self.load_input(&kind, &args).with_context(|| {
                format!("Unable to load the \"{}\" input", kind)
            })?;

            inputs.insert(id, tensor);
        }

        Ok(inputs)
    }

    fn load_input(
        &self,
        kind: &str,
        args: &Arguments,
    ) -> Result<hotg_rune_runtime::Tensor, Error> {
        match kind {
            "IMAGE" => builtins::source(&self.image, args)
                .and_then(|path| {
                    image::open(path).with_context(|| {
                        format!("Unable to read \"{}\"", path.display())
                    })
                })
                .and_then(|img| builtins::image(args, &img)),

            "SOUND" => builtins::source(&self.sound, args)
                .and_then(|path| AudioClip::from_wav_file(path))
                .and_then(|audio| builtins::sound(args, &audio)),

            "RAW" => builtins::source(&self.raw, args)
                .and_then(|path| {
                    std::fs::read_to_string(path).with_context(|| {
                        format!("Unable to read \"{}\"", path.display())
                    })
                })
                .and_then(|audio| builtins::raw(args, &audio)),

            _ => anyhow::bail!("Unknown input type, \"{}\"", kind),
        }
    }

    pub(crate) fn load_runtime(&self, rune: &[u8]) -> Result<Runtime, Error> {
        match self.engine {
            Engine::Wasm3 => Runtime::wasm3(rune),
            Engine::Wasmer => Runtime::wasmer(rune),
        }
    }
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

#[derive(
    Debug, Copy, Clone, PartialEq, strum::EnumVariantNames, strum::EnumString,
)]
#[strum(serialize_all = "kebab-case")]
enum Engine {
    Wasm3,
    Wasmer,
}
