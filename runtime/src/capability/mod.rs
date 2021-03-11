use runic_types::{CAPABILITY, Value};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CapabilityRequest {
    pub c_type: CAPABILITY,
    pub params: HashMap<String, Value>,
}

impl CapabilityRequest {
    pub fn new(c_type: CAPABILITY) -> CapabilityRequest {
        return CapabilityRequest {
            c_type,
            params: HashMap::new(),
        };
    }
}
