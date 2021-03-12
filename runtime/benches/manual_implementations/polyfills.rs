//! Substitutes for the `runic_types::wasm32` types which can be used for
//! benchmarking and manual implementations.

use std::marker::PhantomData;

use std::io::Cursor;
use hound::WavReader;
use anyhow::Error;
use rand::{
    Rng, SeedableRng, distributions::Standard, prelude::Distribution,
    rngs::SmallRng,
};
use rune_runtime::Environment;
use runic_types::{Buffer, Source, Transform, Value};
use tflite::{
    FlatBufferModel, Interpreter, InterpreterBuilder,
    ops::builtin::BuiltinOpResolver,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Random<T> {
    rng: SmallRng,
    _output_type: PhantomData<T>,
}

impl<T> Random<T> {
    pub fn from_env(env: &dyn Environment) -> Result<Self, Error> {
        let mut seed = <SmallRng as SeedableRng>::Seed::default();
        env.fill_random(&mut seed)?;

        Ok(Random {
            rng: SmallRng::from_seed(seed),
            _output_type: PhantomData,
        })
    }
}

impl<T> runic_types::Source for Random<T>
where
    Standard: Distribution<T>,
{
    type Output = T;

    fn generate(&mut self) -> Self::Output { self.rng.gen() }

    fn set_parameter(
        &mut self,
        _key: &str,
        _value: impl Into<Value>,
    ) -> &mut Self {
        self
    }
}

pub struct Model<Input, Output> {
    interpreter: Interpreter<'static, BuiltinOpResolver>,
    _type: PhantomData<fn(Input) -> Output>,
}

impl<In, Out, const M: usize, const N: usize> Model<[In; M], [Out; N]> {
    pub fn load(raw_blob: impl Into<Vec<u8>>) -> Result<Self, Error> {
        let model = FlatBufferModel::build_from_buffer(raw_blob.into())?;
        let resolver = BuiltinOpResolver::default();
        let builder = InterpreterBuilder::new(model, resolver)?;
        let mut interpreter = builder.build()?;
        interpreter.allocate_tensors()?;

        Ok(Model {
            interpreter,
            _type: PhantomData,
        })
    }
}

impl<In, Out> Transform<In> for Model<In, Out>
where
    Out: Buffer,
    In: Buffer,
{
    type Output = Out;

    fn transform(&mut self, input: In) -> Out {
        let input_index = self.interpreter.inputs()[0];
        let buffer = self.interpreter.tensor_buffer_mut(input_index).unwrap();

        unsafe {
            let raw_input = std::slice::from_raw_parts(
                input.as_ptr().cast::<u8>(),
                input.size_in_bytes(),
            );
            buffer.copy_from_slice(raw_input);
        }

        self.interpreter.invoke().unwrap();

        let mut output = Out::zeroed();

        unsafe {
            let output_index = self.interpreter.outputs()[0];
            let buffer = self.interpreter.tensor_buffer(output_index).unwrap();

            let raw_output = std::slice::from_raw_parts_mut(
                output.as_mut_ptr().cast::<u8>(),
                output.size_in_bytes(),
            );
            raw_output.copy_from_slice(buffer);
        }

        output
    }
}

#[derive(Debug, Clone)]
pub struct Accelerometer<const N: usize> {
    samples: Vec<[f32; 3]>,
}

impl<const N: usize> Accelerometer<N> {
    pub fn with_samples(csv: &str) -> Result<Self, Error> {
        let samples = crate::build::load_csv(csv)?;

        Ok(Accelerometer { samples })
    }
}

impl<const N: usize> Source for Accelerometer<N> {
    type Output = [[f32; 3]; N];

    fn generate(&mut self) -> Self::Output {
        let mut buffer = [[0.0; 3]; N];

        for (src, dest) in self.samples.iter().zip(&mut buffer) {
            *dest = *src;
        }

        buffer
    }

    fn set_parameter(
        &mut self,
        _key: &str,
        _value: impl Into<Value>,
    ) -> &mut Self {
        self
    }
}

#[derive(Debug, Clone)]
pub struct Sound<const N: usize> {
    samples: Vec<i16>,
}

impl<const N: usize> Sound<N> {
    pub fn from_wav_data(wav_data: &[u8]) -> Result<Self, Error> {
        let cursor = Cursor::new(wav_data);
        let reader = WavReader::new(cursor).unwrap();

        let samples = reader
            .into_samples::<i16>()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        Ok(Sound { samples })
    }
}

impl<const N: usize> Source for Sound<N> {
    type Output = [i16; N];

    fn generate(&mut self) -> Self::Output {
        let mut buffer = [0; N];

        for (src, dest) in self.samples.iter().zip(&mut buffer) {
            *dest = *src;
        }

        buffer
    }

    fn set_parameter(
        &mut self,
        _key: &str,
        _value: impl Into<Value>,
    ) -> &mut Self {
        self
    }
}
