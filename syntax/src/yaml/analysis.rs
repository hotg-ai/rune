use std::{collections::HashMap};
use codespan_reporting::{
    diagnostic::Diagnostic,
};
use petgraph::graph::NodeIndex;
use crate::{
    Diagnostics,
    hir::{self, HirId, Rune, Edge, Primitive},
    yaml::{
        types::*,
        utils::{Builtins, HirIds},
    },
};

pub fn analyse(doc: &Document) -> (Rune, Diagnostics) {
    let mut ctx = Context::default();

    ctx.register_names(&doc.pipeline);
    ctx.register_stages(&doc.pipeline);
    ctx.construct_pipeline(&doc.pipeline);

    let Context { rune, diags, .. } = ctx;

    (rune, diags)
}

#[derive(Debug)]
struct Context {
    diags: Diagnostics,
    rune: Rune,
    ids: HirIds,
    builtins: Builtins,
    stages: HashMap<HirId, NodeIndex>,
    input_types: HashMap<NodeIndex, HirId>,
    output_types: HashMap<NodeIndex, HirId>,
}

impl Context {
    fn register_names(&mut self, pipeline: &HashMap<String, Stage>) {
        for (name, _step) in pipeline {
            let id = self.ids.next();
            self.rune.names.register(name, id);
        }
    }

    fn register_stages(&mut self, pipeline: &HashMap<String, Stage>) {
        for (name, stage) in pipeline {
            let id = self.rune.names[name.as_str()];

            let node_index = self.rune.graph.add_node(hir::Stage::from(stage));
            self.rune.add_hir_id_and_node_index(id, node_index);
        }
    }

    fn construct_pipeline(&mut self, pipeline: &HashMap<String, Stage>) {
        for (name, stage) in pipeline {
            let node_index = self.node_index_by_name(name).unwrap();

            for previous in stage.inputs() {
                let stage = match pipeline.get(previous) {
                    Some(s) => s,
                    None => {
                        let msg = format!("The \"{}\" stage declares \"{}\" as an input, but no such stage exists", name, previous);
                        let diag = Diagnostic::error().with_message(msg);
                        self.diags.push(diag);

                        continue;
                    },
                };
                let previous_node_index = match self
                    .node_index_by_name(previous)
                {
                    Some(ix) => ix,
                    None => {
                        let msg = format!("The \"{}\" stage declares \"{}\" as an input, but no such stage was added to the pipeline graph", name, previous);
                        let diag = Diagnostic::error().with_message(msg);
                        self.diags.push(diag);

                        continue;
                    },
                };

                let output_type = match stage.output_type() {
                    Some(t) => t,
                    None => {
                        let msg = format!("\"{}\" is used as an input to \"{}\", but it doesn't declare any outputs", previous, name);
                        let diag = Diagnostic::error().with_message(msg);
                        self.diags.push(diag);

                        continue;
                    },
                };

                let edge = Edge {
                    type_id: self.intern_type(output_type),
                };
                self.rune
                    .graph
                    .add_edge(previous_node_index, node_index, edge);
            }
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

    fn node_index_by_name(&self, name: &str) -> Option<NodeIndex> {
        let id = self.rune.names.get_id(name)?;
        self.rune.hir_id_to_node_index.get(&id).copied()
    }
}

impl Default for Context {
    fn default() -> Context {
        let mut rune = Rune::default();
        let mut ids = HirIds::new();
        let builtins = Builtins::new(&mut ids);
        builtins.copy_into(&mut rune);

        Context {
            ids,
            builtins,
            rune,
            diags: Diagnostics::default(),
            stages: HashMap::default(),
            input_types: HashMap::default(),
            output_types: HashMap::default(),
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
                    args: HashMap::new(),
                    inputs: vec![String::from("label")],
                },
                label: Stage::ProcBlock {
                    proc_block: "hotg-ai/rune#proc_blocks/ohv_label".parse().unwrap(),
                    inputs: vec![String::from("model")],
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
                    inputs: vec![String::from("audio")],
                    outputs: vec![ty!(i8[1960])],
                    args: HashMap::new(),
                },
                model: Stage::Model {
                    model: String::from("./model.tflite"),
                    inputs: vec![String::from("fft")],
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
                    inputs: vec![String::from("audio")],
                    outputs: vec![
                        ty!(i8[1960]),
                    ],
                    args: HashMap::new(),
                },
                model: Stage::Model {
                    model: String::from("./model.tflite"),
                    inputs: vec![String::from("fft")],
                    outputs: vec![
                        ty!(i8[6]),
                    ],
                },
                label: Stage::ProcBlock {
                    proc_block: "hotg-ai/rune#proc_blocks/ohv_label".parse().unwrap(),
                    inputs: vec![String::from("model")],
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
                    inputs: vec![String::from("label")],
                    args: HashMap::default(),
                }
            },
        }
    }

    #[test]
    fn register_all_stage_names() {
        let doc = dummy_document();
        let mut ctx = Context::default();

        ctx.register_names(&doc.pipeline);

        let expected = vec!["audio", "fft", "model", "label", "output"];
        let got = &ctx.rune.names;

        for name in expected {
            assert!(got.get_id(name).is_some(), "{}", name);
        }
    }

    #[test]
    fn register_all_stages() {
        let doc = dummy_document();
        let mut ctx = Context::default();
        let stages = vec!["audio", "fft", "model", "label", "output"];
        ctx.register_names(&doc.pipeline);

        ctx.register_stages(&doc.pipeline);

        for ty in stages {
            let node_index = ctx.node_index_by_name(ty).unwrap();
            assert!(ctx.rune.graph.node_weight(node_index).is_some());
        }
    }

    #[test]
    fn construct_the_pipeline() {
        let doc = dummy_document();
        let mut ctx = Context::default();
        ctx.register_names(&doc.pipeline);
        ctx.register_stages(&doc.pipeline);
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
            let from_ix = ctx.node_index_by_name(from).unwrap();
            let to_ix = ctx.node_index_by_name(to).unwrap();

            let _edge = ctx.rune.graph.find_edge(from_ix, to_ix).unwrap();
        }
    }
}
