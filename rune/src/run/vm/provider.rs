
use log;
use std::collections::HashMap;

use runic_types::*;
use crate::run::vm::capability::*;
use std::cell::RefCell;


pub struct Model {
    weights: Vec<u8>
}

pub struct Provider {
    requests: RefCell<Vec<RefCell<CapabilityRequest>>>,
    models: Vec<Model>

}

impl Provider {
    pub fn init() -> Provider{
        return Provider{
            requests: RefCell::new(Vec::new()),
            models: vec![]
        };
    }

    // pub fn request_capability(&mut self, requested: runic_types::CAPABILITY) -> u32 {
    //     let idx = self.requests.len() as u32;
    //     self.requests.push(CapabilityRequest::init(requested));
    //     return idx;
    // }


    pub fn request_capability(&mut self, requested: u32) -> u32 {
        
        let idx = self.requests.borrow().len() as u32;
        let mut cr = CapabilityRequest::init(runic_types::CAPABILITY::from_u32(requested));

        self.requests.borrow_mut().push(RefCell::new(cr));
        log::info!("Setting capability({}) {:?}", idx, runic_types::CAPABILITY::from_u32(requested));
        return idx;
    }



    pub fn set_capability_request_param(&mut self, request_idx: u32, key: String, value: Vec<u8>, value_t: runic_types::PARAM_TYPE) {
        let capability_request = match self.requests.borrow_mut().get(request_idx as usize)  {
            Some(cr) => {
                cr.borrow_mut().set_param(key, value, value_t);
                log::info!("Parameter set for Capability {:?}", cr.borrow());
            },
            _ => {
                log::warn!("Rune called to set param on capability_request({}) that does not exist", request_idx);
                return;
            }
        };


    }
}
