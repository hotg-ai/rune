#![no_std]
extern crate alloc;

#[cfg(test)]
#[macro_use]
extern crate std;

use runic_types::{HasOutputs, Transform};
use alloc::collections::VecDeque;

/// Gesture Aggregator takes a list of confidences and returns the associated
/// label of the most confident gesture so it identifies which gesture is
/// occuring.
#[derive(Debug, Clone, PartialEq)]
pub struct GestureAgg<const N: usize> {
    labels: [&'static str; N],
    history: VecDeque<[f32; N]>,
    max_capacity: usize,
    unknown: &'static str,
    throttle_interval: usize,
    countdown: usize,
}
const MAX_CAPACITY: usize = 1024;
const UNKNOWN_LABEL: &'static str = "<MISSING>";
const DEFAULT_THROTTLE_INTERVAL: usize = 16;

impl<const N: usize> GestureAgg<N> {
    pub fn new() -> Self {
        GestureAgg {
            labels: [""; N],
            history: VecDeque::new(),
            max_capacity: MAX_CAPACITY,
            unknown: UNKNOWN_LABEL,
            throttle_interval: DEFAULT_THROTTLE_INTERVAL,
            countdown: 0,
        }
    }

    pub fn with_labels(self, labels: [&'static str; N]) -> Self {
        GestureAgg { labels, ..self }
    }

    pub fn with_throttle_interval(self, throttle_interval: usize) -> Self {
        GestureAgg {
            throttle_interval,
            ..self
        }
    }

    fn add_history(&mut self, input: [f32; N]) {
        self.history.push_back(input);

        while self.history.len() > self.max_capacity {
            self.history.pop_front();
        }
    }

    fn most_likely_gesture(&self) -> Option<usize> {
        if self.history.is_empty() {
            return None;
        }

        (0..N)
            .fold(None, |previous_most_likely, gesture_index| {
                let sum: f32 =
                    self.history.iter().map(|input| input[gesture_index]).sum();
                let avg = sum / self.history.len() as f32;

                match previous_most_likely {
                    Some((_, previous_avg)) if previous_avg >= avg => {
                        previous_most_likely
                    },
                    _ => Some((gesture_index, avg)),
                }
            })
            .map(|pair| pair.0)
    }

    fn label_for_index(&self, index: Option<usize>) -> Option<&'static str> {
        index.and_then(|ix| self.labels.get(ix)).copied()
    }
}

impl<const N: usize> Transform<[f32; N]> for GestureAgg<N> {
    type Output = &'static str;

    fn transform(&mut self, input: [f32; N]) -> Self::Output {
        // This is a rust port of https://github.com/andriyadi/MagicWand-TFLite-ESP32/blob/00fd15f0861b27437236689ceb642a05cf5fb028/src/gesture_predictor.cpp#L35-L101

        self.add_history(input);
        let gesture_index = self.most_likely_gesture();
        let label = self.label_for_index(gesture_index);
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

impl<const N: usize> Default for GestureAgg<N> {
    fn default() -> Self { GestureAgg::new() }
}

impl<const N: usize> HasOutputs for GestureAgg<N> {}

#[cfg(test)]
mod tests {
    use super::*;
    use runic_types::Transform;

    #[test]
    fn it_works() {
        let input = [0.010419448, 0.9256067, 0.016922968, 0.047050953];
        let mut pb =
            GestureAgg::new().with_labels(["Wing", "Ring", "Slope", "Unknown"]);

        let out = pb.transform(input);

        assert_eq!(out, "Ring");
    }

    #[test]
    fn add_one_in_to_history() {
        let mut ges = GestureAgg::new();
        assert!(ges.history.is_empty());

        ges.add_history([0.010419448, 0.9256067, 0.016922968, 0.047050953]);
        assert_eq!(ges.history.len(), 1);
    }
    #[test]
    fn history_buf_is_less_than_1024() {
        let mut ges = GestureAgg::new();

        for _ in 0..MAX_CAPACITY {
            ges.add_history([0.5])
        }
        assert_eq!(ges.history.len(), MAX_CAPACITY);

        ges.add_history([0.5]);
        assert_eq!(ges.history.len(), MAX_CAPACITY);
    }

    #[test]
    fn empty_history_has_no_most_likely_ges() {
        let ges: GestureAgg<42> = GestureAgg::new();

        let got = ges.most_likely_gesture();

        assert!(got.is_none());
    }

    #[test]
    fn previous_most_likely_ges() {
        let mut ges = GestureAgg::new();
        ges.add_history([0.0, 0.0, 0.0, 1.0, 0.0, 0.0]);

        let got = ges.most_likely_gesture();

        assert_eq!(got, Some(3));
    }

    #[test]
    fn labels_for_valid_index() {
        let ges =
            GestureAgg::new().with_labels(["Wing", "Ring", "Slope", "Unknown"]);

        let got = ges.label_for_index(Some(2));

        assert_eq!(got, Some("Slope"));
    }
    #[test]
    fn labels_for_out_of_bounds_index() {
        let ges =
            GestureAgg::new().with_labels(["Wing", "Ring", "Slope", "Unknown"]);

        let got = ges.label_for_index(Some(5));

        assert_eq!(got, None);
    }

    #[test]
    fn labels_for_no_index() {
        let ges =
            GestureAgg::new().with_labels(["Wing", "Ring", "Slope", "Unknown"]);

        let got = ges.label_for_index(None);

        assert_eq!(got, None);
    }

    #[test]
    fn throttling() {
        let mut ges = GestureAgg::new()
            .with_labels(["Wing", "Ring", "Slope", "Unknown"])
            .with_throttle_interval(3);

        let got = ges.transform([0.0, 1.0, 0.0, 0.0]);
        assert_eq!(got, "Ring");

        let got = ges.transform([0.0, 1.0, 0.0, 0.0]);
        assert_eq!(got, ges.unknown);

        let got = ges.transform([0.0, 1.0, 0.0, 0.0]);
        assert_eq!(got, ges.unknown);

        let got = ges.transform([0.0, 1.0, 0.0, 0.0]);
        assert_eq!(got, "Ring");
    }
}
