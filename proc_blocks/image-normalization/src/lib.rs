#![no_std]

mod distribution;

pub use crate::distribution::{Distribution, DistributionConversionError};

use core::{convert::TryInto, fmt::Display};
use runic_types::{HasOutputs, Tensor, Transform, TensorViewMut};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ImageNormalization {
    red: Distribution,
    green: Distribution,
    blue: Distribution,
}

impl ImageNormalization {
    pub fn with_red<D>(self, distribution: D) -> Self
    where
        D: TryInto<Distribution>,
        D::Error: Display,
    {
        match distribution.try_into() {
            Ok(d) => ImageNormalization { red: d, ..self },
            Err(e) => panic!("Invalid distribution: {}", e),
        }
    }

    pub fn with_green<D>(self, distribution: D) -> Self
    where
        D: TryInto<Distribution>,
        D::Error: Display,
    {
        match distribution.try_into() {
            Ok(d) => ImageNormalization { green: d, ..self },
            Err(e) => panic!("Invalid distribution: {}", e),
        }
    }

    pub fn with_blue<D>(self, distribution: D) -> Self
    where
        D: TryInto<Distribution>,
        D::Error: Display,
    {
        match distribution.try_into() {
            Ok(d) => ImageNormalization { blue: d, ..self },
            Err(e) => panic!("Invalid distribution: {}", e),
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

        let [channels, _, _] = view.dimensions();

        assert_eq!(
            channels, 3,
            "The image must have 3 channels - red, green, and blue"
        );

        transform(self.red, 0, &mut view);
        transform(self.green, 1, &mut view);
        transform(self.blue, 2, &mut view);

        input
    }
}

fn transform(
    d: Distribution,
    channel: usize,
    view: &mut TensorViewMut<'_, f32, 3>,
) {
    let [_, rows, columns] = view.dimensions();

    for y in 0..rows {
        for x in 0..columns {
            let ix = [channel, x, y];
            let current_value = view[ix];
            view[ix] = d.z_score(current_value);
        }
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
