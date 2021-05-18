use std::ops::Range;

use crate::{
    Diagnostics,
    ast::{
        self, Argument, ArgumentValue, CapabilityInstruction, FromInstruction,
        Instruction, Literal, LiteralKind, ModelInstruction, OutInstruction,
        ProcBlockInstruction, RunInstruction, Runefile,
    },
    yaml::{Document, Input, Path, Stage, Type, Value, utils},
};
use codespan::Span;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use indexmap::IndexMap;

pub fn document_from_runefile(runefile: Runefile) -> (Document, Diagnostics) {
    let mut diags = Diagnostics::new();

    let image = determine_image(&runefile.instructions, &mut diags);
    let mut pipeline = determine_pipeline(&runefile.instructions, &mut diags);

    connect_inputs(&runefile.instructions, &mut pipeline, &mut diags);

    (Document { image, pipeline }, diags)
}

fn connect_inputs(
    instructions: &[Instruction],
    pipeline: &mut IndexMap<String, Stage>,
    diags: &mut Diagnostics,
) {
    let run_instructions = instructions.iter().filter_map(|i| match i {
        Instruction::Run(r) => Some(r),
        _ => None,
    });

    for run in run_instructions {
        let RunInstruction { steps, .. } = run;

        // first we make sure all the steps exist
        let mut steps_missing = false;
        for step in steps {
            if !pipeline.contains_key(&step.value) {
                let label = Label::primary((), utils::range_span(step.span));
                diags.push(
                    Diagnostic::error()
                        .with_message(format!("Can't find \"{}\"", step.value,))
                        .with_labels(vec![label]),
                );

                steps_missing = true;
            }
        }

        if steps_missing {
            continue;
        }

        // now we can wire up their inputs
        for window in steps.windows(2) {
            let from = &window[0];
            let to = &window[1];
            let stage = &mut pipeline[&to.value];
            match stage.inputs_mut() {
                Some(inputs) => inputs.push(Input::new(&from.value, None)),
                None => {
                    let label = Label::primary((), utils::range_span(to.span));
                    diags.push(
                        Diagnostic::error()
                            .with_message(format!(
                                "The \"{}\" stage doesn't accept any inputs",
                                to.value
                            ))
                            .with_labels(vec![label]),
                    );
                },
            }

            if pipeline[&from.value].output_types().is_empty() {
                let label = Label::primary((), utils::range_span(from.span));
                diags.push(
                    Diagnostic::error()
                        .with_message(format!(
                            "The \"{}\" stage doesn't have any outputs",
                            from.value
                        ))
                        .with_labels(vec![label]),
                );
            }
        }
    }
}

fn determine_pipeline(
    instructions: &[Instruction],
    diags: &mut Diagnostics,
) -> IndexMap<String, Stage> {
    let mut pipeline = IndexMap::new();

    for instruction in instructions.to_vec() {
        let (name, stage, span) = match instruction {
            Instruction::From(_) => continue,
            Instruction::Model(m) => {
                let ModelInstruction {
                    name,
                    file,
                    output_type,
                    span,
                    ..
                } = m;
                let stage = Stage::Model {
                    model: file.clone(),
                    inputs: Vec::new(),
                    outputs: vec![convert_type(output_type)],
                };
                (name.value, stage, span)
            },
            Instruction::Capability(cap) => {
                let CapabilityInstruction {
                    kind,
                    name,
                    output_type,
                    parameters,
                    span,
                } = cap;

                let stage = Stage::Capability {
                    capability: kind.value,
                    outputs: vec![convert_type(output_type)],
                    args: convert_args(parameters),
                };
                (name.value, stage, span)
            },
            Instruction::Out(out) => {
                let OutInstruction { out_type, span } = out;
                let name = out_type.value.to_lowercase();
                let stage = Stage::Out {
                    out: out_type.value.clone(),
                    inputs: Vec::new(),
                    args: IndexMap::default(),
                };
                (name, stage, span)
            },
            Instruction::ProcBlock(pb) => {
                let ProcBlockInstruction {
                    path,
                    output_type,
                    name,
                    params,
                    span,
                    ..
                } = pb;

                let stage = Stage::ProcBlock {
                    proc_block: yaml_path(path),
                    inputs: Vec::new(),
                    outputs: vec![convert_type(output_type)],
                    args: convert_args(params),
                };
                (name.value, stage, span)
            },
            Instruction::Run(_) => continue,
        };

        if pipeline.contains_key(&name) {
            let label = Label::primary((), utils::range_span(span))
                .with_message("Duplicate defined here");
            let diag = Diagnostic::error()
                .with_message(format!(
                    "The \"{}\" stage is already defined",
                    name
                ))
                .with_labels(vec![label]);
            diags.push(diag);
        } else {
            pipeline.insert(name, stage);
        }
    }

    pipeline
}

fn determine_image(
    instructions: &[Instruction],
    diags: &mut Diagnostics,
) -> Path {
    let mut from: Option<&FromInstruction> = None;

    for instruction in instructions {
        if let Instruction::From(instruction) = instruction {
            if let Some(previous_image) = from {
                // Let the user know they used the FROM instruction twice
                let duplicate_label =
                    Label::primary((), utils::range_span(instruction.span))
                        .with_message("Duplicate defined here");
                let original_label = Label::secondary(
                    (),
                    utils::range_span(previous_image.span),
                )
                .with_message("Original defined here");

                let diag = Diagnostic::error()
                    .with_message("Base image already specified")
                    .with_labels(vec![duplicate_label, original_label]);
                diags.push(diag);
            }

            from = Some(instruction);
        }
    }

    match from {
        Some(from) => Path::from(&from.image),
        None => {
            diags.push(
                Diagnostic::error().with_message("No base image specified"),
            );
            Path::new("runicos/base", None, None)
        },
    }
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
                    inputs: vec!["model".parse().unwrap()],
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
                    inputs: vec!["audio".parse().unwrap()],
                    outputs: vec![ty!(I8[1960])],
                    args: IndexMap::new(),
                },
                model: Stage::Model {
                    model: String::from("./model.tflite"),
                    inputs: vec!["fft".parse().unwrap()],
                    outputs: vec![ty!(I8[4])],
                },
                serial: Stage::Out {
                    out: String::from("serial"),
                    args: IndexMap::new(),
                    inputs: vec!["label".parse().unwrap()],
                },
            },
        };

        let (got, diags) = document_from_runefile(runefile);

        assert!(!diags.has_errors());
        for (key, should_be) in &should_be.pipeline {
            println!("{}", key);
            assert_eq!(&got.pipeline[key], should_be);
        }
        assert_eq!(got, should_be);
    }
}
