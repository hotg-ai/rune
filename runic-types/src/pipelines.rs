use crate::Value;

/// A stream of data, typically something like a random number generator or
/// sensor.
pub trait Source: HasOutputs {
    type Output;

    fn generate(&mut self) -> Self::Output;

    fn set_parameter(
        &mut self,
        key: &str,
        value: impl Into<Value>,
    ) -> &mut Self;
}

/// Process some data, transforming it from one form to another.
pub trait Transform<Input>: HasOutputs {
    type Output;

    fn transform(&mut self, input: Input) -> Self::Output;
}

/// A consumer of data.
pub trait Sink<Input> {
    fn consume(&mut self, input: Input);
}

pub trait HasOutputs {
    fn set_output_dimensions(&mut self, _dimensions: &[usize]) {}
}
