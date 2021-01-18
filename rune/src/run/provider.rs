
use std::collections::HashMap;

use runic_types::*;
use crate::capability::*;


impl ProviderResponse {
    pub fn to_bytes(&self) -> Vec<u8> {
        let result = bincode::serialize(self);

        let mbytes = match result {
            Ok(serialized_manifest) => serialized_manifest,
            Err(error) => vec![0]
        };

        return mbytes;
    }

    pub fn from_bytes(buf: Vec<u8>) -> ProviderResponse {
        let manifest = bincode::deserialize(&buf[..]);

        let manifest = match manifest {
            Ok(m) => m,
            Err(_err) => ProviderResponse{inputs: vec![], err: None}
        };

        return manifest
    }

    pub fn from_slice(buf: &[u8]) -> ProviderResponse {
        let manifest = bincode::deserialize(&buf[..]);

        let manifest = match manifest {
            Ok(m) => m,
            Err(_err) => ProviderResponse{inputs: vec![], err: None}
        };

        return manifest
    }
}

pub struct Provider {
    pub capabilities: HashMap<CAPABILITY, Capability>,
}

impl Provider {
    pub fn register_capability(&mut self, capability: Capability) {
        self.capabilities.insert(capability.name, capability);
    }

    pub fn execute_capability(&self, capability_request: &CapabilityRequest) -> Vec<u8> {
        let capability = capability_request.capability;
        let capability = self.capabilities.get(&capability);

        let capability = capability.unwrap();

        return (capability.process)(capability_request);
    }
}