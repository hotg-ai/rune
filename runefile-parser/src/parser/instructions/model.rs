use crate::parser::{Pair, Rule};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone)]
pub struct ModelInstruction {
    pub model_name: String,
    pub model_file: String,
    pub model_parameters: HashMap<String, String>,
    pub code: String,
    pub dependencies: HashMap<String, String>,
}
impl ModelInstruction {
    pub(crate) fn from_record(record: Pair) -> Self {
        let mut parameters_param = HashMap::new();
        let mut model_name_param = "".to_string();
        let mut model_file_param = "".to_string();
        let mut dependencies_map: HashMap<String, String> = HashMap::new();

        for args in record.into_inner() {
            match args.as_rule() {
                Rule::model_file => model_file_param = args.as_str().to_string(),
                Rule::model_name => model_name_param = args.as_str().to_string(),
                Rule::model_args => {
                    for arg in args.into_inner() {
                        match arg.as_rule() {
                            Rule::model_step => {
                                let mut last_param_name = "".to_string();
                                for part in arg.into_inner() {
                                    match part.as_rule() {
                                        Rule::model_arg_variable => {
                                            last_param_name = part.as_str().to_string();
                                        }
                                        Rule::model_arg_value => {
                                            let last_param_value = part.as_str().to_string();
                                            let last_param_name_cloned = last_param_name.clone();
                                            parameters_param
                                                .insert(last_param_name_cloned, last_param_value);
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        //default input & output is [-1,-1]

        let mut input_dims = String::from("[-1,-1]");
        let mut output_dims = String::from("[-1,-1]");
        if parameters_param.contains_key("input") {
            input_dims = parameters_param.get("input").unwrap().to_string();
            //println!("{:?}",input_dims);
        }
        if parameters_param.contains_key("output") {
            output_dims = parameters_param.get("output").unwrap().to_string();
            //println!("{:?}",output_dims);
        }
        let code_string = format!("ml::Model {{ name: String::from(\"{}\"), input_dims: vec!{}, output_dims: vec!{}, framework: ml::FRAMEWORK::TFLITE }}",model_name_param,input_dims,output_dims);
        //add CARGO dependencies
        dependencies_map.insert(
            "no-std-compat".to_string(),
            "\"0.4.1\"".to_string()
        );
        dependencies_map.insert(
            "wee_alloc".to_string(),
            "\"0.4.5\"".to_string()
        );
        dependencies_map.insert(
            "runic-types".to_string(),
            // "{ path = \"../../runic-types\" }".to_string()
            // "{ git = \"ssh://git@github.com/hotg-ai/runic-types\" }".to_string()
            "{ git = \"ssh://git@github.com/hotg-ai/runic-types\" , branch = \"feature/generics_integration_lang\" }".to_string()
        );
        //generate some code

        Self {
            model_name: model_name_param,
            model_file: model_file_param,
            model_parameters: parameters_param,
            code: code_string,
            dependencies: dependencies_map,
        }
    }
}

impl std::fmt::Debug for ModelInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[Model]         name:{}\tfile:{}\tparams:{:?}",
            self.model_name, self.model_file, self.model_parameters
        )
    }
}
