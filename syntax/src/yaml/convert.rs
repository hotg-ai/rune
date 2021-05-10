use crate::{
    ast::{
        self, Runefile, Instruction, CapabilityInstruction, Argument,
        ArgumentValue, Literal, LiteralKind, ProcBlockInstruction,
        ModelInstruction, OutInstruction,
    },
    yaml::{Path, Document, Stage, Value, Type},
};
use indexmap::IndexMap;

pub fn document_from_runefile(runefile: Runefile) -> Document {
    let mut image = Path::new("runicos/base", None, None);
    let mut pipeline = IndexMap::new();

    for instruction in runefile.instructions {
        match instruction {
            Instruction::From(from) => {
                image = yaml_path(from.image);
            },
            Instruction::Model(m) => {
                let ModelInstruction {
                    name,
                    file,
                    output_type,
                    ..
                } = m;
                let stage = Stage::Model {
                    model: file,
                    inputs: Vec::new(),
                    outputs: vec![convert_type(output_type)],
                };
                pipeline.insert(name.value, stage);
            },
            Instruction::Capability(cap) => {
                let CapabilityInstruction {
                    kind,
                    name,
                    output_type,
                    parameters,
                    ..
                } = cap;

                let stage = Stage::Capability {
                    capability: kind.value,
                    outputs: vec![convert_type(output_type)],
                    args: convert_args(parameters),
                };
                pipeline.insert(name.value, stage);
            },
            Instruction::Out(out) => {
                let OutInstruction { out_type, .. } = out;
                let name = out_type.value.to_lowercase();
                let stage = Stage::Out {
                    out: out_type.value.clone(),
                    inputs: Vec::new(),
                    args: IndexMap::default(),
                };
                pipeline.insert(name, stage);
            },
            Instruction::ProcBlock(pb) => {
                let ProcBlockInstruction {
                    path,
                    output_type,
                    name,
                    params,
                    ..
                } = pb;

                let stage = Stage::ProcBlock {
                    proc_block: yaml_path(path),
                    inputs: Vec::new(),
                    outputs: vec![convert_type(output_type)],
                    args: convert_args(params),
                };
                pipeline.insert(name.value, stage);
            },
            Instruction::Run(r) => {
                for window in r.steps.windows(2) {
                    let prev = &window[0].value;
                    let next = &window[1].value;

                    if let Some(stage) = pipeline.get_mut(next) {
                        if let Some(inputs) = stage.inputs_mut() {
                            inputs.push(prev.to_string());
                        }
                    }
                }
            },
        }
    }

    Document { image, pipeline }
}

fn convert_type(t: ast::Type) -> Type {
    match t.kind {
        ast::TypeKind::Buffer {
            type_name,
            dimensions,
        } => Type {
            name: type_name.value,
            dimensions,
        },
        ast::TypeKind::Named(name) => Type {
            name: name.value,
            dimensions: Vec::new(),
        },
        ast::TypeKind::Inferred => todo!(),
    }
}

fn convert_args(arguments: Vec<Argument>) -> IndexMap<String, Value> {
    let mut args = IndexMap::new();

    for arg in arguments {
        args.insert(arg.name.value, convert_value(arg.value));
    }

    args
}

fn convert_value(value: ArgumentValue) -> Value {
    match value {
        ArgumentValue::Literal(Literal {
            kind: LiteralKind::Integer(i),
            ..
        }) => i.into(),
        ArgumentValue::Literal(Literal {
            kind: LiteralKind::Float(f),
            ..
        }) => f.into(),
        ArgumentValue::Literal(Literal {
            kind: LiteralKind::String(s),
            ..
        }) => s.into(),
        ArgumentValue::List(items) => {
            Value::List(items.into_iter().map(Value::from).collect())
        },
    }
}

fn yaml_path(p: ast::Path) -> Path {
    let ast::Path {
        base,
        sub_path,
        version,
        ..
    } = p;

    Path::new(base, sub_path, version)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::yaml::{Stage, Type, Value};

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
    fn convert_existing_runefile() {
        let runefile = r#"
            FROM runicos/base

            CAPABILITY<I16[16000]> audio SOUND --hz 16000 --sample_duration_ms 1000
            PROC_BLOCK<I16[16000],I8[1960]> fft hotg-ai/rune#proc_blocks/fft
            MODEL<I8[1960],I8[4]> model ./model.tflite
            PROC_BLOCK<I8[4], UTF8> label hotg-ai/rune#proc_blocks/ohv_label --labels=silence,unknown,yes,no
            OUT serial

            RUN audio fft model label serial
        "#;
        let runefile = crate::parse(runefile).unwrap();
        let should_be = Document {
            image: "runicos/base".parse().unwrap(),
            pipeline: map! {
                audio: Stage::Capability {
                    capability: String::from("SOUND"),
                    outputs: vec![ty!(I16[16000])],
                    args: map! {
                        sample_duration_ms: Value::Int(1000),
                        hz: Value::Int(16000),
                     },
                },
                label: Stage::ProcBlock {
                    proc_block: "hotg-ai/rune#proc_blocks/ohv_label".parse().unwrap(),
                    inputs: vec![String::from("model")],
                    outputs: vec![ty!(UTF8)],
                    args: map! {
                        labels: Value::from(vec![
                            Value::from("silence"),
                            Value::from("unknown"),
                            Value::from("yes"),
                            Value::from("no"),
                        ]),
                    },
                },
                fft: Stage::ProcBlock {
                    proc_block: "hotg-ai/rune#proc_blocks/fft".parse().unwrap(),
                    inputs: vec![String::from("audio")],
                    outputs: vec![ty!(I8[1960])],
                    args: IndexMap::new(),
                },
                model: Stage::Model {
                    model: String::from("./model.tflite"),
                    inputs: vec![String::from("fft")],
                    outputs: vec![ty!(I8[4])],
                },
                serial: Stage::Out {
                    out: String::from("serial"),
                    args: IndexMap::new(),
                    inputs: vec![String::from("label")],
                },
            },
        };

        let got = document_from_runefile(runefile);

        for (key, should_be) in &should_be.pipeline {
            println!("{}", key);
            assert_eq!(&got.pipeline[key], should_be);
        }
        assert_eq!(got, should_be);
    }
}
