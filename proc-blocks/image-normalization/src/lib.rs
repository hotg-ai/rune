#![no_std]

mod distribution;

pub use crate::distribution::{Distribution, DistributionConversionError};

use core::{convert::TryInto, fmt::Display};
use rune_core::{HasOutputs, Tensor, TensorViewMut};
use rune_proc_blocks::{ProcBlock, Transform};

#[derive(Debug, Default, Clone, PartialEq, ProcBlock)]
#[non_exhaustive]
#[transform(input = [u8; 4], output = [f32; 4])]
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
        let mut view = input.view_mut::<4>()
            .expect("The image normalization proc block only supports outputs of the form [frames, rows, columns, channels]");

        let [frames, _rows, _columns, channels] = view.dimensions();

        assert_eq!(channels, 3,
                "the image normalization proc block only supports images with 3 channels, but there are {} channels in a {}",
                channels,  input.shape(),
        );

        for frame in 0..frames {
            transform(self.red, frame, 0, &mut view);
            transform(self.green, frame, 1, &mut view);
            transform(self.blue, frame, 2, &mut view);
        }

        input
    }
}

fn transform(
    d: Distribution,
    frame: usize,
    channel: usize,
    view: &mut TensorViewMut<'_, f32, 4>,
) {
    let [_frames, rows, columns, _channels] = view.dimensions();

    for row in 0..rows {
        for column in 0..columns {
            let ix = [frame, row, column, channel];
            let current_value = view[ix];
            view[ix] = d.z_score(current_value);
        }
    }
}

impl HasOutputs for ImageNormalization {
    fn set_output_dimensions(&mut self, dimensions: &[usize]) {
        match *dimensions {
            [_, _, _, 3] => {},
            [_, _, _, channels] => panic!(
                "The number of channels should be either 1 or 3, found {}",
                channels
            ),
            _ => panic!("The image normalization proc block only supports outputs of the form [frames, rows, columns, channels], found {:?}", dimensions),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizing_with_default_distribution_is_noop() {
        let pixel_11 = [1.0, 2.0, 3.0];
        let pixel_12 = [4.0, 5.0, 6.0];
        let first_row = [pixel_11, pixel_12];
        let pixel_21 = [7.0, 8.0, 9.0];
        let pixel_22 = [10.0, 11.0, 12.0];
        let second_row = [pixel_21, pixel_22];
        let frame = [first_row, second_row];
        let image: Tensor<f32> = Tensor::from([frame]);
        let mut norm = ImageNormalization::default();

        let got = norm.transform(image.clone());

        assert_eq!(got, image);
    }
}
