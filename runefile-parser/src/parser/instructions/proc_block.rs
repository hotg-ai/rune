use crate::parser::{Pair, Rule};
use codegen::Scope;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone)]
pub struct ProcBlockInstruction {
    pub path: String,
    pub name: String,
    pub params: HashMap<String, String>,
    pub code: String,
    pub dependencies: HashMap<String, String>,
}
impl ProcBlockInstruction {
    pub(crate) fn from_record(record: Pair) -> Self {
        let mut parameters_param = HashMap::new();
        let mut path_string = "None".to_string();
        let mut name_string = "None".to_string();
        let mut dependencies_map: HashMap<String, String> = HashMap::new();
        for step_record in record.into_inner() {
            match step_record.as_rule() {
                Rule::proc_path => {
                    path_string = step_record.as_str().to_string()
                },
                Rule::proc_name => {
                    name_string = step_record.as_str().to_string()
                },
                Rule::proc_args => {
                    for arg in step_record.into_inner() {
                        match arg.as_rule() {
                            Rule::proc_step => {
                                let mut last_param_name = "".to_string();
                                for part in arg.into_inner() {
                                    match part.as_rule() {
                                        Rule::proc_arg_variable => {
                                            last_param_name =
                                                part.as_str().to_string();
                                        },
                                        Rule::proc_arg_value => {
                                            let last_param_value =
                                                part.as_str().to_string();
                                            let last_param_name_cloned =
                                                last_param_name.clone();
                                            parameters_param.insert(
                                                last_param_name_cloned,
                                                last_param_value,
                                            );
                                        },
                                        _ => {},
                                    }
                                }
                            },
                            _ => {},
                        }
                    }
                },
                _ => {},
            }
        }

        let mut scope = Scope::new();
        if path_string == "runicos/proc-block/fft" {
            // add CARGO dependencies
            dependencies_map.insert(
                "runic-pb-fft".to_string(),
                "{ git = \"ssh://git@github.com/hotg-ai/runic-pb-fft\" }"
                    .to_string(),
            );
            dependencies_map.insert(
                "runic-types".to_string(),
                "{ git = \"ssh://git@github.com/hotg-ai/runic-types\" }"
                    .to_string(),
            );
            // generate some code
            scope.import("std::collections", "HashMap");
            scope.import("runic_pb_fft", "Processor");
            scope.import("runic_types::proc_block", "ProcBlock");
            scope
                .new_fn(&name_string)
                .allow("Debug")
                .vis("pub")
                .ret("Vec<u8>")
                .arg("waveform", "Vec<i16>")
                .line("let mut map = HashMap::new();")
                .line("map.insert(String::from(\"hz\"), String::from(\"16000\"));")
                .line("let fft = Processor{};")
                .line("return fft.process(waveform, map);");
        }

        Self {
            path: path_string,
            name: name_string,
            params: parameters_param,
            code: scope.to_string(),
            dependencies: dependencies_map,
        }
    }
}

impl std::fmt::Debug for ProcBlockInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[ProcBlock]     path:{}\tname:{}\tparams:{:?}",
            self.path, self.name, self.params
        )
    }
}
