use legion::{Resources, World, systems::Runnable};
use crate::{
    BuildContext, codegen, compile,
    hooks::{Continuation, Ctx, Hooks},
    lowering, parse, type_check,
};

/// Execute the `rune build` process.
pub fn build(ctx: BuildContext) -> (World, Resources) {
    struct NopHooks;
    impl Hooks for NopHooks {}

    build_with_hooks(ctx, &mut NopHooks)
}

/// Execute the `rune build` process, passing in custom [`Hooks`] which will
/// be fired after each phase.
pub fn build_with_hooks(
    ctx: BuildContext,
    hooks: &mut dyn Hooks,
) -> (World, Resources) {
    let mut world = World::default();
    let mut res = Resources::default();

    res.insert(ctx);

    if hooks.before_parse(&mut c(&mut world, &mut res))
        != Continuation::Continue
    {
        return (world, res);
    }

    log::debug!("Beginning the \"parse\" phase");
    parse::phase().run(&mut world, &mut res);

    if hooks.after_parse(&mut c(&mut world, &mut res)) != Continuation::Continue
    {
        return (world, res);
    }

    log::debug!("Beginning the \"lowering\" phase");
    lowering::phase().run(&mut world, &mut res);

    if hooks.after_lowering(&mut c(&mut world, &mut res))
        != Continuation::Continue
    {
        return (world, res);
    }

    log::debug!("Beginning the \"type_check\" phase");
    type_check::phase().run(&mut world, &mut res);

    if hooks.after_type_checking(&mut c(&mut world, &mut res))
        != Continuation::Continue
    {
        return (world, res);
    }

    log::debug!("Beginning the \"codegen\" phase");
    codegen::phase().run(&mut world, &mut res);

    if hooks.after_codegen(&mut c(&mut world, &mut res))
        != Continuation::Continue
    {
        return (world, res);
    }

    compile::phase().run(&mut world, &mut res);

    if hooks.after_compile(&mut c(&mut world, &mut res))
        != Continuation::Continue
    {
        return (world, res);
    }

    (world, res)
}

/// A group of operations which make up a single "phase" in the build process.
pub struct Phase(legion::systems::Builder);

impl Phase {
    pub(crate) fn new() -> Self { Phase(legion::Schedule::builder()) }

    pub(crate) fn with_setup(
        mut setup: impl FnMut(&mut Resources) + 'static,
    ) -> Self {
        let mut phase = Phase::new();
        phase.0.add_thread_local_fn(move |_, res| setup(res));

        phase
    }

    pub(crate) fn and_then<F, R>(mut self, run_system: F) -> Self
    where
        R: legion::systems::ParallelRunnable + 'static,
        F: FnOnce() -> R,
    {
        self.0
            .add_system(TracingRunnable {
                runnable: run_system(),
                name: std::any::type_name::<F>(),
            })
            .flush();

        self
    }

    /// Execute the phase, updating the [`World`].
    pub fn run(&mut self, world: &mut World, resources: &mut Resources) {
        self.0.build().execute(world, resources);
    }
}

/// A wrapper around some [`Runnable`] which logs whenever it starts.
struct TracingRunnable<R> {
    runnable: R,
    name: &'static str,
}

impl<R: Runnable> Runnable for TracingRunnable<R> {
    fn name(&self) -> Option<&legion::systems::SystemId> {
        self.runnable.name()
    }

    fn reads(
        &self,
    ) -> (
        &[legion::systems::ResourceTypeId],
        &[legion::storage::ComponentTypeId],
    ) {
        self.runnable.reads()
    }

    fn writes(
        &self,
    ) -> (
        &[legion::systems::ResourceTypeId],
        &[legion::storage::ComponentTypeId],
    ) {
        self.runnable.writes()
    }

    fn prepare(&mut self, world: &World) { self.runnable.prepare(world); }

    fn accesses_archetypes(&self) -> &legion::world::ArchetypeAccess {
        self.runnable.accesses_archetypes()
    }

    unsafe fn run_unsafe(
        &mut self,
        world: &World,
        resources: &legion::systems::UnsafeResources,
    ) {
        let pretty_name = self
            .name
            .trim_start_matches(env!("CARGO_CRATE_NAME"))
            .trim_end_matches("_system")
            .trim_end_matches("::run")
            .trim_matches(':');
        log::debug!("Starting the \"{}\" pass", pretty_name);

        self.runnable.run_unsafe(world, resources);
    }

    fn command_buffer_mut(
        &mut self,
        world: legion::world::WorldId,
    ) -> Option<&mut legion::systems::CommandBuffer> {
        self.runnable.command_buffer_mut(world)
    }
}

fn c<'world, 'res>(
    world: &'world mut World,
    res: &'res mut Resources,
) -> Ctx<'world, 'res> {
    Ctx { world, res }
}

#[cfg(test)]
#[cfg(never)]
mod tests {
    use indexmap::IndexMap;
    use super::*;

    #[test]
    fn detect_pipeline_cycle() {
        let src = r#"
image: runicos/base
version: 1

pipeline:
  audio:
    proc-block: "hotg-ai/rune#proc_blocks/fft"
    inputs:
    - model
    outputs:
    - type: i16
      dimensions: [16000]

  fft:
    proc-block: "hotg-ai/rune#proc_blocks/fft"
    inputs:
    - audio
    outputs:
    - type: i8
      dimensions: [1960]

  model:
    model: "./model.tflite"
    inputs:
    - fft
    outputs:
    - type: i8
      dimensions: [6]
            "#;
        let doc = Document::parse(src).unwrap();
        let mut diags = Diagnostics::new();

        let _ = crate::analyse(doc, &mut diags);

        assert!(diags.has_errors());
        let errors: Vec<_> = diags
            .iter_severity(codespan_reporting::diagnostic::Severity::Error)
            .collect();
        assert_eq!(errors.len(), 1);
        let diag = errors[0];
        assert_eq!(diag.message, "Cycle detected when checking \"audio\"");
        assert!(diag.notes[0].contains("model"));
        assert!(diag.notes[1].contains("fft"));
        assert_eq!(
            diag.notes[2],
            "... which receives input from \"audio\", completing the cycle."
        );
    }
}
