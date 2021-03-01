use anyhow::Error;
use runic_types::{CAPABILITY, PARAM_TYPE};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum CapabilityParam {
    Integer(i64),
    Float(f64),
    Utf8(String),
    Binary(Vec<u8>),
}

impl CapabilityParam {
    pub fn from_raw(raw: Vec<u8>, ty: PARAM_TYPE) -> Result<Self, Error> {
        match ty {
            PARAM_TYPE::INT => match raw.len() {
                2 => {
                    let mut buffer = [0; 2];
                    buffer.copy_from_slice(&raw);
                    Ok(CapabilityParam::Integer(
                        i16::from_le_bytes(buffer) as i64
                    ))
                },
                4 => {
                    let mut buffer = [0; 4];
                    buffer.copy_from_slice(&raw);
                    Ok(CapabilityParam::Integer(
                        i32::from_le_bytes(buffer) as i64
                    ))
                },
                8 => {
                    let mut buffer = [0; 8];
                    buffer.copy_from_slice(&raw);
                    Ok(CapabilityParam::Integer(i64::from_le_bytes(buffer)))
                },
                _ => Err(Error::msg("Unsupported integer length")),
            },
            PARAM_TYPE::FLOAT => {
                match raw.len() {
                    4 => {
                        let mut buffer = [0; 4];
                        buffer.copy_from_slice(&raw);
                        Ok(CapabilityParam::Float(
                            f32::from_le_bytes(buffer) as f64
                        ))
                    },
                    8 => {
                        let mut buffer = [0; 8];
                        buffer.copy_from_slice(&raw);
                        Ok(CapabilityParam::Float(f64::from_le_bytes(buffer)))
                    },
                    _ => Err(Error::msg("Unsupported float length")),
                }
            },
            PARAM_TYPE::UTF8 => match String::from_utf8(raw) {
                Ok(s) => Ok(CapabilityParam::Utf8(s)),
                Err(e) => Err(Error::new(e)),
            },
            PARAM_TYPE::BINARY => Ok(CapabilityParam::Binary(raw)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CapabilityRequest {
    pub c_type: CAPABILITY,
    pub params: HashMap<String, CapabilityParam>,
}

impl CapabilityRequest {
    pub fn new(c_type: CAPABILITY) -> CapabilityRequest {
        return CapabilityRequest {
            c_type,
            params: HashMap::new(),
        };
    }
}
