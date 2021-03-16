#![no_std]
extern crate alloc;

#[cfg(test)]
#[macro_use]
extern crate std;

use runic_types::Transform;
use alloc::collections::VecDeque;

/// Debounce Confidence takes a list of confidences and returns the associated
/// label of the most confident value so it identifies which value is occuring.

#[derive(Debug, Clone, PartialEq)]
pub struct DebounceConfidence<T: Sized, const N: usize> {
    labels: [&'static str; N],
    history: VecDeque<[T; N]>,
    max_capacity: usize,
    unknown: &'static str,
    throttle_interval: usize,
    countdown: usize,
    // type: usize,
}
const MAX_CAPACITY: usize = 1024;
const UNKNOWN_LABEL: &'static str = "<MISSING>";
const DEFAULT_THROTTLE_INTERVAL: usize = 16;

impl<T: Sized, const N: usize> DebounceConfidence<T, N> {
    pub fn new() -> Self {
        DebounceConfidence {
            labels: [""; N],
            history: VecDeque::new(),
            max_capacity: MAX_CAPACITY,
            unknown: UNKNOWN_LABEL,
            throttle_interval: DEFAULT_THROTTLE_INTERVAL,
            countdown: 0,
        }
    }

    pub fn with_labels(self, labels: [&'static str; N]) -> Self {
        DebounceConfidence { labels, ..self }
    }

    pub fn with_throttle_interval(self, throttle_interval: usize) -> Self {
        DebounceConfidence {
            throttle_interval,
            ..self
        }
    }

    fn add_history(&mut self, input: [T; N]) {
        self.history.push_back(input);

        while self.history.len() > self.max_capacity {
            self.history.pop_front();
        }
    }

    fn most_likely_debounce_confidence(&self) -> Option<usize> {
        if self.history.is_empty() {
            return None;
        }

        (0..N)
            .fold(None, |previous_most_likely, debounce_confidence_index| {
                let sum: T = self
                // let sum: u8 = self
                    .history
                    .iter()
                    .map(|input| input[debounce_confidence_index])
                    .sum();
                let avg = sum / self.history.len() as T;
                // let avg = sum / self.history.len();

                match previous_most_likely {
                    Some((_, previous_avg)) if previous_avg >= avg => {
                        previous_most_likely
                    },
                    _ => Some((debounce_confidence_index, avg)),
                }
            })
            .map(|pair| pair.0)
    }

    fn label_for_index(&self, index: Option<usize>) -> Option<&'static str> {
        index.and_then(|ix| self.labels.get(ix)).copied()
    }
}

impl<T: Sized, const N: usize> Transform<[T; N]>
    for DebounceConfidence<T, N>
{
    type Output = &'static str;

    fn transform(&mut self, input: [T; N]) -> Self::Output {
        self.add_history(input);
        let debounce_confidence_index = self.most_likely_debounce_confidence();
        let label = self.label_for_index(debounce_confidence_index);
        self.countdown = self.countdown.saturating_sub(1);

        match label {
            Some(label) if self.countdown == 0 => {
                self.countdown = self.throttle_interval;
                label
            },
            _ => self.unknown,
        }
    }
}

impl<T: Sized, const N: usize> Default for DebounceConfidence<T, N> {
    fn default() -> Self { DebounceConfidence::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use runic_types::Transform;

    #[test]
    fn it_works() {
        let input = [0, 1, 0, 0];
        let mut pb = DebounceConfidence::new()
            .with_labels(["Wing", "Ring", "Slope", "Unknown"]);

        let out = pb.transform(input);

        assert_eq!(out, "Ring");
    }

    #[test]
    fn add_one_in_to_history() {
        let mut ges = DebounceConfidence::new();
        assert!(ges.history.is_empty());

        ges.add_history([0, 1, 0, 0]);
        assert_eq!(ges.history.len(), 1);
    }
    #[test]
    fn history_buf_is_less_than_1024() {
        let mut ges = DebounceConfidence::new();

        for _ in 0..MAX_CAPACITY {
            ges.add_history([2])
        }
        assert_eq!(ges.history.len(), MAX_CAPACITY);

        ges.add_history([2]);
        assert_eq!(ges.history.len(), MAX_CAPACITY);
    }

    #[test]
    fn empty_history_has_no_most_likely_ges() {
        let ges: DebounceConfidence<u8, 42> = DebounceConfidence::new();

        let got = ges.most_likely_debounce_confidence();

        assert!(got.is_none());
    }

    #[test]
    fn previous_most_likely_ges() {
        let ges: DebounceConfidence<u8, 6> = DebounceConfidence::new();
        ges.add_history([0, 0, 0, 1, 0, 0]);

        let got = ges.most_likely_debounce_confidence();

        assert_eq!(got, Some(3));
    }

    #[test]
    fn labels_for_valid_index() {
        let ges: DebounceConfidence<u8, 4> = DebounceConfidence::new()
        // let ges = DebounceConfidence::new()
            .with_labels(["Wing", "Ring", "Slope", "Unknown"]);

        let got = ges.label_for_index(Some(2));

        assert_eq!(got, Some("Slope"));
    }
    #[test]
    fn labels_for_out_of_bounds_index() {
        // let ges = DebounceConfidence::new()
        let ges: DebounceConfidence<u8, 4> = DebounceConfidence::new()
            .with_labels(["Wing", "Ring", "Slope", "Unknown"]);

        let got = ges.label_for_index(Some(5));

        assert_eq!(got, None);
    }

    #[test]
    fn labels_for_no_index() {
        // let ges = DebounceConfidence::new()
        let ges: DebounceConfidence<u8, 4> = DebounceConfidence::new()
            .with_labels(["Wing", "Ring", "Slope", "Unknown"]);

        let got = ges.label_for_index(None);

        assert_eq!(got, None);
    }

    #[test]
    fn throttling() {
        // let ges: DebounceConfidence<u8, 4> = DebounceConfidence::new()
        let mut ges = DebounceConfidence::new()
            .with_labels(["Wing", "Ring", "Slope", "Unknown"])
            .with_throttle_interval(3);

        let got = ges.transform([0, 1, 0, 0]);
        assert_eq!(got, "Ring");

        let got = ges.transform([0, 1, 0, 0]);
        assert_eq!(got, ges.unknown);

        let got = ges.transform([0, 1, 0, 0]);
        assert_eq!(got, ges.unknown);

        let got = ges.transform([0, 1, 0, 0]);
        assert_eq!(got, "Ring");
    }
}
