#![no_std]
use runic_types::Transform;

struct Normalize {}

impl<const N: usize> Transform<[f32; N]> for Normalize {
    type Output = [f32; N];

    fn transform(&mut self, mut input: [f32; N]) -> Self::Output {
        let min_value = input.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let max_value = input.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let denom = max_value - min_value;
        if denom == 0.0 {
            return input;
        }

        for element in &mut input {
            *element = (*element - min_value) / denom;
        }

        return input;
    }
}

#[cfg(test)]
mod tests {
    use crate::Normalize;
    use rand::{thread_rng, Rng};
    use runic_types::Transform;

    #[test]
    fn it_works() {
        let mut input: [f32; 348] = [0.0; 348];
        thread_rng().fill(&mut input[..]);
        let mut norm_pb: Normalize = Normalize {};
        let output = norm_pb.transform(input);
        let min_value = output.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let max_value = output.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        assert_eq!(min_value, 0.0);
        assert_eq!(max_value, 1.0);
        assert_eq!(output.len(), 348);
    }

    #[test]
    fn handle_empty() {
        let input: [f32; 384] = [0.0; 384];

        let mut norm_pb: Normalize = Normalize {};
        let output = norm_pb.transform(input);

        assert_eq!(output, input);
        assert_eq!(output.len(), 384);
    }
}
