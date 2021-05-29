#![no_std]

mod distribution;

pub use crate::distribution::{Distribution, DistributionConversionError};

use core::{convert::TryInto, fmt::Display};
use runic_types::{HasOutputs, Tensor, Transform, TensorViewMut};

#[derive(Debug, Default, Clone, PartialEq)]
#[non_exhaustive]
pub struct ImageNormalization {
    pub red: Distribution,
    pub green: Distribution,
    pub blue: Distribution,
}

impl ImageNormalization {
    /// A shortcut for initializing the red, green, and blue distributions in
    /// one call.
    pub fn set_rgb<D>(&mut self, distribution: D) -> &mut Self
    where
        D: TryInto<Distribution>,
        D::Error: Display,
    {
        let d = match distribution.try_into() {
            Ok(d) => d,
            Err(e) => panic!("Invalid distribution: {}", e),
        };

        self.set_red(d).set_green(d).set_blue(d)
    }

    pub fn set_red<D>(&mut self, distribution: D) -> &mut Self
    where
        D: TryInto<Distribution>,
        D::Error: Display,
    {
        match distribution.try_into() {
            Ok(d) => self.red = d,
            Err(e) => panic!("Invalid distribution: {}", e),
        }

        self
    }

    pub fn set_green<D>(&mut self, distribution: D) -> &mut Self
    where
        D: TryInto<Distribution>,
        D::Error: Display,
    {
        match distribution.try_into() {
            Ok(d) => self.green = d,
            Err(e) => panic!("Invalid distribution: {}", e),
        }

        self
    }

    pub fn set_blue<D>(&mut self, distribution: D) -> &mut Self
    where
        D: TryInto<Distribution>,
        D::Error: Display,
    {
        match distribution.try_into() {
            Ok(d) => self.blue = d,
            Err(e) => panic!("Invalid distribution: {}", e),
        }

        self
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

    for row in 0..rows {
        for column in 0..columns {
            let ix = [channel, row, column];
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizing_with_default_distribution_is_noop() {
        let red = [[1.0], [4.0], [7.0], [10.0]];
        let green = [[2.0], [5.0], [8.0], [11.0]];
        let blue = [[3.0], [6.0], [9.0], [12.0]];
        let image: Tensor<f32> = Tensor::from([red, green, blue]);
        let mut norm = ImageNormalization::default();

        let got = norm.transform(image.clone());

        assert_eq!(got, image);
    }
}
