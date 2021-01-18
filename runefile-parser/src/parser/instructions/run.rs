use crate::parser::{Pair, Rule};

#[derive(PartialEq, Eq, Clone)]
pub struct RunInstruction {
    pub steps: Vec<String>,
}

impl RunInstruction {
    pub(crate) fn from_record(record: Pair) -> Self {
        let mut steps_vector: Vec<String> = vec![];
        for args in record.into_inner() {
            match args.as_rule() {
                Rule::run_args => {
                    for step_record in args.into_inner() {
                        match step_record.as_rule() {
                            Rule::run_step => steps_vector.push(step_record.as_str().to_string()),
                            _ => {}
                        }
                    }
                }
                _ => {}
            };
        }

        Self {
            steps: steps_vector,
        }
    }
}

impl std::fmt::Debug for RunInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[Run]           steps:{:?}", self.steps)
    }
}
