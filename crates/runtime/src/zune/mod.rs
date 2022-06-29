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
    input_nodes: HashSet<String>,
    output_nodes: HashSet<String>,
    processing_order: Vec<String>,

    nodes: HashMap<String, Box<dyn GraphNode>>,
    tensors: HashMap<usize, Tensor>,
    tensor_constraints: HashMap<usize, TensorConstraint>,
    input_tensor_mappings: HashMap<String, usize>,
    output_tensor_mappings: HashMap<String, usize>, // resources not yet implemented
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

        let input_nodes = runefile.get_input_nodes();
        let output_nodes = runefile.get_output_nodes();
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
        let (tensor_constraints, input_tensor_mappings, output_tensor_mappings) =
            get_tensor_constraints(
                &runefile,
                &nodes,
                &processing_order,
                &input_nodes,
                &output_nodes,
            )?;

        println!("Tensor constraints: {:?}", tensor_constraints);

        Ok(ZuneEngine {
            runefile,
            input_nodes,
            output_nodes,
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

            let node = self.nodes.get(node_name).unwrap();

            let input_tensors: Result<HashMap<&str, &Tensor>, Error> = node
                .tensor_constraints()
                .inputs
                .iter()
                .map(|(tensor_name, _)| -> Result<(&str, &Tensor), Error> {
                    let tensor = self.get_input_tensor(node_name, tensor_name)?.ok_or_else(|| anyhow!("Input tensor not set: {node_name}.{tensor_name}"))?;
                    Ok((tensor_name, tensor))
                })
                .collect();

            let outputs = node.run(input_tensors?)?;
        }
        Ok(())
    }

    pub fn input_nodes(&self) -> &HashSet<String> {
        return &self.input_nodes;
    }

    pub fn output_nodes(&self) -> &HashSet<String> {
        return &self.output_nodes;
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
    ) -> Result<Option<&Tensor>, Error> {
        let k = key(node_name, tensor_name);
        let tensor_id = self
            .input_tensor_mappings
            .get(&k)
            .ok_or_else(|| anyhow!("Tensor not found for {k}"))?;
        Ok(self.get_tensor(tensor_id))
    }

    pub fn set_input_tensor(
        &mut self,
        node_name: &str,
        tensor_name: &str,
        tensor: &Tensor,
    ) -> Result<(), Error> {
        let k = key(node_name, tensor_name);
        let tensor_id = self
            .input_tensor_mappings
            .get(&k)
            .ok_or_else(|| anyhow!("Tensor not found for {k}"))?;
        self.set_tensor(*tensor_id, tensor)
    }

    pub fn get_output_tensor(
        &mut self,
        node_name: &str,
        tensor_name: &str,
    ) -> Option<&Tensor> {
        let tensor_id = self
            .output_tensor_mappings
            .get(&key(node_name, tensor_name));
        match tensor_id {
            Some(id) => self.get_tensor(id),
            None => None,
        }
    }

    pub fn set_output_tensor(
        &mut self,
        node_name: &str,
        tensor_name: &str,
        tensor: &Tensor,
    ) -> Result<(), Error> {
        let k = key(node_name, tensor_name);
        let tensor_id = self
            .output_tensor_mappings
            .get(&k)
            .ok_or_else(|| anyhow!("Tensor not found for {k}"))?;
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

fn key(node_name: &str, tensor_name: &str) -> String {
    format!("{}.{}", node_name, tensor_name)
}

fn get_tensor_constraints(
    runefile: &runefile::Document,
    nodes: &HashMap<String, Box<dyn GraphNode>>,
    processing_order: &Vec<String>,
    input_nodes: &HashSet<String>,
    output_nodes: &HashSet<String>,
) -> Result<
    (
        HashMap<usize, TensorConstraint>,
        HashMap<String, usize>,
        HashMap<String, usize>,
    ),
    Error,
> {
    let mut merged_tensor_constraints = HashMap::new();
    let mut input_tensor_mappings: HashMap<String, usize> = HashMap::new();
    let mut output_tensor_mappings: HashMap<String, usize> = HashMap::new();

    // First allocate all the output tensors and input tensors of input nodes
    // TODO: Change the definition of input nodes/output nodes to accomodate for partially connected nodes?
    for node in processing_order {
        let tensor_constraints = nodes[node].tensor_constraints();

        if input_nodes.contains(node.as_str()) {
            for (tensor_name, tensor_constraint) in &tensor_constraints.inputs {
                let tensor_id = merged_tensor_constraints.len();
                let k = key(node, tensor_name);
                merged_tensor_constraints
                    .insert(tensor_id, tensor_constraint.clone());
                input_tensor_mappings.insert(k, tensor_id);
            }
        }

        for (tensor_name, tensor_constraint) in &tensor_constraints.outputs {
            let tensor_id = merged_tensor_constraints.len();
            let k = key(node, tensor_name);
            merged_tensor_constraints
                .insert(tensor_id, tensor_constraint.clone());
            output_tensor_mappings.insert(k, tensor_id);
        }

        // TODO: Support reading runefiles where an input tensor is specified by a node alone.
        // Like sine { input: mod360 } - Then simply take the first output and plug it in
    }

    let mut merge_constraints = |node: &str,
                                 input_tensor: &str,
                                 target_node: &str,
                                 target_tensor: &str,
                                 constraint: &TensorConstraint|
     -> Result<(), Error> {
        let output_tensor_key = key(target_node, target_tensor);
        let &existing_constraint_index = output_tensor_mappings
            .get(&output_tensor_key)
            .ok_or_else(|| anyhow!("{node}.{input_tensor}'s target tensor not found: {target_node} {target_tensor} {output_tensor_key}"))?;
        let existing_constraint =
            merged_tensor_constraints[&existing_constraint_index].clone();
        let element_types = ElementTypeConstraint::from_bits_truncate(
            existing_constraint.element_types.bits()
                & constraint.element_types.bits(),
        );
        let dimensions = match (&existing_constraint.dimensions, &constraint.dimensions) {
            (DimensionsConstraint::Dynamic, DimensionsConstraint::Dynamic) => Ok(DimensionsConstraint::Dynamic),
            (DimensionsConstraint::Fixed(t), DimensionsConstraint::Dynamic) => Ok(DimensionsConstraint::Fixed(t.clone())),
            (DimensionsConstraint::Dynamic, DimensionsConstraint::Fixed(t)) => Ok(DimensionsConstraint::Fixed(t.clone())),
            (DimensionsConstraint::Fixed(a), DimensionsConstraint::Fixed(b)) if a == b => Ok(DimensionsConstraint::Fixed(a.clone())),
            _ => Err(anyhow!("{node}.{input_tensor}'s target {output_tensor_key} dimensions mismatch"))
        }?;

        input_tensor_mappings
            .insert(key(node, input_tensor), existing_constraint_index);
        merged_tensor_constraints.insert(
            existing_constraint_index,
            TensorConstraint {
                element_types,
                dimensions,
            },
        );

        Ok(())
    };

    // Then simply walk through all the nodes and merge the input constraints, of the
    for node_name in processing_order {
        if !input_nodes.contains(node_name.as_str()) {
            let node_details = &runefile.pipeline[node_name];
            let current_node_constraints =
                &nodes[node_name].tensor_constraints().inputs;

            for (input_name, target) in &node_details.inputs {
                let current_input_constraint =
                    current_node_constraints.get(input_name);
                match current_input_constraint {
                    Some(constraint) => {
                        merge_constraints(
                            node_name,
                            input_name,
                            &target.node,
                            &target.tensor_name,
                            constraint,
                        )?;
                    },
                    None => {
                        println!("{node_name} does not contain input tensor: {input_name}. Ignoring...");
                    },
                }
            }
        }
    }

    tracing::debug!("Input tensor mappings: {:?}", input_tensor_mappings);
    tracing::debug!("Output tensor mappings: {:?}", output_tensor_mappings);

    Ok((
        merged_tensor_constraints,
        input_tensor_mappings,
        output_tensor_mappings,
    ))
}

impl Default for ElementType {
    fn default() -> Self {
        ElementType::U8
    }
}

impl TensorConstraint {
    fn is_satisfied(&self, tensor: &Tensor) -> Result<(), Error> {
        let element_type_value = match (tensor.element_type) {
            U8 => 1 << 0,
            I8 => 1 << 1,
            U16 => 1 << 2,
            I16 => 1 << 3,
            U32 => 1 << 4,
            I32 => 1 << 5,
            F32 => 1 << 6,
            U64 => 1 << 7,
            I64 => 1 << 8,
            F64 => 1 << 9,
            Complex64 => 1 << 10,
            Complex128 => 1 << 11,
            Utf8 => 1 << 12,
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
