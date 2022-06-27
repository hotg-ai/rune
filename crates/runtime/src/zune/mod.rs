use std::{
    io::{Cursor, Read},
    collections::{HashMap, HashSet}
};

use anyhow::{anyhow, Context, Error};
use zip;
use bitflags::bitflags;
use indexmap::IndexMap;

use crate::LoadError;

use self::tflite::ModelNode;
mod runefile;
mod tflite;
mod proc_block;

#[derive(Debug, Default, Clone)]
pub struct Tensor {
    element_type: ElementType,
    dimensions: Vec<u32>,
    buffer: Vec<u8>
}

#[derive(Debug, Clone)]
pub struct TensorConstraint {
    element_types: ElementTypeConstraint,
    dimensions: DimensionsConstraint
}

#[derive(Debug, Clone)]
pub struct TensorConstraints {
    inputs: IndexMap<String, TensorConstraint>,
    outputs: IndexMap<String, TensorConstraint>
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
    Utf8
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
    Fixed(Vec<u32>)
}

pub struct ZuneEngine {
    runefile: runefile::Document,
    input_nodes: HashSet<String>,
    output_nodes: HashSet<String>,
    processing_order: Vec<String>,

    nodes: HashMap<String, Box<dyn GraphNode>>,
    tensors: Vec<Option<Tensor>>,
    tensor_constraints: Vec<TensorConstraint>,
    input_tensor_mappings: HashMap<String, Vec<usize>>,
    // resources not yet implemented
}

pub(crate) trait GraphNode {
    fn load(node_id: &str, args: &HashMap<String, String>, node_data: &[u8]) -> Result<Box<dyn GraphNode>, Error> where Self: Sized;
    fn node_id(&self) -> &str;
    fn tensor_constraints(&self) -> &TensorConstraints;
    fn run(&mut self, inputs: HashMap<&str, &Tensor>) -> Result<HashMap<&str, Tensor>, Error>;
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

        let runefile =
            runefile::Document::parse(&runefile).map_err(|e| LoadError::Other(anyhow!("Unable to parse runefile: {}", e.to_string())))?;

        let input_nodes = runefile.get_input_nodes();
        let output_nodes = runefile.get_output_nodes();
        let processing_order = runefile.get_processing_order()
            .map_err(|e| LoadError::Other(anyhow!("Unable to determine processing order of the pipeline: {}", e.to_string())))?;
        let tensors = Vec::new();
        let tensor_constraints = Vec::new();
        let input_tensor_mappings = HashMap::new();
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
                        ModelNode::load(node_name, &node_details.args, &node_data)?
                    );
                },
                runefile::NodeType::ProcBlock => {
                    nodes.insert(
                        node_name.to_string(),
                        proc_block::ProcBlockNode::load(node_name, &node_details.args, &node_data)?
                    );
                }
            }
        }

        Ok(ZuneEngine {
            runefile,
            input_nodes,
            output_nodes,
            processing_order,
            nodes,
            tensors,
            tensor_constraints,
            input_tensor_mappings
        })
    }

    #[tracing::instrument(skip_all)]
    pub fn run(&mut self) -> Result<(), Error> {
        for stage_name in &self.processing_order {
            let _span =
                tracing::debug_span!("Running Stage", %stage_name).entered();

            // if let Some(node) = self.nodes.get_mut(stage_name) {
            //     node.run()?;
            // }
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
            Some(node) => {
                Some(node.inputs.iter().map(|(_input_name, tensor)| tensor.node.to_string()).collect())
            },
            None => None
        }
    }

    pub fn get_tensor_constraints(
        &self,
        node_name: &str,
    ) -> Result<&TensorConstraints, Error> {
        let node = self.nodes.get(node_name).ok_or_else( || anyhow!("Unable to find node {}", node_name))?;
        Ok(node.tensor_constraints())
    }

    pub fn get_input_tensor(
        &self,
        node_name: &str,
        tensor_name: &str,
    ) -> Option<Tensor> {
        todo!()
    }

    pub fn set_input_tensor(
        &mut self,
        node_name: &str,
        tensor_name: &str,
        tensor: &Tensor,
    ) {
        todo!()
    }

    pub fn get_output_tensor(
        &mut self,
        node_name: &str,
        tensor_name: &str,
    ) -> Option<Tensor> {
        todo!()
    }

    // pub fn get_tensor(&self, tensor_id: usize) -> Option<&TensorResult> {
    //     self.shared_state
    //         .lock()
    //         .unwrap()
    //         .tensors
    //         .get(tensor_id)
    //         .unwrap_or(&None)
    //         .as_ref()
    // }

    // pub fn set_tensor(&mut self, tensor_id: usize, tensor: &TensorResult) -> Result<(), Error> {
    //     self.shared_state
    //         .lock()
    //         .unwrap()
    //         .tensors
    //         .get_mut(tensor_id)
    //         .and_then(|t| { t = Some(tensor.clone()); Ok() })
    //         .ok()
    // }

    pub fn set_output_tensor(
        &mut self,
        node_name: &str,
        tensor_name: &str,
        tensor: &Tensor,
    ) {
        todo!()
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

fn key(node_name: &str, tensor_index: Option<usize>) -> String {
    format!("{}.{}", node_name, tensor_index.or(Some(0)).unwrap())
}

impl Default for ElementType {
    fn default() -> Self { ElementType::U8 }
}