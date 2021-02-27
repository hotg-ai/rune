use runic_types::CAPABILITY;

mod rand;

pub trait Capability {
    fn get_type() -> CAPABILITY;
    fn request(
        params: std::collections::HashMap<String, CapabilityParam>,
    ) -> Vec<u8>;
}

#[derive(Debug, Clone)]
pub struct CapabilityParam {
    pub value: Vec<u8>,
    pub value_type: runic_types::PARAM_TYPE,
}

#[derive(Debug, Clone)]
pub struct CapabilityRequest {
    pub c_type: runic_types::CAPABILITY,
    pub params: std::collections::HashMap<String, CapabilityParam>,
}

impl CapabilityRequest {
    pub fn init(c_type: runic_types::CAPABILITY) -> CapabilityRequest {
        return CapabilityRequest {
            c_type,
            params: std::collections::HashMap::new(),
        };
    }

    pub fn set_param(
        &mut self,
        key: String,
        value: Vec<u8>,
        value_type: runic_types::PARAM_TYPE,
    ) {
        self.params
            .insert(key, CapabilityParam { value, value_type });
    }
}
