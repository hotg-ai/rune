// mod check_for_loops;
mod register_names;
mod register_output_slots;
mod register_resources;
mod register_stages;
mod register_tensors;
mod update_nametable;

use codespan_reporting::diagnostic::{Diagnostic, Label};
use legion::{Resources, World};

use crate::{
    Diagnostics,
    hooks::{Continuation, Ctx, Hooks},
    passes::update_nametable::NameTable,
    yaml::*,
};

pub fn build(src: &str, hooks: &mut dyn Hooks) -> (World, Resources) {
    let mut world = World::default();
    let mut res = initialize_resources();

    if hooks.before_parse(&mut c(&mut world, &mut res))
        != Continuation::Continue
    {
        return (world, res);
    }

    let doc = match Document::parse(src) {
        Ok(d) => d,
        Err(e) => {
            let mut diag = Diagnostic::error().with_message(e.to_string());
            if let Some(location) = e.location() {
                let ix = location.index();
                diag = diag.with_labels(vec![Label::primary((), ix..ix)]);
            }
            res.get_mut::<Diagnostics>().unwrap().push(diag);
            hooks.after_parse(&mut c(&mut world, &mut res));
            return (world, res);
        },
    };

    // TODO: move document parsing here
    res.insert(doc.to_v1());

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

    (world, res)
}

pub(crate) fn initialize_resources() -> Resources {
    let mut resources = Resources::default();

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
/// flushes the [`legion::CommandBuffer`] after each step.
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
    fn construct_pipeline_graph_with_multiple_inputs_and_outputs() {
        let doc = Document::V1(DocumentV1 {
            image: "runicos/base@latest".parse().unwrap(),
            pipeline: map! {
                audio: Stage::Capability {
                    capability: String::from("SOUND"),
                    outputs: vec![
                        ty!(i16[16000]),
                    ],
                    args: map! {
                        hz: Value::from(16000),
                    },
                },
                fft: Stage::ProcBlock {
                    proc_block: "hotg-ai/rune#proc_blocks/fft".parse().unwrap(),
                    inputs: vec![
                        "audio".parse().unwrap(),
                        "audio".parse().unwrap(),
                        "audio".parse().unwrap(),
                        ],
                    outputs: vec![
                        ty!(i8[1960]),
                        ty!(i8[1960]),
                        ty!(i8[1960]),
                    ],
                    args: IndexMap::new(),
                },
                serial: Stage::Out {
                    out: String::from("SERIAL"),
                    inputs: vec![
                        "fft.0".parse().unwrap(),
                        "fft.1".parse().unwrap(),
                        "fft.2".parse().unwrap(),
                    ],
                    args: IndexMap::new(),
                },
            },
            resources: map![],
        });
        let mut diags = Diagnostics::new();

        let rune = crate::analyse(doc, &mut diags);

        assert!(!diags.has_errors() && !diags.has_warnings(), "{:#?}", diags);

        let audio_id = rune.get_id_by_name("audio").unwrap();
        let audio_node = &rune.get_stage(&audio_id).unwrap();
        assert!(audio_node.input_slots.is_empty());
        assert_eq!(audio_node.output_slots.len(), 1);
        let audio_output = audio_node.output_slots[0];

        let fft_id = rune.get_id_by_name("fft").unwrap();
        let fft_node = &rune.get_stage(&fft_id).unwrap();
        assert_eq!(
            fft_node.input_slots,
            &[audio_output, audio_output, audio_output]
        );

        let output_id = rune.get_id_by_name("serial").unwrap();
        let output_node = &rune.get_stage(&output_id).unwrap();
        assert_eq!(fft_node.output_slots, output_node.input_slots);
    }

    #[test]
    fn topological_sorting() {
        let doc = crate::utils::dummy_document();
        let mut diags = Diagnostics::new();
        let rune = crate::analyse(doc, &mut diags);
        let should_be = ["audio", "fft", "model", "label", "output"];

        let got: Vec<_> = rune.sorted_pipeline().collect();

        let should_be: Vec<_> = should_be
            .iter()
            .copied()
            .map(|name| rune.get_id_by_name(name).unwrap())
            .map(|id| (id, rune.get_stage(&id).unwrap()))
            .collect();
        assert_eq!(got, should_be);
    }

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
