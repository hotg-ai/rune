use codespan::Span;
use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
};
use crate::{
    Diagnostics,
    hir::{self, HirId, Primitive, Rune},
    utils::{Builtins, HirIds, range_span},
    yaml::*,
};

#[derive(Debug)]
pub(crate) struct Context<'diag> {
    pub(crate) diags: &'diag mut Diagnostics,
    pub(crate) rune: Rune,
    pub(crate) ids: HirIds,
    pub(crate) builtins: Builtins,
}

impl<'diag> Context<'diag> {
    pub(crate) fn new(diags: &'diag mut Diagnostics) -> Self {
        let mut rune = Rune::default();
        let mut ids = HirIds::new();
        let builtins = Builtins::new(&mut ids);
        builtins.copy_into(&mut rune);

        Context {
            ids,
            builtins,
            rune,
            diags,
        }
    }

    pub(crate) fn register_name(
        &mut self,
        name: &str,
        id: HirId,
        definition: Span,
    ) {
        if let Err(original_definition_id) = self.rune.register_name(name, id) {
            let duplicate = Label::primary((), range_span(definition))
                .with_message("Original definition here");
            let mut labels = vec![duplicate];

            if let Some(original_definition) =
                self.rune.spans.get(&original_definition_id)
            {
                let original =
                    Label::secondary((), range_span(*original_definition))
                        .with_message("Original definition here");
                labels.push(original);
            }

            let diag = Diagnostic::error()
                .with_message(format!("\"{}\" is already defined", name))
                .with_labels(labels);
            self.diags.push(diag);
        }
    }

    pub(crate) fn intern_type(&mut self, ty: &Type) -> HirId {
        let underlying_type = match self.primitive_type(&ty.name) {
            Some(p) => p,
            None => {
                let msg = format!("Unknown type: {}", ty.name);
                let diag = Diagnostic::warning().with_message(msg);
                self.diags.push(diag);
                return self.builtins.unknown_type;
            },
        };

        let ty = if ty.dimensions.is_empty() {
            hir::Type::Primitive(underlying_type)
        } else {
            hir::Type::Buffer {
                underlying_type: self.builtins.get_id(underlying_type),
                dimensions: ty.dimensions.clone(),
            }
        };

        match self.rune.types.iter().find(|(_, t)| **t == ty) {
            Some((id, _)) => *id,
            None => {
                // new buffer type
                let id = self.ids.next();
                self.rune.types.insert(id, ty);
                id
            },
        }
    }

    fn primitive_type(&mut self, name: &str) -> Option<Primitive> {
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
    fn parse_yaml_pipeline() {
        let src = r#"
version: 1
image: "runicos/base"

pipeline:
  audio:
    capability: SOUND
    outputs:
    - type: i16
      dimensions: [16000]
    args:
      hz: 16000

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

  label:
    proc-block: "hotg-ai/rune#proc_blocks/ohv_label"
    inputs:
    - model
    outputs:
    - type: utf8
    args:
      labels: ["silence", "unknown", "up", "down", "left", "right"]

  output:
    out: SERIAL
    inputs:
    - label
        "#;
        let should_be = Document::V1 {
            image: Path::new("runicos/base", None, None),
            pipeline: map! {
                audio: Stage::Capability {
                    capability: String::from("SOUND"),
                    outputs: vec![ty!(i16[16000])],
                    args: map! { hz: Value::Int(16000) },
                },
                output: Stage::Out {
                    out: String::from("SERIAL"),
                    args: IndexMap::new(),
                    inputs: vec!["label".parse().unwrap()],
                },
                label: Stage::ProcBlock {
                    proc_block: "hotg-ai/rune#proc_blocks/ohv_label".parse().unwrap(),
                    inputs: vec!["model".parse().unwrap()],
                    outputs: vec![Type { name: String::from("utf8"), dimensions: Vec::new() }],
                    args: map! {
                        labels: Value::from(vec![
                            Value::from("silence"),
                            Value::from("unknown"),
                            Value::from("up"),
                            Value::from("down"),
                            Value::from("left"),
                            Value::from("right"),
                        ]),
                    },
                },
                fft: Stage::ProcBlock {
                    proc_block: "hotg-ai/rune#proc_blocks/fft".parse().unwrap(),
                    inputs: vec!["audio".parse().unwrap()],
                    outputs: vec![ty!(i8[1960])],
                    args: IndexMap::new(),
                },
                model: Stage::Model {
                    model: "./model.tflite".into(),
                    inputs: vec!["fft".parse().unwrap()],
                    outputs: vec![ty!(i8[6])],
                },
            },
            resources: map![],
        };

        let got = Document::parse(src).unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_audio_block() {
        let src = r#"
              capability: SOUND
              outputs:
              - type: i16
                dimensions: [16000]
              args:
                hz: 16000
        "#;
        let should_be = Stage::Capability {
            capability: String::from("SOUND"),
            outputs: vec![Type {
                name: String::from("i16"),
                dimensions: vec![16000],
            }],
            args: map! { hz: Value::Int(16000) },
        };

        let got: Stage = serde_yaml::from_str(src).unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_values() {
        let inputs = vec![
            ("42", Value::Int(42)),
            ("3.14", Value::Float(3.14)),
            ("\"42\"", Value::String("42".into())),
            (
                "[1, 2.0, \"asdf\"]",
                Value::List(vec![
                    Value::Int(1),
                    Value::Float(2.0),
                    Value::String("asdf".into()),
                ]),
            ),
        ];

        for (src, should_be) in inputs {
            let got: Value = serde_yaml::from_str(src).unwrap();
            assert_eq!(got, should_be);
        }
    }

    #[test]
    fn parse_paths() {
        let inputs = vec![
            ("asdf", Path::new("asdf", None, None)),
            ("runicos/base", Path::new("runicos/base", None, None)),
            (
                "runicos/base@0.1.2",
                Path::new("runicos/base", None, "0.1.2".to_string()),
            ),
            (
                "runicos/base@latest",
                Path::new("runicos/base", None, "latest".to_string()),
            ),
            (
                "https://github.com/hotg-ai/rune",
                Path::new("https://github.com/hotg-ai/rune", None, None),
            ),
            (
                "https://github.com/hotg-ai/rune@2",
                Path::new(
                    "https://github.com/hotg-ai/rune",
                    None,
                    "2".to_string(),
                ),
            ),
            (
                "hotg-ai/rune@v1.2#proc_blocks/normalize",
                Path::new(
                    "hotg-ai/rune",
                    "proc_blocks/normalize".to_string(),
                    "v1.2".to_string(),
                ),
            ),
        ];

        for (src, should_be) in inputs {
            let got: Path = src.parse().unwrap();
            assert_eq!(got, should_be);
        }
    }

    #[test]
    fn construct_pipeline_graph_with_multiple_inputs_and_outputs() {
        let doc = Document::V1 {
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
        };
        let mut diags = Diagnostics::new();

        let rune = crate::analyse(&doc, &mut diags);

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
        let rune = crate::analyse(&doc, &mut diags);
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

        let _ = crate::analyse(&doc, &mut diags);

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
