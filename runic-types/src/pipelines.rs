/// Contextual information passed in by the runtime while executing a particular
/// pipeline.
pub struct PipelineContext {}

/// A stream of data, typically something like a random number generator or
/// sensor.
pub trait Source {
    type Output;

    fn generate(&mut self, ctx: &mut PipelineContext) -> Self::Output;
}

/// Process some data, transforming it from one form to another.
pub trait Transform<Input> {
    type Output;

    fn transform(
        &mut self,
        input: Input,
        ctx: &mut PipelineContext,
    ) -> Self::Output;
}

/// A consumer of data.
pub trait Sink<Input> {
    fn consume(&mut self, input: Input);
}
