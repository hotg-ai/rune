mod check_for_loops;
mod construct_pipeline;
mod register_output_slots;
mod register_resources;
mod register_stages;

use crate::{
    Diagnostics,
    hir::{Image, Rune},
    utils::{Builtins, HirIds},
    yaml::*,
};

pub fn analyse(doc: Document, diags: &mut Diagnostics) -> Rune {
    let mut rune = Rune::default();
    let mut ids = HirIds::new();
    let builtins = Builtins::new(&mut ids);
    builtins.copy_into(&mut rune);

    let DocumentV1 {
        image,
        pipeline,
        resources,
    } = doc.to_v1();

    rune.base_image = Some(Image(image.clone().into()));

    register_resources::run(
        diags,
        &mut ids,
        &resources,
        &mut rune.resources,
        &rune.spans,
        &mut rune.names,
    );

    register_stages::run(
        &mut ids,
        &pipeline,
        &rune.spans,
        &mut rune.stages,
        &rune.resources,
        &mut rune.names,
        diags,
    );

    register_output_slots::run(
        &mut ids,
        &pipeline,
        &mut rune.types,
        &builtins,
        &rune.names,
        &mut rune.slots,
        &mut rune.stages,
        diags,
    );

    construct_pipeline::run(
        &pipeline,
        &rune.names,
        &mut rune.stages,
        &mut rune.slots,
        diags,
    );
    check_for_loops::run(
        &rune.stages,
        &rune.slots,
        &rune.names,
        &rune.spans,
        diags,
    );

    rune
}

mod helpers {
    use codespan::Span;
    use codespan_reporting::diagnostic::{Diagnostic, Label};
    use indexmap::IndexMap;
    use crate::{
        hir::{self, HirId, NameTable, Primitive},
        utils::{Builtins, HirIds, range_span},
    };
    use super::*;

    pub(crate) fn register_name(
        name: &str,
        id: HirId,
        definition_span: Span,
        spans: &IndexMap<HirId, Span>,
        names: &mut NameTable,
        diags: &mut Diagnostics,
    ) {
        if let Err(original_definition_id) = names.register(name, id) {
            let duplicate = Label::primary((), range_span(definition_span))
                .with_message("Original definition here");
            let mut labels = vec![duplicate];

            if let Some(original_definition) =
                spans.get(&original_definition_id)
            {
                let original =
                    Label::secondary((), range_span(*original_definition))
                        .with_message("Original definition here");
                labels.push(original);
            }

            let diag = Diagnostic::error()
                .with_message(format!("\"{}\" is already defined", name))
                .with_labels(labels);
            diags.push(diag);
        }
    }

    pub(crate) fn intern_type(
        ids: &mut HirIds,
        ty: &Type,
        types: &mut IndexMap<HirId, hir::Type>,
        builtins: &Builtins,
        diags: &mut Diagnostics,
    ) -> HirId {
        let underlying_type = match primitive_type(&ty.name) {
            Some(p) => p,
            None => {
                let msg = format!("Unknown type: {}", ty.name);
                let diag = Diagnostic::warning().with_message(msg);
                diags.push(diag);
                return builtins.unknown_type;
            },
        };

        let ty = if ty.dimensions.is_empty() {
            hir::Type::Primitive(underlying_type)
        } else {
            hir::Type::Buffer {
                underlying_type: builtins.get_id(underlying_type),
                dimensions: ty.dimensions.clone(),
            }
        };

        match types.iter().find(|(_, t)| **t == ty) {
            Some((id, _)) => *id,
            None => {
                // new buffer type
                let id = ids.next();
                types.insert(id, ty);
                id
            },
        }
    }

    pub(crate) fn primitive_type(name: &str) -> Option<Primitive> {
        match name {
            "u8" | "U8" => Some(Primitive::U8),
            "i8" | "I8" => Some(Primitive::I8),
            "u16" | "U16" => Some(Primitive::U16),
            "i16" | "I16" => Some(Primitive::I16),
            "u32" | "U32" => Some(Primitive::U32),
            "i32" | "I32" => Some(Primitive::I32),
            "u64" | "U64" => Some(Primitive::U64),
            "i64" | "I64" => Some(Primitive::I64),
            "f32" | "F32" => Some(Primitive::F32),
            "f64" | "F64" => Some(Primitive::F64),
            "utf8" | "UTF8" => Some(Primitive::String),
            _ => None,
        }
    }
}

#[cfg(test)]
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
