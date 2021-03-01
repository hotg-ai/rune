#![no_std]
extern crate alloc;

#[cfg(test)]
#[macro_use]
extern crate std;

use alloc::vec::Vec;
use runic_types::Transform;

#[derive(Debug, Clone, PartialEq)]
pub struct OhvLabel {
    labels: Vec<&'static str>,
    unknown_label: &'static str,
}

impl OhvLabel {
    pub const MISSING_LABEL: &'static str = "<MISSING>";

    pub fn new() -> Self {
        OhvLabel {
            unknown_label: OhvLabel::MISSING_LABEL,
            labels: Vec::new(),
        }
    }

    pub fn with_unknown_label(self, unknown_label: &'static str) -> Self {
        OhvLabel {
            unknown_label,
            ..self
        }
    }

    pub fn with_labels<I>(self, labels: I) -> Self
    where
        I: IntoIterator<Item = &'static str>,
    {
        OhvLabel {
            labels: labels.into_iter().collect(),
            ..self
        }
    }
}

impl<S: AsRef<[u8]>> Transform<S> for OhvLabel {
    type Output = &'static str;

    fn transform(&mut self, input: S) -> Self::Output {
        input
            .as_ref()
            .iter()
            .position(|&r| r == 1)
            .and_then(|index| self.labels.get(index))
            .copied()
            .unwrap_or(self.unknown_label)
    }
}

impl Default for OhvLabel {
    fn default() -> Self { OhvLabel::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use runic_types::Transform;

    #[test]
    fn it_works() {
        let input = [0, 1, 0, 0];
        let mut pb = OhvLabel::new()
            .with_unknown_label("NO_LABEL_FOUND")
            .with_labels(vec!["Wing", "Ring", "Slope", "Unknown"]);

        let out = pb.transform(input.to_vec());

        assert_eq!(out, "Ring");
    }

    #[test]
    fn handles_missing_labels() {
        let input = [0, 1];
        let mut pb = OhvLabel::new().with_unknown_label("NO_LABEL_FOUND");

        let out = pb.transform(input.to_vec());

        assert_eq!(out, "NO_LABEL_FOUND");
    }

    #[test]
    fn handles_null_ohv() {
        let input = [0; 4];
        let mut pb = OhvLabel::new()
            .with_unknown_label("NO_LABEL_FOUND")
            .with_labels(vec!["a", "b", "c", "d"]);

        let out = pb.transform(input);

        assert_eq!(out, "NO_LABEL_FOUND");
    }
}
