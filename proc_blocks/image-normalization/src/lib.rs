#![no_std]

use runic_types::{HasOutputs, Tensor, Transform};

#[derive(Debug, Clone, PartialEq)]
pub struct ImageNormalization {
    mean: f32,
    standard_deviation: f32,
}

impl ImageNormalization {
    pub const fn with_mean(self, mean: f32) -> Self {
        ImageNormalization { mean, ..self }
    }

    pub const fn with_std_dev(self, std_dev: f32) -> Self {
        self.with_standard_deviation(std_dev)
    }

    pub const fn with_standard_deviation(
        self,
        standard_deviation: f32,
    ) -> Self {
        ImageNormalization {
            standard_deviation,
            ..self
        }
    }
}

impl Transform<Tensor<u8>> for ImageNormalization {
    type Output = Tensor<f32>;

    fn transform(&mut self, input: Tensor<u8>) -> Self::Output {
        self.transform(input.map(|_dims, &elem| elem as f32))
    }
}

impl Transform<Tensor<f32>> for ImageNormalization {
    type Output = Tensor<f32>;

    fn transform(&mut self, mut input: Tensor<f32>) -> Self::Output {
        let mut view = input.view_mut::<3>()
            .expect("The image normalization proc block only supports outputs of the form [channels, rows, columns]");

        let [channels, rows, columns] = view.dimensions();

        for y in 0..rows {
            for x in 0..columns {
                for channel in 0..channels {
                    let ix = [channel, x, y];
                    let current_value = view[ix];
                    view[ix] =
                        (current_value - self.mean) / self.standard_deviation;
                }
            }
        }

        input
    }
}

impl HasOutputs for ImageNormalization {
    fn set_output_dimensions(&mut self, dimensions: &[usize]) {
        match *dimensions {
            [1, _, _] | [3, _, _] => {},
            [channels, _, _] => panic!(
                "The number of channels should be either 1 or 3, found {}",
                channels
            ),
            _ => panic!("The image normalization proc block only supports outputs of the form [channels, rows, columns], found {:?}", dimensions),
        }
    }
}

impl Default for ImageNormalization {
    fn default() -> Self {
        ImageNormalization {
            mean: 0.0,
            standard_deviation: 1.0,
        }
    }
}
