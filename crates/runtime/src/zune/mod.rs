use std::{
    collections::{HashMap, HashSet},
    io::{Cursor, Read},
};

use anyhow::{anyhow, Context, Error};
use bitflags::bitflags;
use indexmap::IndexMap;
use zip;

use crate::LoadError;

use self::tflite::ModelNode;
mod proc_block;
mod runefile;
mod tflite;

#[derive(Debug, Default, Clone)]
pub struct Tensor {
    element_type: ElementType,
    dimensions: Vec<u32>,
    buffer: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct TensorConstraint {
    element_types: ElementTypeConstraint,
    dimensions: DimensionsConstraint,
}

#[derive(Debug, Clone)]
pub struct TensorConstraints {
    inputs: IndexMap<String, TensorConstraint>,
    outputs: IndexMap<String, TensorConstraint>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum ElementType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    F32,
    U64,
    I64,
    F64,
    Complex64,
    Complex128,
    /// A string as UTF-8 encoded bytes.
    Utf8,
}

bitflags! {
    struct ElementTypeConstraint: u32 {
        const U8             = 1 << 0;
        const I8             = 1 << 1;
        const U16            = 1 << 2;
        const I16            = 1 << 3;
        const U32            = 1 << 4;
        const I32            = 1 << 5;
        const F32            = 1 << 6;
        const U64            = 1 << 7;
        const I64            = 1 << 8;
        const F64            = 1 << 9;
        const COMPLEX_64      = 1 << 10;
        const COMPLEX_128     = 1 << 11;
        const UTF8           = 1 << 12;
    }
}

#[derive(Debug, Clone)]
pub enum DimensionsConstraint {
    Dynamic,
    Fixed(Vec<u32>),
}

pub struct ZuneEngine {
    runefile: runefile::Document,
    input_tensor_names: HashMap<String, HashSet<String>>,
    output_tensor_names: HashMap<String, HashSet<String>>,
    processing_order: Vec<String>,

    nodes: HashMap<String, Box<dyn GraphNode>>,
    tensors: HashMap<usize, Tensor>,
    tensor_constraints: HashMap<usize, TensorConstraint>,
    input_tensor_mappings: HashMap<String, HashMap<String, usize>>,
    output_tensor_mappings: HashMap<String, HashMap<String, usize>>, // resources not yet implemented
}

pub(crate) trait GraphNode {
    fn load(
        node_id: &str,
        args: &HashMap<String, String>,
        node_data: &[u8],
    ) -> Result<Box<dyn GraphNode>, Error>
    where
        Self: Sized;
    fn node_id(&self) -> &str;
    fn tensor_constraints(&self) -> &TensorConstraints;
    fn run(
        &mut self,
        inputs: HashMap<&str, &Tensor>,
    ) -> Result<HashMap<&str, Tensor>, Error>;
}

impl ZuneEngine {
    #[tracing::instrument(skip_all)]
    pub fn load(binary: &[u8]) -> Result<Self, LoadError>
    where
        Self: Sized,
    {
        let mut archive = zip::ZipArchive::new(Cursor::new(binary))
            .context("Unable to load Zune")?;

        let mut read_zip_resource_by_path =
            |path: &str| -> Result<Vec<u8>, Error> {
                let mut requested_file =
                    archive.by_name(path).with_context(|| {
                        anyhow!("Unable to find {} in zune", path)
                    })?;
                let mut buffer = Vec::new();
                requested_file.read_to_end(&mut buffer).with_context(|| {
                    format!("Unable to read {} from zune", path)
                })?;
                Ok(buffer)
            };

        let runefile =
            String::from_utf8(read_zip_resource_by_path("Runefile.yml")?)
                .context("Unable to read Runefile")?;

        tracing::debug!(length = runefile.len(), "Read the Rune");

        let runefile = runefile::Document::parse(&runefile).map_err(|e| {
            LoadError::Other(anyhow!(
                "Unable to parse runefile: {}",
                e.to_string()
            ))
        })?;

        let processing_order =
            runefile.get_processing_order().map_err(|e| {
                LoadError::Other(anyhow!(
                    "Unable to determine processing order of the pipeline: {}",
                    e.to_string()
                ))
            })?;

        let mut nodes = HashMap::new();

        tracing::debug!(order=?processing_order, "Determined the execution order");

        // TODO: Validate and allocate input/output tensors
        // TODO: Add support for metadata in tflite
        // TODO: Add support for m3 in wit bindgen
        for (node_name, node_details) in &runefile.pipeline {
            let node_data = read_zip_resource_by_path(&node_details.uri)?;
            match node_details.ty {
                runefile::NodeType::Model => {
                    nodes.insert(
                        node_name.to_string(),
                        ModelNode::load(
                            node_name,
                            &node_details.args,
                            &node_data,
                        )?,
                    );
                },
                runefile::NodeType::ProcBlock => {
                    nodes.insert(
                        node_name.to_string(),
                        proc_block::ProcBlockNode::load(
                            node_name,
                            &node_details.args,
                            &node_data,
                        )?,
                    );
                },
            }
        }

        let tensors = HashMap::new();
        let (
            tensor_constraints,
            input_tensor_names,
            output_tensor_names,
            input_tensor_mappings,
            output_tensor_mappings,
        ) = get_tensor_constraints(&runefile, &nodes, &processing_order)?;

        Ok(ZuneEngine {
            runefile,
            input_tensor_names,
            output_tensor_names,
            processing_order,
            nodes,
            tensors,
            tensor_constraints,
            input_tensor_mappings,
            output_tensor_mappings,
        })
    }

    #[tracing::instrument(skip_all)]
    pub fn run(&mut self) -> Result<(), Error> {
        for node_name in &self.processing_order {
            let _span =
                tracing::debug_span!("Running Stage", %node_name).entered();

            let input_tensors: Result<HashMap<&str, &Tensor>, Error> =
                self.input_tensor_mappings[node_name]
                    .iter()
                    .map(|(tensor_name, tensor_id)|
                        Ok((tensor_name.as_str(), self.tensors
                                             .get(tensor_id)
                                             .ok_or_else(|| anyhow!("Tensor value not set: {node_name} {tensor_name}"))? )))
                    .collect();

            let node = self.nodes.get_mut(node_name).unwrap();

            // let outputs = node.run(input_tensors?)?;
        }
        Ok(())
    }

    pub fn input_tensor_names(&self) -> &HashMap<String, HashSet<String>> {
        &self.input_tensor_names
    }

    pub fn output_tensor_names(&self) -> &HashMap<String, HashSet<String>> {
        &self.output_tensor_names
    }

    // Just for compatibility with existing zune api
    pub fn input_nodes(&self) -> Vec<String> {
        self.input_tensor_names.keys().map(|k| k.to_string()).collect()
    }

    // Just for compatibility with existing zune api
    pub fn input_tensor_names_of_node(&self, node_name: &str) -> Option<&HashSet<String>> {
        self.input_tensor_names.get(node_name)
    }

    // Just for compatibility with existing zune api
    pub fn output_nodes(&self) -> Vec<String> {
        self.output_tensor_names.keys().map(|k| k.to_string()).collect()
    }

    // Just for compatibility with existing zune api
    pub fn output_tensor_names_of_node(&self, node_name: &str) -> Option<&HashSet<String>> {
        self.output_tensor_names.get(node_name)
    }

    pub fn dependent_nodes(&self, node_name: &str) -> Option<HashSet<String>> {
        let node = self.runefile.pipeline.get(node_name);
        match node {
            Some(node) => Some(
                node.inputs
                    .iter()
                    .map(|(_input_name, tensor)| tensor.node.to_string())
                    .collect(),
            ),
            None => None,
        }
    }

    pub fn get_tensor_constraints(
        &self,
        node_name: &str,
    ) -> Result<&TensorConstraints, Error> {
        let node = self
            .nodes
            .get(node_name)
            .ok_or_else(|| anyhow!("Unable to find node {}", node_name))?;
        Ok(node.tensor_constraints())
    }

    pub fn get_input_tensor_constraint(
        &self,
        node_name: &str,
        tensor_name: &str,
    ) -> Result<&TensorConstraint, Error> {
        let node = self
            .nodes
            .get(node_name)
            .ok_or_else(|| anyhow!("Unable to find node {}", node_name))?;
        node.tensor_constraints()
            .inputs
            .get(tensor_name)
            .ok_or_else(|| {
                anyhow!("Unable to find input tensor {tensor_name} in node")
            })
    }

    pub fn get_output_tensor_constraint(
        &self,
        node_name: &str,
        tensor_name: &str,
    ) -> Result<&TensorConstraint, Error> {
        let node = self
            .nodes
            .get(node_name)
            .ok_or_else(|| anyhow!("Unable to find node {}", node_name))?;
        node.tensor_constraints()
            .outputs
            .get(tensor_name)
            .ok_or_else(|| {
                anyhow!("Unable to find input tensor {tensor_name} in node")
            })
    }

    pub fn get_tensor(&self, tensor_id: &usize) -> Option<&Tensor> {
        self.tensors.get(tensor_id)
    }

    pub fn set_tensor(
        &mut self,
        tensor_id: usize,
        tensor: &Tensor,
    ) -> Result<(), Error> {
        let tensor_constraint =
            self.tensor_constraints.get(&tensor_id).ok_or_else(|| {
                anyhow!("Unable to find tensor with id: {tensor_id}")
            })?;
        tensor_constraint.is_satisfied(tensor)?;
        self.tensors.insert(tensor_id, tensor.clone());
        Ok(())
    }

    pub fn get_input_tensor(
        &self,
        node_name: &str,
        tensor_name: &str,
    ) -> Result<&Tensor, Error> {
        let tensor_id = self
            .input_tensor_mappings
            .get(node_name)
            .ok_or_else(|| anyhow!("Node not found: {node_name}"))?
            .get(tensor_name)
            .ok_or_else(|| anyhow!("Tensor not found: {tensor_name}"))?;
        Ok(self.get_tensor(tensor_id).unwrap())
    }

    pub fn set_input_tensor(
        &mut self,
        node_name: &str,
        tensor_name: &str,
        tensor: &Tensor,
    ) -> Result<(), Error> {
        let tensor_id = self
            .input_tensor_mappings
            .get(node_name)
            .ok_or_else(|| anyhow!("Node not found: {node_name}"))?
            .get(tensor_name)
            .ok_or_else(|| anyhow!("Tensor not found: {tensor_name}"))?;
        self.set_tensor(*tensor_id, tensor)
    }

    pub fn get_output_tensor(
        &mut self,
        node_name: &str,
        tensor_name: &str,
    ) -> Result<&Tensor, Error> {
        let tensor_id = self
            .output_tensor_mappings
            .get(node_name)
            .ok_or_else(|| anyhow!("Node not found: {node_name}"))?
            .get(tensor_name)
            .ok_or_else(|| anyhow!("Tensor not found: {tensor_name}"))?;
        Ok(self.get_tensor(tensor_id).unwrap())
    }

    pub fn set_output_tensor(
        &mut self,
        node_name: &str,
        tensor_name: &str,
        tensor: &Tensor,
    ) -> Result<(), Error> {
        let tensor_id = self
            .output_tensor_mappings
            .get(node_name)
            .ok_or_else(|| anyhow!("Node not found: {node_name}"))?
            .get(tensor_name)
            .ok_or_else(|| anyhow!("Tensor not found: {tensor_name}"))?;
        self.set_tensor(*tensor_id, tensor)
    }
}

fn get_buffer_size(element_type: ElementType, dimensions: &Vec<u32>) -> usize {
    (dimensions.iter().fold(1, |a, &b| a * b)
        * get_bytes_per_element(element_type)) as usize
}

fn get_bytes_per_element(element_type: ElementType) -> u32 {
    match element_type {
        ElementType::I16 => 2,
        ElementType::I32 | ElementType::F32 => 4,
        ElementType::I64 | ElementType::F64 => 8,
        _ => 1,
    }
}

// Allocate a global set of tensor_constraints
// And the associated mappings,
// Which map each tensor_constraint to nodes's inputs and outputs
fn get_tensor_constraints(
    runefile: &runefile::Document,
    nodes: &HashMap<String, Box<dyn GraphNode>>,
    processing_order: &Vec<String>,
) -> Result<
    (
        HashMap<usize, TensorConstraint>,
        HashMap<String, HashSet<String>>,
        HashMap<String, HashSet<String>>,
        HashMap<String, HashMap<String, usize>>,
        HashMap<String, HashMap<String, usize>>,
    ),
    Error,
> {
    let mut global_tensor_constraints = HashMap::new();
    let mut input_tensor_mappings: HashMap<String, HashMap<String, usize>> =
        HashMap::new();
    let mut output_tensor_mappings: HashMap<String, HashMap<String, usize>> =
        HashMap::new();
    let global_input_tensors = get_input_tensors(runefile, nodes);
    let global_output_tensors = get_output_tensors(runefile, nodes);

    let is_global_input_tensor = |node_name: &str, tensor_name: &str| -> bool {
        let node = global_input_tensors.get(node_name);
        match node {
            Some(node) => node.contains(tensor_name),
            None => false,
        }
    };

    // First allocate all the global input tensors
    for (node_name, input_tensors) in global_input_tensors.iter() {
        let mut tensors = HashMap::new();

        for tensor_name in input_tensors {
            let tensor_id = global_tensor_constraints.len();
            // No error checking because global_tensor_constraints already got tensor names from the same place
            let input_tensor_constraints =
                &nodes[node_name].tensor_constraints().inputs;
            let tensor_constraint = &input_tensor_constraints[tensor_name];
            global_tensor_constraints
                .insert(tensor_id, tensor_constraint.clone());
            tensors.insert(tensor_name.to_string(), tensor_id);
        }
        input_tensor_mappings.insert(node_name.to_string(), tensors);
    }

    // Then allocate all the output tensors of each nodes
    for node in processing_order {
        let tensor_constraints = nodes[node].tensor_constraints();
        let mut output_tensors = HashMap::new();

        for (tensor_name, tensor_constraint) in &tensor_constraints.outputs {
            let tensor_id = global_tensor_constraints.len();
            global_tensor_constraints
                .insert(tensor_id, tensor_constraint.clone());
            output_tensors.insert(tensor_name.to_string(), tensor_id);
        }

        output_tensor_mappings.insert(node.to_string(), output_tensors);
    }

    // Then simply walk through all the nodes and merge the input constraints, of each node with output tensor of the target
    for node_name in processing_order {
        let node_details = &runefile.pipeline[node_name];

        if !input_tensor_mappings.contains_key(node_name) {
            input_tensor_mappings.insert(node_name.to_string(), HashMap::new());
        }

        for (tensor_name, target) in &node_details.inputs {
            if !is_global_input_tensor(&node_name, &tensor_name) {
                let target_node = &target.node;
                let target_tensor_name = &target.tensor_name;
                let &target_tensor_index =
                    output_tensor_mappings
                        .get(&target.node)
                        .ok_or_else(|| anyhow!("Invalid tensor mapping: {node_name}::{tensor_name} --> {target_node}.{target_tensor_name}: {target_node} doesnt exist"))?
                        .get(&target.tensor_name)
                        .ok_or_else(|| anyhow!("Invalid tensor mapping: {node_name}::{tensor_name} --> {target_node}.{target_tensor_name}: {target_tensor_name} doesnt exist"))?;

                let target_constraint =
                    &global_tensor_constraints[&target_tensor_index];

                let current_tensor =
                    nodes[node_name]
                        .tensor_constraints()
                        .inputs.get(tensor_name)
                        .ok_or_else(|| anyhow!("Invalid tensor mapping: {node_name}::{tensor_name} --> {target_node}.{target_tensor_name}: {tensor_name} doesnt exist"))?;

                let merged_tensor = target_constraint
                    .merge(current_tensor)
                    .with_context(|| anyhow!("Unable to merge constraints: {node_name}::{tensor_name} --> {target_node}.{target_tensor_name}"))?;

                input_tensor_mappings
                    .get_mut(node_name)
                    .unwrap()
                    .insert(tensor_name.to_string(), target_tensor_index);

                global_tensor_constraints.insert(target_tensor_index, merged_tensor);
            }
        }
    }

    tracing::debug!("Input tensor mappings: {:?}", input_tensor_mappings);
    tracing::debug!("Output tensor mappings: {:?}", output_tensor_mappings);
    tracing::debug!("Tensor constraints: {:?}", global_tensor_constraints);

    Ok((
        global_tensor_constraints,
        global_input_tensors,
        global_output_tensors,
        input_tensor_mappings,
        output_tensor_mappings,
    ))
}

// Input tensors are those proc bloc tensors which aren't connected to other nodes' outputs
fn get_input_tensors(
    runefile: &runefile::Document,
    nodes: &HashMap<String, Box<dyn GraphNode>>,
) -> HashMap<String, HashSet<String>> {
    let mut result = HashMap::new();

    for (node_name, node) in nodes {
        let node_tensors: HashSet<String> = node
            .tensor_constraints()
            .inputs
            .iter()
            .map(|(tensor_name, _)| tensor_name.to_string())
            .collect();

        // No error checking here because we created nodes based on the pipeline and node_name will always be found
        let mapped_tensors: HashSet<String> = runefile.pipeline[node_name]
            .inputs
            .iter()
            .map(|(tensor_name, _)| tensor_name.to_string())
            .collect();

        let unmapped_tensors: HashSet<String> = node_tensors
            .difference(&mapped_tensors)
            .map(|x| x.to_string())
            .collect();
        if !unmapped_tensors.is_empty() {
            result.insert(node_name.to_string(), unmapped_tensors);
        }
    }

    result
}

// Output tensors are those proc bloc tensors which aren't connected to any other nodes as inputs
fn get_output_tensors(
    runefile: &runefile::Document,
    nodes: &HashMap<String, Box<dyn GraphNode>>,
) -> HashMap<String, HashSet<String>> {
    let mut result: HashMap<String, HashSet<String>> = nodes
        .iter()
        .map(|(node_name, node)| {
            (
                node_name.to_string(),
                node.tensor_constraints()
                    .outputs
                    .iter()
                    .map(|(tensor_name, _)| tensor_name.to_string())
                    .collect(),
            )
        })
        .collect();

    for (_node_name, node) in &runefile.pipeline {
        for (_input_tensor_name, target) in &node.inputs {
            result
                .get_mut(&target.node)
                .unwrap()
                .remove(&target.tensor_name);
        }
    }

    result
        .iter()
        .filter_map(|(node_name, outputs)| {
            if outputs.is_empty() {
                None
            } else {
                Some((node_name.to_string(), outputs.clone()))
            }
        })
        .collect()
}

impl Default for ElementType {
    fn default() -> Self {
        ElementType::U8
    }
}

impl TensorConstraint {
    fn merge(
        &self,
        other: &TensorConstraint,
    ) -> Result<TensorConstraint, Error> {
        let element_types = ElementTypeConstraint::from_bits_truncate(
            self.element_types.bits() & other.element_types.bits(),
        );

        if element_types.is_empty() {
            return Err(anyhow!("Incompatible element types: "));
        }

        let dimensions = match (&self.dimensions, &other.dimensions) {
            (DimensionsConstraint::Dynamic, DimensionsConstraint::Dynamic) => {
                Ok(DimensionsConstraint::Dynamic)
            },
            (DimensionsConstraint::Fixed(t), DimensionsConstraint::Dynamic) => {
                Ok(DimensionsConstraint::Fixed(t.clone()))
            },
            (DimensionsConstraint::Dynamic, DimensionsConstraint::Fixed(t)) => {
                Ok(DimensionsConstraint::Fixed(t.clone()))
            },
            (
                DimensionsConstraint::Fixed(a),
                DimensionsConstraint::Fixed(b),
            ) if a == b => Ok(DimensionsConstraint::Fixed(a.clone())),
            _ => Err(anyhow!(
                "Dimensions mismatch {:?} vs. {:?}", self.dimensions, other.dimensions
            )),
        }?;

        Ok(TensorConstraint {
            element_types,
            dimensions,
        })
    }
    fn is_satisfied(&self, tensor: &Tensor) -> Result<(), Error> {
        let element_type_value = match tensor.element_type {
            ElementType::U8 => 1 << 0,
            ElementType::I8 => 1 << 1,
            ElementType::U16 => 1 << 2,
            ElementType::I16 => 1 << 3,
            ElementType::U32 => 1 << 4,
            ElementType::I32 => 1 << 5,
            ElementType::F32 => 1 << 6,
            ElementType::U64 => 1 << 7,
            ElementType::I64 => 1 << 8,
            ElementType::F64 => 1 << 9,
            ElementType::Complex64 => 1 << 10,
            ElementType::Complex128 => 1 << 11,
            ElementType::Utf8 => 1 << 12,
        };

        if self.element_types.bits() & element_type_value == 0 {
            return Err(anyhow!(
                "Tensor Element type mismatch: Expecting {:?}. Received {:?}",
                self.element_types,
                tensor.element_type
            ));
        }

        if let DimensionsConstraint::Fixed(t) = &self.dimensions {
            if t != &tensor.dimensions {
                return Err(anyhow!(
                    "Dimensions mismatch: Expecting {:?}. Received {:?}",
                    self.dimensions,
                    tensor.dimensions
                ));
            }
        }

        let expected_buffer_size =
            get_buffer_size(tensor.element_type, &tensor.dimensions);

        if expected_buffer_size != tensor.buffer.len() {
            return Err(anyhow!(
                "Tensor Buffer size mismatch: Expecting {:?}. Received {:?}",
                expected_buffer_size,
                tensor.buffer.len()
            ));
        }

        Ok(())
    }
}
