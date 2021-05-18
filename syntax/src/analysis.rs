use codespan::Span;
use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
};
use indexmap::IndexMap;
use crate::{
    Diagnostics,
    hir::{self, HirId, Node, Primitive, Rune, Slot},
    yaml::*,
    utils::{Builtins, HirIds, range_span},
};

pub fn analyse(doc: &Document, diags: &mut Diagnostics) -> Rune {
    let mut ctx = Context::new(diags);

    let image = &doc.image;
    ctx.rune.base_image = Some(image.into());

    ctx.register_stages(&doc.pipeline);
    ctx.register_output_slots(&doc.pipeline);
    ctx.construct_pipeline(&doc.pipeline);

    ctx.rune
}

#[derive(Debug)]
struct Context<'diag> {
    diags: &'diag mut Diagnostics,
    rune: Rune,
    ids: HirIds,
    builtins: Builtins,
}

impl<'diag> Context<'diag> {
    fn new(diags: &'diag mut Diagnostics) -> Self {
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

    fn register_name(&mut self, name: &str, id: HirId, definition: Span) {
        if let Err(original_definition_id) = self.rune.names.register(name, id)
        {
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

    fn register_stages(&mut self, pipeline: &IndexMap<String, Stage>) {
        for (name, stage) in pipeline {
            let id = self.ids.next();
            self.rune.stages.insert(
                id,
                Node {
                    stage: hir::Stage::from(stage),
                    input_slots: Vec::new(),
                    output_slots: Vec::new(),
                },
            );
            self.register_name(name, id, stage.span());
        }
    }

    fn register_output_slots(&mut self, pipeline: &IndexMap<String, Stage>) {
        for (name, stage) in pipeline {
            let node_id = self.rune.names.get_id(name).unwrap();

            let mut output_slots = Vec::new();

            for ty in stage.output_types() {
                let element_type = self.intern_type(ty);
                let id = self.ids.next();
                self.rune.slots.insert(id, Slot { element_type });
                output_slots.push(id);
            }

            let node = self.rune.stages.get_mut(&node_id).unwrap();
            node.output_slots = output_slots;
        }
    }

    fn construct_pipeline(&mut self, pipeline: &IndexMap<String, Stage>) {
        for (name, stage) in pipeline {
            let node_id = self.rune.names.get_id(name).unwrap();

            let mut input_slots = Vec::new();

            for input in stage.inputs() {
                let incoming_node_id = match self.rune.names.get_id(&input.name)
                {
                    Some(id) => id,
                    None => {
                        let diag = Diagnostic::error().with_message(format!(
                            "No node associated with \"{}\"",
                            input
                        ));
                        self.diags.push(diag);
                        input_slots.push(HirId::ERROR);
                        continue;
                    },
                };

                let incoming_node = &self.rune.stages[&incoming_node_id];

                if incoming_node.output_slots.is_empty() {
                    let diag = Diagnostic::error().with_message(format!(
                            "The \"{}\" stage tried to connect to \"{}\" but that stage doesn't have any outputs",
                            name,
                            input
                        ));
                    self.diags.push(diag);
                    input_slots.push(HirId::ERROR);
                    continue;
                }

                let input_index = input.index.unwrap_or(0);
                match incoming_node.output_slots.get(input_index) {
                    Some(slot_id) => input_slots.push(*slot_id),
                    None => {
                        let diag = Diagnostic::error().with_message(format!(
                            "The \"{}\" stage tried to connect to \"{}\" but that stage only has {} outputs",
                            name,
                            input,
                            incoming_node.output_slots.len(),
                        ));
                        self.diags.push(diag);
                        input_slots.push(HirId::ERROR);
                        continue;
                    },
                }
            }

            let node = self.rune.stages.get_mut(&node_id).unwrap();
            node.input_slots = input_slots;
        }
    }

    fn intern_type(&mut self, ty: &Type) -> HirId {
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
    use super::*;

    macro_rules! map {
        // map-like
        ($($k:ident : $v:expr),* $(,)?) => {
            std::iter::Iterator::collect(std::array::IntoIter::new([
                $(
                    (String::from(stringify!($k)), $v)
                ),*
            ]))
        };
        // set-like
        ($($v:expr),* $(,)?) => {
            std::iter::Iterator::collect(std::array::IntoIter::new([$($v,)*]))
        };
    }

    macro_rules! ty {
        ($type:ident [$($dim:expr),*]) => {
            Type {
                name: String::from(stringify!($type)),
                dimensions: vec![ $($dim),*],
            }
        };
        ($type:ident) => {
            Type {
                name: String::from(stringify!($type)),
                dimensions: vec![],
            }
        }
    }

    #[test]
    fn parse_yaml_pipeline() {
        let src = r#"
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
        let should_be = Document {
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
                    model: String::from("./model.tflite"),
                    inputs: vec!["fft".parse().unwrap()],
                    outputs: vec![ty!(i8[6])],
                },
            },
        };

        let got: Document = serde_yaml::from_str(src).unwrap();

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
            ("\"42\"", Value::String(String::from("42"))),
            (
                "[1, 2.0, \"asdf\"]",
                Value::List(vec![
                    Value::Int(1),
                    Value::Float(2.0),
                    Value::String(String::from("asdf")),
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

    fn dummy_document() -> Document {
        Document {
            image: Path::new("runicos/base".to_string(), None, None),
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
                    inputs: vec!["audio".parse().unwrap()],
                    outputs: vec![
                        ty!(i8[1960]),
                    ],
                    args: IndexMap::new(),
                },
                model: Stage::Model {
                    model: String::from("./model.tflite"),
                    inputs: vec!["fft".parse().unwrap()],
                    outputs: vec![
                        ty!(i8[6]),
                    ],
                },
                label: Stage::ProcBlock {
                    proc_block: "hotg-ai/rune#proc_blocks/ohv_label".parse().unwrap(),
                    inputs: vec!["model".parse().unwrap()],
                    outputs: vec![
                        ty!(utf8),
                    ],
                    args: map! {
                        labels: Value::List(vec![
                            Value::from("silence"),
                            Value::from("unknown"),
                            Value::from("up"),
                        ]),
                    },
                },
                output: Stage::Out {
                    out: String::from("SERIAL"),
                    inputs: vec!["label".parse().unwrap()],
                    args: IndexMap::default(),
                }
            },
        }
    }

    #[test]
    fn register_all_stages() {
        let doc = dummy_document();
        let mut diags = Diagnostics::new();
        let mut ctx = Context::new(&mut diags);
        let stages = vec!["audio", "fft", "model", "label", "output"];

        ctx.register_stages(&doc.pipeline);

        for stage_name in stages {
            let id = ctx.rune.names.get_id(stage_name).unwrap();
            assert!(ctx.rune.stages.contains_key(&id));
        }

        assert!(diags.is_empty());
    }

    #[test]
    fn construct_the_pipeline() {
        let doc = dummy_document();
        let mut diags = Diagnostics::new();
        let mut ctx = Context::new(&mut diags);
        ctx.register_stages(&doc.pipeline);
        ctx.register_output_slots(&doc.pipeline);
        let edges = vec![
            ("audio", "fft"),
            ("fft", "model"),
            ("model", "label"),
            ("label", "output"),
        ];

        ctx.construct_pipeline(&doc.pipeline);

        assert!(ctx.diags.is_empty(), "{:?}", ctx.diags);
        for (from, to) in edges {
            println!("{:?} => {:?}", from, to);
            let from_id = ctx.rune.names.get_id(from).unwrap();
            let to_id = ctx.rune.names.get_id(to).unwrap();

            assert!(ctx.rune.has_connection(from_id, to_id));
        }
    }

    #[test]
    fn construct_pipeline_graph_with_multiple_inputs_and_outputs() {
        let doc = Document {
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
        };
        let mut diags = Diagnostics::new();

        let rune = analyse(&doc, &mut diags);

        assert!(!diags.has_errors() && !diags.has_warnings(), "{:#?}", diags);

        let audio_id = rune.names["audio"];
        let audio_node = &rune.stages[&audio_id];
        assert!(audio_node.input_slots.is_empty());
        assert_eq!(audio_node.output_slots.len(), 1);
        let audio_output = audio_node.output_slots[0];

        let fft_id = rune.names["fft"];
        let fft_node = &rune.stages[&fft_id];
        assert_eq!(
            fft_node.input_slots,
            &[audio_output, audio_output, audio_output]
        );

        let output_id = rune.names["serial"];
        let output_node = &rune.stages[&output_id];
        assert_eq!(fft_node.output_slots, output_node.input_slots);
    }
}
