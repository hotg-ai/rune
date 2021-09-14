mod load_resource_data;
mod parse;
mod register_names;
mod register_resources;
mod register_stages;
mod register_tensors;
mod update_nametable;

use legion::{Resources, World};
use crate::{
    BuildContext, Diagnostics,
    hooks::{Continuation, Ctx, Hooks},
    hir::NameTable,
};

/// Execute the `rune build` process.
pub fn build(ctx: BuildContext, hooks: &mut dyn Hooks) -> (World, Resources) {
    let mut world = World::default();
    let mut res = initialize_resources(ctx);

    if hooks.before_parse(&mut c(&mut world, &mut res))
        != Continuation::Continue
    {
        return (world, res);
    }

    Schedule::new()
        .and_then(parse::run_system())
        .run(&mut world, &mut res);

    if hooks.after_parse(&mut c(&mut world, &mut res)) != Continuation::Continue
    {
        return (world, res);
    }

    Schedule::new()
        .and_then(register_names::run_system())
        .and_then(update_nametable::run_system())
        .and_then(register_resources::run_system())
        .and_then(register_stages::run_system())
        .and_then(register_tensors::run_system())
        .run(&mut world, &mut res);

    if hooks.after_lowering(&mut c(&mut world, &mut res))
        != Continuation::Continue
    {
        return (world, res);
    }

    Schedule::new()
        .and_then(load_resource_data::run_system())
        .run(&mut world, &mut res);

    if hooks.after_type_checking(&mut c(&mut world, &mut res))
        != Continuation::Continue
    {
        return (world, res);
    }

    (world, res)
}

pub(crate) fn initialize_resources(ctx: BuildContext) -> Resources {
    let mut resources = Resources::default();

    resources.insert(ctx);
    resources.insert(Diagnostics::new());
    resources.insert(NameTable::default());

    resources
}

fn c<'world, 'res>(
    world: &'world mut World,
    res: &'res mut Resources,
) -> Ctx<'world, 'res> {
    Ctx { world, res }
}

/// A helper type for constructing a [`legion::Schedule`] which automatically
/// flushes the [`legion::systems::CommandBuffer`] after each step.
pub(crate) struct Schedule(legion::systems::Builder);

impl Schedule {
    fn new() -> Self { Schedule(legion::Schedule::builder()) }

    fn and_then(
        &mut self,
        runnable: impl legion::systems::ParallelRunnable + 'static,
    ) -> &mut Self {
        self.0.add_system(runnable).flush();
        self
    }

    pub fn run(&mut self, world: &mut World, resources: &mut Resources) {
        self.0.build().execute(world, resources);
    }
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
