#![no_std]

extern crate alloc;

use core::{convert::TryInto, fmt::Debug};

use alloc::vec::Vec;
use hotg_rune_proc_blocks::{Tensor, Transform, ProcBlock};

/// A proc block which, when given a set of indices, will return their
/// associated labels.
///
/// # Examples
/// ```rust
/// # use label::Label;
/// # use hotg_rune_core::Tensor;
/// # use hotg_rune_proc_blocks::Transform;
/// let mut proc_block = Label::default();
/// proc_block.set_labels(["zero", "one", "two", "three"]);
/// let input = Tensor::new_vector(vec![3, 1, 2]);
///
/// let got = proc_block.transform(input);
///
/// assert_eq!(got.elements(), &["three", "one", "two"]);
/// ```
#[derive(Debug, Default, Clone, PartialEq, ProcBlock)]
pub struct Label {
    labels: Vec<&'static str>,
}

impl<T> Transform<Tensor<T>> for Label
where
    T: Copy + TryInto<usize>,
    <T as TryInto<usize>>::Error: Debug,
{
    type Output = Tensor<&'static str>;

    fn transform(&mut self, input: Tensor<T>) -> Self::Output {
        let view = input
            .view::<1>()
            .expect("This proc block only supports 1D inputs");

        let indices = view.elements().iter().copied().map(|ix| {
            ix.try_into()
                .expect("Unable to convert the index to a usize")
        });

        // Note: We use a more cumbersome match statement instead of unwrap()
        // to provide the user with more useful error messages
        indices
            .map(|ix| match self.labels.get(ix) {
                Some(&label) => label,
                None => panic!("Index out of bounds: there are {} labels but label {} was requested", self.labels.len(), ix)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn only_works_with_1d_inputs() {
        let mut proc_block = Label::default();
        let inputs: Tensor<u32> =
            Tensor::new_row_major(alloc::vec![0; 1*2*3].into(), alloc::vec![1, 2, 3]);

        let _ = proc_block.transform(inputs);
    }

    #[test]
    #[should_panic = "Index out of bounds: there are 2 labels but label 42 was requested"]
    fn label_index_out_of_bounds() {
        let mut proc_block = Label::default();
        proc_block.set_labels(["first", "second"]);
        let input = Tensor::new_vector(alloc::vec![0_usize, 42]);

        let _ = proc_block.transform(input);
    }

    #[test]
    fn get_the_correct_labels() {
        let mut proc_block = Label::default();
        proc_block.set_labels(["zero", "one", "two", "three"]);
        let input = Tensor::new_vector(alloc::vec![3, 1, 2]);
        let should_be = Tensor::new_vector(alloc::vec!["three", "one", "two"]);

        let got = proc_block.transform(input);

        assert_eq!(got, should_be);
    }
}
