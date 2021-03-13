use std::{io::Cursor, path::Path};
use hound::WavReader;
use anyhow::Error;
use once_cell::sync::Lazy;
use rune_codegen::Compilation;
use rune_runtime::{DefaultEnvironment, Runtime};
use rune_syntax::Diagnostics;
use wasmer::{Module, Store};

pub const SINE_RUNEFILE: &str = include_str!("../../../examples/sine/Runefile");
pub const GESTURE_RUNEFILE: &str =
    include_str!("../../../examples/gesture/Runefile");
pub const MICROSPEECH_RUNEFILE: &str =
    include_str!("../../../examples/microspeech/Runefile");

pub const WING: &str =
    include_str!("../../../examples/gesture/example_wing.csv");
pub const RING: &str =
    include_str!("../../../examples/gesture/example_ring.csv");
pub const SLOPE: &str =
    include_str!("../../../examples/gesture/example_slope.csv");

pub const YES: &[u8] = include_bytes!(
    "../../../examples/microspeech/data/yes_01d22d03_nohash_0.wav"
);

pub static SINE_DEBUG: Lazy<Vec<u8>> =
    Lazy::new(|| compile("sine", SINE_RUNEFILE, false));
pub static SINE_RELEASE: Lazy<Vec<u8>> =
    Lazy::new(|| compile("sine", SINE_RUNEFILE, true));
pub static GESTURE_DEBUG: Lazy<Vec<u8>> =
    Lazy::new(|| compile("gesture", GESTURE_RUNEFILE, false));
pub static GESTURE_RELEASE: Lazy<Vec<u8>> =
    Lazy::new(|| compile("gesture", GESTURE_RUNEFILE, true));
pub static MICROSPEECH_DEBUG: Lazy<Vec<u8>> =
    Lazy::new(|| compile("microspeech", MICROSPEECH_RUNEFILE, false));
pub static MICROSPEECH_RELEASE: Lazy<Vec<u8>> =
    Lazy::new(|| compile("microspeech", MICROSPEECH_RUNEFILE, true));

pub static SINE_DEBUG_MODULE: Lazy<Module> = Lazy::new(|| module(&SINE_DEBUG));
pub static SINE_RELEASE_MODULE: Lazy<Module> =
    Lazy::new(|| module(&SINE_RELEASE));
pub static GESTURE_DEBUG_MODULE: Lazy<Module> =
    Lazy::new(|| module(&GESTURE_DEBUG));
pub static GESTURE_RELEASE_MODULE: Lazy<Module> =
    Lazy::new(|| module(&GESTURE_RELEASE));
pub static MICROSPEECH_DEBUG_MODULE: Lazy<Module> =
    Lazy::new(|| module(&MICROSPEECH_DEBUG));
pub static MICROSPEECH_RELEASE_MODULE: Lazy<Module> =
    Lazy::new(|| module(&MICROSPEECH_RELEASE));

fn module(wasm: &[u8]) -> Module {
    let store = Store::default();
    Module::new(&store, wasm).unwrap()
}

pub fn compile(name: &str, runefile: &str, optimized: bool) -> Vec<u8> {
    let parsed = rune_syntax::parse(runefile).unwrap();

    let mut diags = Diagnostics::new();
    let rune = rune_syntax::analyse(0, &parsed, &mut diags);

    assert!(!diags.has_errors());

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let sine_dir = repo_root.join("examples").join(name);

    let working_dir = tempfile::tempdir().unwrap();

    let compilation = Compilation {
        name: String::from(name),
        rune,
        rune_project_dir: repo_root.to_path_buf(),
        current_directory: sine_dir,
        working_directory: working_dir.path().to_path_buf(),
        optimized,
    };

    rune_codegen::generate(compilation).unwrap()
}

pub fn load_csv(csv: &str) -> Result<Vec<[f32; 3]>, Error> {
    let mut samples = Vec::new();

    for line in csv.lines() {
        let words: Vec<_> = line.split(",").map(|s| s.trim()).collect();

        match words.as_slice() {
            [a, b, c] => samples.push([a.parse()?, b.parse()?, c.parse()?]),
            [] => {},
            _ => anyhow::bail!("Expected 3 columns, found {}", words.len()),
        }
    }

    Ok(samples)
}

#[derive(Default)]
pub struct EnvBuilder {
    env: DefaultEnvironment,
}

impl EnvBuilder {
    pub fn new() -> Self { EnvBuilder::default() }

    pub fn with_sound(mut self, wav_data: &[u8]) -> Self {
        let cursor = Cursor::new(wav_data);
        let reader = WavReader::new(cursor).unwrap();

        let samples = reader
            .into_samples::<i16>()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        self.env.set_sound(samples);

        self
    }

    pub fn yes(self) -> Self { self.with_sound(YES) }

    pub fn with_accelerometer(mut self, csv: &str) -> Self {
        let samples = load_csv(csv).unwrap();
        self.env.set_accelerometer_data(samples);

        self
    }

    pub fn wing(self) -> Self { self.with_accelerometer(WING) }

    pub fn ring(self) -> Self { self.with_accelerometer(RING) }

    pub fn slope(self) -> Self { self.with_accelerometer(SLOPE) }

    pub fn finish(self) -> DefaultEnvironment { self.env }
}

pub struct RuntimeBuilder {
    rune: Rune,
    env: EnvBuilder,
}

impl RuntimeBuilder {
    pub fn gesture() -> Self {
        RuntimeBuilder {
            rune: Rune::Undecided {
                debug: &GESTURE_DEBUG_MODULE,
                release: &*GESTURE_RELEASE_MODULE,
            },
            env: EnvBuilder::default(),
        }
    }

    pub fn sine() -> Self {
        RuntimeBuilder {
            rune: Rune::Undecided {
                debug: &*SINE_DEBUG_MODULE,
                release: &*SINE_RELEASE_MODULE,
            },
            env: EnvBuilder::default(),
        }
    }

    pub fn microspeech() -> Self {
        RuntimeBuilder {
            rune: Rune::Undecided {
                debug: &*MICROSPEECH_DEBUG_MODULE,
                release: &*MICROSPEECH_RELEASE_MODULE,
            },
            env: EnvBuilder::default(),
        }
    }

    pub fn debug(mut self) -> Self {
        match self.rune {
            Rune::Undecided { debug, .. } => {
                self.rune = Rune::Decided(debug);
            },
            _ => panic!("You can't decide debug/release twice"),
        }

        self
    }

    pub fn release(mut self) -> Self {
        match self.rune {
            Rune::Undecided { release, .. } => {
                self.rune = Rune::Decided(release);
            },
            _ => panic!("You can't decide debug/release twice"),
        }

        self
    }

    pub fn ring(mut self) -> Self {
        self.env = self.env.ring();
        self
    }

    pub fn wing(mut self) -> Self {
        self.env = self.env.wing();
        self
    }

    pub fn slope(mut self) -> Self {
        self.env = self.env.slope();
        self
    }

    pub fn yes(mut self) -> Self {
        self.env = self.env.yes();
        self
    }

    pub fn finish(self) -> Runtime {
        let RuntimeBuilder { rune, env } = self;

        let wasm = match rune {
            Rune::Undecided { .. } => {
                panic!("Please choose between debug/release")
            },
            Rune::Decided(wasm) => wasm,
        };

        let env = env.finish();
        let store = Store::default();

        Runtime::load_from_module(wasm, &store, env).unwrap()
    }
}

#[derive(Debug, Copy, Clone)]
enum Rune {
    Undecided {
        debug: &'static Module,
        release: &'static Module,
    },
    Decided(&'static Module),
}
