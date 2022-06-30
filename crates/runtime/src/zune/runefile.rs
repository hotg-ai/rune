use anyhow::{anyhow, Context};
use serde::{Serialize, Deserialize, Deserializer};
use serde_yaml;
use std::{
    option::Option,
    collections::HashMap,
    collections::HashSet,
};
use core::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    pub version: usize,
    pub pipeline: HashMap<String, Node>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    #[serde(rename = "type")]
    pub ty: NodeType,
    pub uri: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub args: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub inputs: HashMap<String, FullyQualifiedTensorName>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NodeType {
    #[serde(rename = "model")]
    Model,
    #[serde(rename = "proc-block")]
    ProcBlock,
}

#[derive(Serialize, Debug)]
pub struct FullyQualifiedTensorName {
    pub node: String,
    // TODO: Make tensor_name optional - can be useful when trying to join the first output of a given node
    pub tensor_name: String
}

impl<'de> Deserialize<'de> for FullyQualifiedTensorName {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
      let raw = String::deserialize(de)?;
      raw.parse().map_err(serde::de::Error::custom)
    }
}

impl FromStr for FullyQualifiedTensorName {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, anyhow::Error> {
      let dot = s.find(".").with_context(|| anyhow!("Unable to find tensor_name in string: {}", s))?;
      let (node, tensor_name) = s.split_at(dot);
      Ok(FullyQualifiedTensorName { node: node.to_string(), tensor_name: tensor_name[1..].to_string() })
    }
}

impl Document {
    pub fn parse(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }

    // Input nodes are defined as nodes which rely on no other nodes
    pub fn get_input_nodes(&self) -> HashSet<String> {
        self.pipeline.iter()
            .filter_map(|(k, v)| if v.inputs.len() == 0 { Some(k.to_string()) } else { None })
            .collect()
    }

    // Output nodes are defined as nodes which no other nodes rely on
    pub fn get_output_nodes(&self) -> HashSet<String> {
        let mut result: HashSet<String> = self.pipeline.keys().map(|k| k.to_string()).collect();

        for (_, node) in &self.pipeline {
            for (_, input) in node.inputs.iter() {
                result.remove(&input.node);
            }
        }

        result.iter()
            .map(|i| i.to_string())
            .collect()
    }

    // Processing order of this runefile is a simple topological sort, starting from all the output nodes
    pub fn get_processing_order(&self) -> Result<Vec<String>, anyhow::Error> {
        let mut nodes_to_visit: Vec<String> = self.get_output_nodes().iter().map(|e| e.to_string()).collect();
        let mut nodes_visited = Vec::new();

        while !nodes_to_visit.is_empty() {
            let current_node = nodes_to_visit.pop().unwrap();
            nodes_visited.push(current_node.clone());

            for (_, dependency) in &self.pipeline[&current_node].inputs {
                let d = &dependency.node;
                if !self.pipeline.contains_key(d) {
                    return Err(anyhow!("Unable to process node: {} because dependency {} is not found in the pipeline", current_node, d));
                }
                if !nodes_visited.contains(d) && !nodes_to_visit.contains(d) {
                    nodes_to_visit.push(d.to_string());
                }
            }
        }

        nodes_visited.reverse();
        Ok(nodes_visited)
    }
}