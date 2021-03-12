mod polyfills;

use anyhow::Error;
use polyfills::{Model, Random, Accelerometer};
use runic_types::{Source, Transform};
use modulo::Modulo;
use rune_runtime::{DefaultEnvironment};
use gesture_agg::GestureAgg;
use normalize::Normalize;

use crate::build::{RING, SLOPE, WING};

const SINE_MODEL: &[u8] =
    include_bytes!("../../../examples/sine/sinemodel.tflite");
const GESTURE_MODEL: &[u8] =
    include_bytes!("../../../examples/gesture/model.tflite");

pub struct ManualSine {
    random: Random<[f32; 1]>,
    modulo: Modulo<f32>,
    model: Model<[f32; 1], [f32; 1]>,
}

impl ManualSine {
    pub fn load() -> Self {
        let env = DefaultEnvironment::default();

        let random = Random::from_env(&env).unwrap();
        let modulo = Modulo::default().with_modulus(360.0);
        let model = Model::load(SINE_MODEL).unwrap();

        ManualSine {
            random,
            modulo,
            model,
        }
    }

    pub fn call(&mut self) -> [f32; 1] {
        let input = self.random.generate();
        let normalized = self.modulo.transform(input);
        let result = self.model.transform(normalized);

        result
    }
}

pub struct ManualGesture {
    accelerometer: Accelerometer<128>,
    model: Model<[[f32; 3]; 128], [f32; 4]>,
    gesture_agg: GestureAgg<4>,
    normalize: Normalize<[[f32; 3]; 128]>,
}

impl ManualGesture {
    pub fn wing() -> Self { ManualGesture::load(WING).unwrap() }

    pub fn ring() -> Self { ManualGesture::load(RING).unwrap() }

    pub fn slope() -> Self { ManualGesture::load(SLOPE).unwrap() }

    fn load(samples: &str) -> Result<Self, Error> {
        let accelerometer = Accelerometer::with_samples(samples)?;
        let model = Model::load(GESTURE_MODEL)?;
        let gesture_agg = GestureAgg::default()
            .with_labels(["Wing", "Ring", "Slope", "Unknown"]);
        let normalize = Normalize::default();

        Ok(ManualGesture {
            accelerometer,
            model,
            gesture_agg,
            normalize,
        })
    }

    pub fn call(&mut self) -> &'static str {
        let data: [[f32; 3]; 128] = self.accelerometer.generate();
        let data: [[f32; 3]; 128] = self.normalize.transform(data);
        let data: [f32; 4] = self.model.transform(data);
        self.gesture_agg.transform(data)
    }
}
