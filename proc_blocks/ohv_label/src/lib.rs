#![no_std]
extern crate alloc;

#[cfg(test)]
#[macro_use]
extern crate std;

use alloc::vec::Vec;
use runic_types::{PipelineContext, Transform};

struct OhvLabel {
    labels: Vec<&'static str>,
    unknown_label: &'static str,
}

impl Transform<Vec<u8>> for OhvLabel {
    type Output = &'static str;

    fn transform(
        &mut self,
        input: Vec<u8>,
        _ctx: &mut PipelineContext,
    ) -> Self::Output {
        input
            .iter()
            .position(|&r| r == 1)
            .and_then(|index| self.labels.get(index))
            .copied()
            .unwrap_or(self.unknown_label)
    }
}

#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};
    use runic_types::{PipelineContext, Transform};
    use std::prelude::v1::*;

    use crate::OhvLabel;

    #[test]
    fn it_works() {
        let mut input: [u8; 4] = [0; 4];
        let mut rng = thread_rng();
        let idx = rng.gen_range(0..4) as usize;
        let mut pipeline = PipelineContext {};
        input[idx] = 1;

        let mut pb = OhvLabel {
            labels: vec!["Wing", "Ring", "Slope", "Unknown"],
            unknown_label: "NO_LABEL_FOUND",
        };

        let out = pb.transform(input.to_vec(), &mut pipeline);
        let labels = vec!["Wing", "Ring", "Slope", "Unknown"];
        assert_eq!(out, labels[idx]);
        println!("OhV={:?} | Label={}", input, out);
    }

    #[test]
    fn handles_missing_labels() {
        let mut input: [u8; 4] = [0; 4];
        let mut rng = thread_rng();
        let idx = rng.gen_range(0..4) as usize;
        let mut pipeline = PipelineContext {};
        input[idx] = 1;

        let mut pb = OhvLabel {
            labels: vec![],
            unknown_label: "NO_LABEL_FOUND",
        };

        let out = pb.transform(input.to_vec(), &mut pipeline);

        assert_eq!(out, "NO_LABEL_FOUND");
        println!("OhV={:?} | Label={}", input, out);
    }

    #[test]
    fn handles_null_ohv() {
        let input: [u8; 4] = [0; 4];

        let mut pb = OhvLabel {
            labels: vec!["a", "b", "c", "d"],
            unknown_label: "NO_LABEL_FOUND",
        };
        let mut pipeline = PipelineContext {};

        let out = pb.transform(input.to_vec(), &mut pipeline);

        assert_eq!(out, "NO_LABEL_FOUND");
        println!("OhV={:?} | Label={}", input, out);
    }
}
