/// A consumer of data.
pub trait Sink<Input> {
    fn consume(&mut self, input: Input);
}

pub trait HasOutputs {
    fn set_output_dimensions(&mut self, _dimensions: &[usize]) {}
}
