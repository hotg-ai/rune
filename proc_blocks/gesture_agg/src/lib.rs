#![no_std]
extern crate alloc;

#[cfg(test)]
#[macro_use]
extern crate std;

use runic_types::Transform;


#[derive(Debug, Clone, PartialEq)]
pub struct GestureAgg<const N: usize> {
    labels: [N],
}

impl<const N: usize> GestureAgg<N> {
    pub fn new() -> Self {
            labels: [N],
        }
    }

pub extern "C" fn PredictGesture(tfArray: <[f32; N]>) -> f32 {
    unsafe{
        float prediction_history[kGestureCount][kPredictionHistoryLength] = {};
        int prediction_history_index = 0;
        int prediction_suppression_count = 0;

        for(int j=0; j < kgestureCount; j++){
            prediction_history[i][prediction_history_index] = output[i];
        }

        prediction_history_index++;
        if prediction_history_index >= kPredictionHistoryLength {
            prediction_history_index = 0
        }

        int max_predict_index = -1;
        float max_predict_score = 0.0;

        for (int i = 0; i < kGestureCount; i++) {
            float prediction_sum = 0.0;
            for (int j = 0; j < kPredictionHistoryLength; ++j) {
                prediction_sum += prediction_history[i][j];
            }
            const float prediction_average = prediction_sum / kPredictionHistoryLength;
            if ((max_predict_index == -1) || (prediction_average > max_predict_score)) {
                max_predict_index = i;
                max_predict_score = prediction_average;
            }
        }

        // If there's been a recent prediction, don't trigger a new one too soon.
        if (prediction_suppression_count > 0) {
            --prediction_suppression_count;
        }

        if ((max_predict_index == kNoGesture) ||
            (max_predict_score < kDetectionThreshold) ||
            (prediction_suppression_count > 0)) {
            return kNoGesture;
        } else {
            // Reset the suppression counter so we don't come up with another prediction
            // too soon.
            prediction_suppression_count = kPredictionSuppressionDuration;
            return max_predict_index;
        }
    }
}


impl<const N: usize> Transform<[f32; N]> for GestureAgg<N> {
    type Output = &'static str;

    fn transform(&mut self, input: [f32; N]) -> Self::Output {

        PredictGesture(input: [f32; N]);
        // Some(PredictGesture(input: [f32; N])) => { println!("warning: sound buffer is full"); }
        _ => { }
    }
}

impl<const N: usize> Default for GestureAgg<N> {
    fn default() -> Self { GestureAgg::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use runic_types::Transform;

    #[test]
    fn it_works() {
        let input = [0.0, 1.0, 0.0, 0.0];
        let mut pb = GestureAgg::new()

        let out = pb.transform(input);

        // assert_eq!(out, "Ring");
    }

    // #[test]
    // fn handles_empty_input() {
    //     let input = [];
    //     let mut pb = GestureAgg::new().with_unknown_label(MISSING_LABEL);

    //     let out = pb.transform(input);

    //     assert_eq!(out, MISSING_LABEL);
    // }

    // #[test]
    // fn handles_null_ohv() {
    //     let input = [0.0; 4];
    //     let mut pb = GestureAgg::new()
    //         .with_unknown_label(MISSING_LABEL)
    //         .with_labels(["a", "b", "c", "d"]);

    //     let out = pb.transform(input);

    //     assert_eq!(out, MISSING_LABEL);
    // }
}
