use crate::parser::{Pair, Rule};
use codegen::Scope;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone)]
pub struct CapabilityInstruction {
    pub capability_name: String,
    pub capability_description: String,
    pub capability_parameters: HashMap<String, String>,
    pub code: String,
    pub dependencies: HashMap<String, String>,
    pub input_type: String,
    pub output_type: String
}

impl CapabilityInstruction {
    pub(crate) fn from_record(record: Pair) -> Self {
        let mut capability_parameters_param: HashMap<String, String> = HashMap::new();
        let mut capability_name_param = "".to_string();
        let mut capability_description_param = "".to_string();
        let mut input_type  = "".to_string();
        let mut output_type = "".to_string();
        let dependencies_map: HashMap<String, String> = HashMap::new();

        for args in record.into_inner() {
            match args.as_rule() {
                Rule::INPUT_TYPES => {
                    for arg in args.into_inner() {
                        match arg.as_rule() {
                            Rule::input_type => {
                                input_type   = arg.as_str().to_string();
                            }
                            Rule::output_type => {
                                output_type = arg.as_str().to_string();
                            }
                            _ => {
                                log::info!("{:#?}", arg.as_str().to_string());
                            }
                        }
                       
                    }
                    
                },
                Rule::capability_name => capability_name_param = args.as_str().to_string(),
                Rule::capability_description => {
                    capability_description_param = args.as_str().to_string()
                }
                Rule::capability_args => {
                    for arg in args.into_inner() {
                        match arg.as_rule() {
                            
                            Rule::capability_step => {
                                let mut last_param_name = "".to_string();
                                for part in arg.into_inner() {
                                    match part.as_rule() {
                                        Rule::capability_arg_variable => {
                                            last_param_name = part.as_str().to_string();
                                        }
                                        Rule::capability_arg_value => {
                                            let last_param_value = part.as_str().to_string();
                                            let last_param_name_cloned = last_param_name.clone();
                                            capability_parameters_param
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
        let mut scope = Scope::new();

        for parameter in capability_parameters_param.keys() {
            match capability_parameters_param.get(parameter) {
                Some(value) => scope.raw(&format!(
                    "params.insert(String::from(\"{}\"), String::from(\"{}\"));",
                    parameter.to_string(),
                    value.to_string()
                )),
                None => scope.raw(""),
            };
        }
        scope.raw(
            &format!(
                "let {}_capability_request = CapabilityRequest {{",
                capability_name_param.to_lowercase()
            )
            .to_string(),
        );
        scope.raw(&format!(
            "    capability: CAPABILITY::{},",
            capability_name_param.to_string()
        ));
        scope.raw("    params,");
        scope.raw("};");

        Self {
            capability_name: capability_name_param,
            capability_description: capability_description_param,
            capability_parameters: capability_parameters_param,
            code: scope.to_string(),
            dependencies: dependencies_map,
            input_type,
            output_type
        }
    }
}

impl std::fmt::Debug for CapabilityInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[Capability]<{},{}>    name:{}\tdescription:{}\tparams:{:?}",
            self.input_type, self.output_type, self.capability_name, self.capability_description, self.capability_parameters, 
        )
    }
}
