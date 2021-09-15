use std::collections::HashMap;
use codespan_reporting::diagnostic::Diagnostic;
use hotg_rune_core::{Shape, reflect::Type};
use legion::{Entity, systems::CommandBuffer};

use crate::{
    Diagnostics,
    lowering::{Inputs, NameTable, Outputs, Tensor},
    parse::{self, DocumentV1},
};

/// Register all [`Tensor`]s and associate them as node [`Inputs`] or
/// [`Outputs`].
#[legion::system]
pub(crate) fn run(
    cmd: &mut CommandBuffer,
    #[resource] names: &NameTable,
    #[resource] doc: &DocumentV1,
    #[resource] diags: &mut Diagnostics,
) {
    let output_tensors_by_node = register_outputs(cmd, names, doc, diags);
    register_inputs(doc, names, output_tensors_by_node, cmd, diags);
}

fn register_inputs(
    doc: &DocumentV1,
    names: &NameTable,
    output_tensors_by_node: HashMap<Entity, Outputs>,
    cmd: &mut CommandBuffer,
    diags: &mut Diagnostics,
) {
    for (name, stage) in &doc.pipeline {
        let ent = match names.get(name) {
            Some(&e) => e,
            None => continue,
        };

        match register_stage_inputs(
            name,
            stage.inputs(),
            names,
            &output_tensors_by_node,
        ) {
            Ok(inputs) if inputs.tensors.is_empty() => {},
            Ok(inputs) => cmd.add_component(ent, inputs),
            Err(diag) => diags.push(diag),
        }
    }
}

fn register_stage_inputs(
    parent_name: &str,
    inputs: &[parse::Input],
    names: &NameTable,
    output_tensors_by_node: &HashMap<Entity, Outputs>,
) -> Result<Inputs, Diagnostic<()>> {
    let mut tensors = Vec::new();

    for input in inputs {
        let tensor = get_input_tensor(
            parent_name,
            input,
            names,
            output_tensors_by_node,
        )?;
        tensors.push(tensor);
    }

    Ok(Inputs { tensors })
}

fn get_input_tensor(
    parent_name: &str,
    input: &parse::Input,
    names: &NameTable,
    output_tensors_by_node: &HashMap<Entity, Outputs>,
) -> Result<Entity, Diagnostic<()>> {
    // Find the node this "Input" refers to
    let input_node = names
        .get(&input.name)
        .copied()
        .ok_or_else(|| unknown_input_name_diagnostic(parent_name, input))?;

    // Then get its set of Outputs
    let output_tensors = output_tensors_by_node
        .get(&input_node)
        .ok_or_else(|| node_has_no_outputs_diagnostic(parent_name, input))?;

    // Finally, get the Entity for the index'th item
    let tensor = output_tensors
        .tensors
        .get(input.index.unwrap_or(0))
        .copied()
        .ok_or_else(|| no_such_output_diagnostic(input))?;

    Ok(tensor)
}

fn no_such_output_diagnostic(input: &parse::Input) -> Diagnostic<()> {
    Diagnostic::error().with_message(format!(
        "The \"{}\" node has no {}'th output",
        input.name,
        input.index.unwrap_or(0)
    ))
}

fn node_has_no_outputs_diagnostic(
    parent_name: &str,
    input: &parse::Input,
) -> Diagnostic<()> {
    Diagnostic::error().with_message(format!(
        "The \"{}\" in {}'s \"{}\" input has no inputs",
        input.name, parent_name, input,
    ))
}

fn unknown_input_name_diagnostic(
    parent_name: &str,
    input: &parse::Input,
) -> Diagnostic<()> {
    Diagnostic::error().with_message(format!(
        "The \"{}\" in {}'s \"{}\" input was not defined",
        input.name, parent_name, input,
    ))
}

fn register_outputs(
    cmd: &mut CommandBuffer,
    names: &NameTable,
    doc: &DocumentV1,
    diags: &mut Diagnostics,
) -> HashMap<Entity, Outputs> {
    let mut node_to_output_tensors = HashMap::new();

    for (name, stage) in &doc.pipeline {
        let ent = match names.get(name) {
            Some(&e) => e,
            None => continue,
        };

        match allocate_output_tensors(cmd, stage.output_types()) {
            Ok(outputs) if outputs.tensors.is_empty() => {},
            Ok(outputs) => {
                node_to_output_tensors.insert(ent, outputs.clone());
                cmd.add_component(ent, outputs);
            },
            Err(diag) => diags.push(diag),
        }
    }

    node_to_output_tensors
}

/// Allocate new entities for each output.
fn allocate_output_tensors(
    cmd: &mut CommandBuffer,
    output_types: &[parse::Type],
) -> Result<Outputs, Diagnostic<()>> {
    let mut outputs = Vec::new();

    for ty in output_types {
        let tensor = shape(ty)?;
        outputs.push(cmd.push((tensor,)));
    }

    Ok(Outputs { tensors: outputs })
}

fn shape(ty: &parse::Type) -> Result<Tensor, Diagnostic<()>> {
    let element_type = match ty.name.as_str() {
        "u8" | "U8" => Type::u8,
        "i8" | "I8" => Type::i8,
        "u16" | "U16" => Type::u16,
        "i16" | "I16" => Type::i16,
        "u32" | "U32" => Type::u32,
        "i32" | "I32" => Type::i32,
        "f32" | "F32" => Type::f32,
        "u64" | "U64" => Type::u64,
        "i64" | "I64" => Type::i64,
        "f64" | "F64" => Type::f64,
        "utf8" | "UTF8" => Type::str,
        _ => return Err(unknown_element_type_diagnostic(&ty.name)),
    };

    Ok(Tensor::from(Shape::new(
        element_type,
        ty.dimensions.clone(),
    )))
}

fn unknown_element_type_diagnostic(name: &str) -> Diagnostic<()> {
    Diagnostic::error()
        .with_message(format!("Unknown element type, \"{}\"", name))
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use legion::{World, IntoQuery};
    use crate::{
        BuildContext,
        phases::{self, Phase},
        lowering,
    };
    use super::*;

    fn doc() -> DocumentV1 {
        DocumentV1 {
            image: "image".parse().unwrap(),
            pipeline: map! {
                rand: parse::Stage::Capability {
                    capability: "RAND".to_string(),
                    outputs: vec![
                        ty!(f32[128]),
                    ],
                    args: map! {},
                },
                transform: parse::Stage::ProcBlock {
                    proc_block: "proc-block".parse().unwrap(),
                    inputs: vec![
                        "rand".parse().unwrap(),
                    ],
                    outputs: vec![
                        ty!(u8[1]),
                        ty!(u8[2]),
                    ],
                    args: map! {},
                },
                output: parse::Stage::Out {
                    out: "SERIAL".to_string(),
                    inputs: vec![
                        "transform.1".parse().unwrap(),
                        "transform.0".parse().unwrap(),
                    ],
                    args: map! {},
                }
            },
            resources: map! {},
        }
    }

    #[test]
    fn construct_pipeline() {
        let mut world = World::default();
        let mut res =
            phases::initialize_resources(BuildContext::from_doc(doc().into()));
        crate::parse::phase().run(&mut world, &mut res);

        Phase::new()
            .and_then(lowering::register_names::run_system())
            .and_then(lowering::update_nametable::run_system())
            .and_then(lowering::register_stages::run_system())
            .and_then(run_system())
            .run(&mut world, &mut res);

        let diags = res.get::<Diagnostics>().unwrap();
        assert!(diags.is_empty());

        let names = res.get::<NameTable>().unwrap();
        let connections = vec![
            (("rand", 0), ("transform", 0), "f32[128]"),
            (("transform", 0), ("output", 1), "u8[1]"),
            (("transform", 1), ("output", 0), "u8[2]"),
        ];
        let mut inputs = <&Inputs>::query();
        let mut outputs = <&Outputs>::query();
        let mut tensors = <&Tensor>::query();

        for ((prev_name, prev_ix), (next_name, next_ix), ty) in connections {
            let ty_should_be = Tensor::from(Shape::from_str(ty).unwrap());

            let prev = names[prev_name];
            let outputs = outputs.get(&world, prev).unwrap();
            let output_tensor = outputs.tensors[prev_ix];
            assert_eq!(
                tensors.get(&world, output_tensor).unwrap(),
                &ty_should_be
            );

            let next = names[next_name];
            let inputs = inputs.get(&world, next).unwrap();
            let input_tensor = inputs.tensors[next_ix];
            assert_eq!(
                tensors.get(&world, input_tensor).unwrap(),
                &ty_should_be
            );

            assert_eq!(input_tensor, output_tensor);
        }
    }
}
