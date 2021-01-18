

use std::collections::HashMap;
use runic_types::*;


#[derive(Debug, Clone)]
pub struct CapabilityRequest {
    pub capability: runic_types::CAPABILITY,
    // TODO: change to params::Value and do lifetime properly
    pub params: HashMap<String, String>
}


#[derive(Copy, Clone)]
pub struct Capability {
    pub name: runic_types::CAPABILITY,
    pub process: fn( bytes: Vec<u8>, param_type: runic_types::PARAM_TYPE ) -> Vec<u8>,
}

impl Capability {
    pub fn init(name: runic_types::CAPABILITY, process: fn(params: HashMap<String, Vec<u8>>) -> Vec<u8>) -> Self {
        Self { name, process }
    }
}