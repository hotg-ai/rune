

use serde::{Deserialize, Serialize};

use std::collections::HashMap;


#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum CAPABILITY {
    AUDIO,
    RAND
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CapabilityRequest {
    pub capability: CAPABILITY,
    // TODO: change to params::Value and do lifetime properly
    pub params: HashMap<String, String>
}

#[derive(Serialize, Deserialize)]
pub struct CapabilityResponse {
    pub name: CAPABILITY,
    pub input: Vec<u8>
}

#[derive(Copy, Clone)]
pub struct Capability {
    pub name: CAPABILITY,
    pub process: fn(&CapabilityRequest) -> Vec<u8>,
}

impl Capability {
    pub fn init(name: CAPABILITY, process: fn(&CapabilityRequest) -> Vec<u8>) -> Self {
        Self { name, process }
    }
}