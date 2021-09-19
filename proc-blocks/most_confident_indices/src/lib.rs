#![no_std]

extern crate alloc;

use core::{convert::TryInto, fmt::Debug};

use alloc::vec::Vec;
use hotg_rune_core::Tensor;
use hotg_rune_proc_blocks::{ProcBlock, Transform};

/// A proc block which, when given a list of confidences, will return the
/// indices of the top N most confident values.
///
/// Will return a 1-element [`Tensor`] by default.
#[derive(Debug, Clone, PartialEq, ProcBlock)]
pub struct MostConfidentIndices {
    /// The number of indices to return.
    count: usize,
}

impl MostConfidentIndices {
    pub fn new(count: usize) -> Self { MostConfidentIndices { count } }
}

impl Default for MostConfidentIndices {
    fn default() -> Self { MostConfidentIndices::new(1) }
}

impl<T: PartialOrd + Copy> Transform<Tensor<T>> for MostConfidentIndices {
    type Output = Tensor<u32>;

    fn transform(&mut self, input: Tensor<T>) -> Self::Output {
        let elements = input.elements();
        assert!(
            self.count <= elements.len(),
            "Unable to take the top {} values from a {}-item input",
            self.count,
            elements.len()
        );

        let mut indices_and_confidence: Vec<_> =
            elements.iter().copied().enumerate().collect();

        indices_and_confidence.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        indices_and_confidence
            .into_iter()
            .map(|(index, _confidence)| index.try_into().unwrap())
            .take(self.count)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn only_works_with_1d() {
        let mut proc_block = MostConfidentIndices::default();
        let input: Tensor<u32> =
            Tensor::new_row_major(alloc::vec![0; 6].into(), alloc::vec![1, 2, 3]);

        let _ = proc_block.transform(input);
    }

    #[test]
    #[should_panic]
    fn count_must_be_less_than_input_size() {
        let mut proc_block = MostConfidentIndices::new(42);
        let input = Tensor::new_vector(alloc::vec![0, 0, 1, 2]);

        let _ = proc_block.transform(input);
    }

    #[test]
    fn get_top_3_values() {
        let mut proc_block = MostConfidentIndices::new(3);
        let input =
            Tensor::new_vector(alloc::vec![0.0, 0.5, 10.0, 3.5, -200.0]);
        let should_be = Tensor::new_vector(alloc::vec![2, 3, 1]);

        let got = proc_block.transform(input);

        assert_eq!(got, should_be);
    }
}
