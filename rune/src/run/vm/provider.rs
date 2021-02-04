use log;

use crate::run::vm::capability::*;
use runic_types::*;
use std::cell::RefCell;

use runic_transform::{Transform, Transformable};

use tflite::ops::builtin::BuiltinOpResolver;
use tflite::{FlatBufferModel, InterpreterBuilder};

#[derive(Clone)]
pub struct Model {
    model_weights: RefCell<&'static [u8]>,
    inputs: u32,
    outputs: u32,
}

impl Model {
    pub fn predict<T, F>(mut self, input: Vec<T>) -> Vec<F> {
        return vec![];

        //    let fb = match FlatBufferModel::build_from_buffer(self.model_weights) {
        //     Ok(fb) => {
        //         log::info!("Successfully Loaded model as FlatbufferModel");
        //         fb
        //     },
        //     Err(err) => {
        //         log::error!("Invalid model provided {:?}", err);
        //         panic!("Invalid model");
        //     }
        //};

        // let resolver = BuiltinOpResolver::default();

        // let builder = InterpreterBuilder::new(fb, resolver).unwrap();
        // let interpreter: tflite::Interpreter<tflite::ops::builtin::BuiltinOpResolver>  = builder.build().unwrap();

        //    let inputs = interpreter.inputs().to_vec();

        //    let outputs = interpreter.outputs().to_vec();

        //    return vec![];
        // }
    }
}

pub struct Provider {
    requests: RefCell<Vec<RefCell<CapabilityRequest>>>,
    model: RefCell<Vec<u8>>,
}

impl Provider {
    pub fn init() -> Provider {
        return Provider {
            requests: RefCell::new(Vec::new()),
            model: RefCell::new(vec![]),
        };
    }

    pub fn predict_model<T>(&mut self, idx: u32, input: Vec<u8>, value_t: PARAM_TYPE) -> Vec<T> {
        let fb = match FlatBufferModel::build_from_buffer(self.model.borrow_mut().to_vec()) {
            Ok(fb) => {
                log::info!("Successfully Loaded model as FlatbufferModel");
                fb
            }
            Err(err) => {
                log::error!("Invalid model provided {:?}", err);
                panic!("Invalid model");
            }
        };

        let resolver = BuiltinOpResolver::default();

        let builder = InterpreterBuilder::new(fb, resolver).unwrap();
        let mut interpreter: tflite::Interpreter<tflite::ops::builtin::BuiltinOpResolver> =
            builder.build().unwrap();
        
        let input = Transform::<f32, f32>::from_buffer(&input).unwrap();
        log::info!("{:?}", interpreter.inputs().to_vec());
        log::info!("INPUT<{:?}>", input);
        interpreter.allocate_tensors().unwrap();

        let inputs = interpreter.inputs().to_vec();

        let outputs = interpreter.outputs().to_vec();
        let input_index = inputs[0];

        let input_tensor = interpreter.tensor_info(input_index).unwrap();
        log::info!("DIMS = {:?}", input_tensor.dims);
        let output_index = outputs[0];
        let output_tensor = interpreter.tensor_info(output_index).unwrap();
        log::info!("Model loaded with input tensor: {:?}", input_tensor);
        log::info!("Model loaded with output tensor: {:?}", output_tensor);
        let input_tensors: &mut [f32] = interpreter.tensor_data_mut(input_index).unwrap();

        input_tensors[0] = input[0];
    
        interpreter.invoke().unwrap();
    
        let output: &[f32] = interpreter.tensor_data(output_index).unwrap();
    
        log::info!("Output: {:?}", output);
        return vec![];
    }

    pub fn add_model(&mut self, model_weights: Vec<u8>, inputs: u32, outputs: u32) -> u32 {
        let idx = 0;

        //let mut model_weights = &model_weights.clone()[..];
        for i in model_weights.iter() {
            self.model.borrow_mut().push(*i);
        }

        log::info!("Setting Model<{},{}>({})", inputs, outputs, idx);
        return idx;
    }

    pub fn request_capability(&mut self, requested: u32) -> u32 {
        let idx = self.requests.borrow().len() as u32;
        let mut cr = CapabilityRequest::init(runic_types::CAPABILITY::from_u32(requested));

        self.requests.borrow_mut().push(RefCell::new(cr));
        log::info!(
            "Setting capability({}) {:?}",
            idx,
            runic_types::CAPABILITY::from_u32(requested)
        );
        return idx;
    }

    pub fn set_capability_request_param(
        &mut self,
        request_idx: u32,
        key: String,
        value: Vec<u8>,
        value_t: runic_types::PARAM_TYPE,
    ) {
        let capability_request = match self.requests.borrow_mut().get(request_idx as usize) {
            Some(cr) => {
                cr.borrow_mut().set_param(key, value, value_t);
                log::info!("Setting params for capability({})", request_idx);
            }
            _ => {
                log::warn!(
                    "Rune called to set param on capability_request({}) that does not exist",
                    request_idx
                );
                return;
            }
        };
    }
}
