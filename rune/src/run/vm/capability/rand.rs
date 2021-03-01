use crate::run::vm::capability::*;

pub struct RandCapability {}

impl Capability for RandCapability {
    fn get_type() -> CAPABILITY { return CAPABILITY::RAND; }

    fn request(
        params: std::collections::HashMap<String, CapabilityParam>,
    ) -> Vec<u8> {
        let _number_of_samples: u32 = match params.get(&String::from("n")) {
            Some(_number_of_samples) => {
                // let int_value = transform::<u32>(
                // (*number_of_samples.value).to_vec(),
                // number_of_samples.value_type); if int_value.
                // len() > 0usize {     int_value[0]
                // } else {
                //     1
                // }
                1
            },
            _ => 1,
        };

        return vec![];
    }
}
