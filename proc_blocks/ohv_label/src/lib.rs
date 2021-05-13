#![no_std]
extern crate alloc;

#[cfg(test)]
#[macro_use]
extern crate std;

use core::cmp::Ordering;

use runic_types::{HasOutputs, Tensor, Transform};

pub const MISSING_LABEL: &'static str = "<MISSING>";

#[derive(Debug, Clone, PartialEq)]
pub struct OhvLabel<const N: usize> {
    labels: [&'static str; N],
    unknown_label: &'static str,
}

impl<const N: usize> OhvLabel<N> {
    pub fn new() -> Self {
        OhvLabel {
            labels: [MISSING_LABEL; N],
            unknown_label: MISSING_LABEL,
        }
    }

    pub fn with_unknown_label(self, unknown_label: &'static str) -> Self {
        let OhvLabel {
            mut labels,
            unknown_label: old_unknown_label,
        } = self;

        // Make sure any existing "missing" labels are updated.
        for label in &mut labels {
            if *label == old_unknown_label {
                *label = unknown_label;
            }
        }

        OhvLabel {
            labels,
            unknown_label,
        }
    }

    pub fn with_labels(self, labels: [&'static str; N]) -> Self {
        OhvLabel { labels, ..self }
    }
}

impl<const N: usize> Transform<[f32; N]> for OhvLabel<N> {
    type Output = &'static str;

    fn transform(&mut self, input: [f32; N]) -> Self::Output {
        match self.labels.iter().zip(input.iter().copied()).max_by(
            |left, right| {
                left.1.partial_cmp(&right.1).unwrap_or(Ordering::Equal)
            },
        ) {
            Some((label, probability)) if probability > 0.0 => *label,
            _ => MISSING_LABEL,
        }
    }
}

impl<const N: usize> Transform<[u8; N]> for OhvLabel<N> {
    type Output = &'static str;

    fn transform(&mut self, input: [u8; N]) -> Self::Output {
        match self
            .labels
            .iter()
            .zip(input.iter().copied())
            .max_by(|left, right| left.1.cmp(&right.1))
        {
            Some((label, probability)) if probability > 0u8 => *label,
            _ => MISSING_LABEL,
        }
    }
}

impl<const N: usize> Transform<[i8; N]> for OhvLabel<N> {
    type Output = &'static str;

    fn transform(&mut self, input: [i8; N]) -> Self::Output {
        match self
            .labels
            .iter()
            .zip(input.iter().copied())
            .max_by(|left, right| left.1.cmp(&right.1))
        {
            Some((label, probability)) if probability > -128i8 => *label,
            _ => MISSING_LABEL,
        }
    }
}

impl<const N: usize> Transform<Tensor<f32>> for OhvLabel<N> {
    type Output = &'static str;

    fn transform(&mut self, input: Tensor<f32>) -> Self::Output {
        let input = input.elements();

        match self.labels.iter().zip(input.iter().copied()).max_by(
            |left, right| {
                left.1.partial_cmp(&right.1).unwrap_or(Ordering::Equal)
            },
        ) {
            Some((label, probability)) if probability > 0.0 => *label,
            _ => MISSING_LABEL,
        }
    }
}

impl<const N: usize> Transform<Tensor<u8>> for OhvLabel<N> {
    type Output = &'static str;

    fn transform(&mut self, input: Tensor<u8>) -> Self::Output {
        let input = input.elements();

        match self
            .labels
            .iter()
            .zip(input.iter().copied())
            .max_by(|left, right| left.1.cmp(&right.1))
        {
            Some((label, probability)) if probability > 0u8 => *label,
            _ => MISSING_LABEL,
        }
    }
}

impl<const N: usize> Transform<Tensor<i8>> for OhvLabel<N> {
    type Output = &'static str;

    fn transform(&mut self, input: Tensor<i8>) -> Self::Output {
        let input = input.elements();

        match self
            .labels
            .iter()
            .zip(input.iter().copied())
            .max_by(|left, right| left.1.cmp(&right.1))
        {
            Some((label, probability)) if probability > -128i8 => *label,
            _ => MISSING_LABEL,
        }
    }
}

impl<const N: usize> Default for OhvLabel<N> {
    fn default() -> Self { OhvLabel::new() }
}

impl<const N: usize> HasOutputs for OhvLabel<N> {}

#[cfg(test)]
mod tests {
    use super::*;
    use runic_types::Transform;

    #[test]
    fn it_works() {
        let input = [0.0, 1.0, 0.0, 0.0];
        let mut pb = OhvLabel::new()
            .with_unknown_label("NO_LABEL_FOUND")
            .with_labels(["Wing", "Ring", "Slope", "Unknown"]);

        let out = pb.transform(input);

        assert_eq!(out, "Ring");
    }

    #[test]
    fn handles_empty_input() {
        let input: [f32; 0] = [];
        let mut pb = OhvLabel::new()
            .with_labels([])
            .with_unknown_label(MISSING_LABEL);

        let out = pb.transform(input);

        assert_eq!(out, MISSING_LABEL);
    }

    #[test]
    fn handles_null_ohv() {
        let input = [0.0; 4];
        let mut pb = OhvLabel::new()
            .with_unknown_label(MISSING_LABEL)
            .with_labels(["a", "b", "c", "d"]);

        let out = pb.transform(input);

        assert_eq!(out, MISSING_LABEL);
    }

    #[test]
    fn handle_non_finite_values() {
        let input = [
            std::f32::NAN,
            -0.0,
            std::f32::INFINITY,
            std::f32::NEG_INFINITY,
        ];
        let mut pb = OhvLabel::new()
            .with_unknown_label(MISSING_LABEL)
            .with_labels(["a", "b", "c", "d"]);

        let out = pb.transform(input);

        assert_eq!(out, "c");
    }
}
