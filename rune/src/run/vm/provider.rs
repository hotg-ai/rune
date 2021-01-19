
use log;
use std::collections::HashMap;

use runic_types::*;
use crate::run::vm::capability::*;
use std::cell::RefCell;

use tflite::ops::builtin::BuiltinOpResolver;
use tflite::{FlatBufferModel, InterpreterBuilder};

pub struct Model {
    fb: FlatBufferModel,
    inputs: u32,
    outputs: u32
}

pub struct Provider {
    requests: RefCell<Vec<RefCell<CapabilityRequest>>>,
    models: RefCell<Vec<RefCell<Model>>>

}

impl Provider {
    pub fn init() -> Provider{
        return Provider{
            requests: RefCell::new(Vec::new()),
            models: RefCell::new(vec![])
        };
    }

    pub fn add_model(&mut self, model_weights: Vec<u8>, inputs:u32, outputs:u32) -> u32 {
       let idx = self.models.borrow().len() as u32;

       let fb = match FlatBufferModel::build_from_buffer(model_weights) {
           Ok(fb) => fb,
           Err(err) => {
               log::error!("Invalid model provided {:?}", err);
               panic!("Invalid model");
           }
       };

       let model = Model{
           fb,
           inputs,
           outputs
       };

       self.models.borrow_mut().push(RefCell::new(model));
       log::info!("Setting Model<{},{}>({})", inputs, outputs, idx);
       return idx;
    }


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
                log::info!("Setting capability({}) params", request_idx);
            },
            _ => {
                log::warn!("Rune called to set param on capability_request({}) that does not exist", request_idx);
                return;
            }
        };


    }
}
