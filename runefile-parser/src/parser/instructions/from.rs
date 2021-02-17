use crate::parser::{Pair, Rule};

#[derive(PartialEq, Eq, Clone)]
pub struct FromInstruction {
    pub image: String,
}

impl FromInstruction {
    pub(crate) fn from_record(record: Pair) -> Self {
        let mut image_var = "None".to_string();
        for field in record.into_inner() {
            match field.as_rule() {
                Rule::from_image => {
                    image_var = field.as_str().to_string();
                },
                _ => {},
            };
        }
        Self { image: image_var }
    }
}

impl std::fmt::Debug for FromInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[From]          image:{}", self.image)
    }
}
