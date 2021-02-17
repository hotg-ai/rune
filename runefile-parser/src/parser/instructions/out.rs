use crate::parser::{Pair, Rule};

#[derive(PartialEq, Eq, Clone)]
pub struct OutInstruction {
    pub out_type: String,
}

impl OutInstruction {
    pub(crate) fn from_record(record: Pair) -> Self {
        let mut type_var = "None".to_string();
        for field in record.into_inner() {
            match field.as_rule() {
                Rule::out_type => {
                    type_var = field.as_str().to_string();
                },
                _ => {},
            };
        }
        Self { out_type: type_var }
    }
}

impl std::fmt::Debug for OutInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[Out]           type:{}", self.out_type)
    }
}
