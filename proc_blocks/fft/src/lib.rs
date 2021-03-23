#![feature(array_map)]
#![no_std]

extern crate alloc;

#[cfg(test)]
#[macro_use]
extern crate std;

use core::f64::consts::PI;
use alloc::vec::Vec;
use num_complex::Complex;
use runic_types::Transform;

const DEFAULT_SAMPLE_RATE: u32 = 16_000;

/// A [*Fast Fourier Transform*][fft] that transforms samples into a spectrogram
/// with `N` buckets, one bucket for each Hz.
///
/// [fft]: https://en.wikipedia.org/wiki/Fast_Fourier_transform
#[derive(Debug, Clone, PartialEq)]
pub struct Fft<const N: usize> {
    sample_rate: u32,
    preallocated_buffers: Buffer,
}

impl<const N: usize> Fft<N> {
    pub fn new() -> Self {
        Fft {
            sample_rate: DEFAULT_SAMPLE_RATE,
            preallocated_buffers: Buffer::new(N),
        }
    }

    pub fn with_sample_rate(self, sample_rate: u32) -> Self {
        Fft {
            sample_rate,
            ..self
        }
    }
}

impl<const N: usize> Default for Fft<N> {
    fn default() -> Self { Fft::new() }
}

impl<A, const N: usize> Transform<A> for Fft<N>
where
    A: AsRef<[i16]>,
{
    type Output = [i8; N];

    fn transform(&mut self, input: A) -> Self::Output {
        let Buffer {
            samples,
            output_a,
            output_b,
        } = &mut self.preallocated_buffers;

        samples.clear();
        for sample in input.as_ref() {
            samples.push(Complex::new(*sample as f64, 0.0));
        }

        let spectrum = Spectrum::from_samples(
            samples,
            self.sample_rate as f64,
            output_a,
            output_b,
        );

        let mut output = [0.0; N];

        for i in 0..N {
            output[i] = spectrum.lookup_frequency(i as f64);
        }

        // normalize and convert to i8
        let (min, max) = min_max(&output);
        let range = max - min;

        output.map(|value| {
            // scale to [-1, 1]
            let normalized = (value - min) * 2.0 / range;
            // then map to [-255, 255]
            libm::round(normalized * (i8::max_value() as f64)) as i8
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Buffer {
    samples: Vec<Complex<f64>>,
    output_a: Vec<Complex<f64>>,
    output_b: Vec<Complex<f64>>,
}

impl Buffer {
    fn new(output_size: usize) -> Self {
        Buffer {
            samples: Vec::new(),
            output_a: alloc::vec![Complex::default(); output_size],
            output_b: alloc::vec![Complex::default(); output_size],
        }
    }
}

fn min_max(items: &[f64]) -> (f64, f64) {
    items.iter().copied().fold(
        (core::f64::INFINITY, core::f64::NEG_INFINITY),
        |(min, max), value| (f64::min(min, value), f64::max(max, value)),
    )
}

#[derive(Debug)]
struct Spectrum<'a> {
    bins: &'a [Complex<f64>],
    sample_rate: f64,
}

impl<'a> Spectrum<'a> {
    fn from_samples(
        samples: &[Complex<f64>],
        sample_rate: f64,
        output_a: &'a mut Vec<Complex<f64>>,
        output_b: &'a mut Vec<Complex<f64>>,
    ) -> Self {
        Spectrum {
            bins: fft(&samples, output_a, output_b),
            sample_rate: sample_rate as f64,
        }
    }

    fn lookup_frequency(&self, frequency: f64) -> f64 {
        // Note: each bin contains the values for a range of frequencies and
        // are defined with the centre at multiples of the bin width.
        //
        // https://stackoverflow.com/questions/10754549/fft-bin-width-clarification

        if frequency < 0.0 {
            // Negative frequencies wrap around
            return self.bins.last().copied().unwrap_or_default().norm();
        }

        let bin_width = self.sample_rate / self.bins.len() as f64;

        let nearest_centre = libm::round(frequency / bin_width) as usize;

        let value = match self.bins.get(nearest_centre).copied() {
            Some(value) => value,
            // it's past our largest bin
            None => self.bins.last().copied().unwrap_or_default(),
        };

        value.norm()
    }
}

/// A Fast Fourier Transform implementation copied from [Rosetta Code][rc],
/// modified to avoid allocating memory.
///
/// [rc]: https://rosettacode.org/wiki/Fast_Fourier_transform#Rust
pub fn fft<'a>(
    input: &[Complex<f64>],
    buf_a: &'a mut Vec<Complex<f64>>,
    buf_b: &'a mut Vec<Complex<f64>>,
) -> &'a mut [Complex<f64>] {
    // round n (length) up to a power of 2:
    let n = input.len().next_power_of_two();

    // clear the previous buffer
    buf_a.clear();
    buf_a.reserve(n);
    // copy the input into a buffer
    buf_a.extend_from_slice(input);
    // pad out the rest with zeroes
    buf_a.extend(core::iter::repeat(Complex::default()).take(n - input.len()));

    // then copy it across to our second buffer
    buf_b.clear();
    buf_b.extend_from_slice(&buf_a);

    fft_recursive(buf_a, buf_b, n, 1);

    for element in buf_a.iter_mut() {
        *element /= n as f64;
    }

    buf_a
}

fn fft_recursive(
    buf_a: &mut [Complex<f64>],
    buf_b: &mut [Complex<f64>],
    n: usize,
    step: usize,
) {
    if step >= n {
        return;
    }

    fft_recursive(buf_b, buf_a, n, step * 2);
    fft_recursive(&mut buf_b[step..], &mut buf_a[step..], n, step * 2);
    let (left, right) = buf_a.split_at_mut(n / 2);

    for i in (0..n).step_by(step * 2) {
        let t = (Complex::new(0.0, -PI) * (i as f64) / (n as f64)).exp()
            * buf_b[i + step];
        left[i / 2] = buf_b[i] + t;
        right[i / 2] = buf_b[i] - t;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Testing data from http://www.sccon.ca/sccon/fft/fft3.htm

    #[test]
    fn impulse() {
        let mut input = [Complex::new(0.0, 0.0); 8];
        input[0] = Complex::new(1.000, 0.000);
        let should_be = [Complex::new(0.125, 0.0); 8];
        let mut a = Vec::new();
        let mut b = Vec::new();

        let got = fft(&input, &mut a, &mut b);

        assert_eq!(got, should_be);
    }

    #[test]
    fn shifted_impulse() {
        let mut input = [Complex::new(0.0, 0.0); 8];
        input[1] = Complex::new(1.000, 0.000);
        let should_be = [
            Complex::new(0.125, 0.000),
            Complex::new(0.088, -0.088),
            Complex::new(0.000, -0.125),
            Complex::new(-0.088, -0.088),
            Complex::new(-0.125, 0.000),
            Complex::new(-0.088, 0.088),
            Complex::new(0.000, 0.125),
            Complex::new(0.088, 0.088),
        ];
        let mut a = Vec::new();
        let mut b = Vec::new();

        let got = fft(&input, &mut a, &mut b);

        println!("{:?}", got);
        assert!(
            got.iter().all(|c| c.norm() == 0.125),
            "Each result should have the same magnitude"
        );
        for (i, (got, should_be)) in got.iter().zip(&should_be).enumerate() {
            let error = got - should_be;
            assert!(
                error.norm() < 0.001,
                "{:?} != {:?} at index {}",
                got,
                should_be,
                i
            );
        }
    }

    #[test]
    fn all_zeroes() {
        let input = [Complex::new(0.0, 0.0); 8];
        let should_be = [Complex::new(0.0, 0.0); 8];
        let mut a = Vec::new();
        let mut b = Vec::new();

        let got = fft(&input, &mut a, &mut b);

        assert_eq!(got, should_be);
    }

    #[test]
    fn one_hz_sine_wave() {
        // one full wavelength of a sine wave
        let input: Vec<_> = (0..360)
            .map(|deg| (deg as f64).to_radians().sin())
            .map(|sin| (sin * (i16::max_value() as f64 - 1.0)).round() as i16)
            .collect();
        // we sampled at 360 Hz and have 360 samples => 1 Hz sine wave
        let mut fft = Fft::default().with_sample_rate(360);

        let got: [i8; 32] = fft.transform(&input);

        let (peak_frequency, _value) = got
            .iter()
            .copied()
            .enumerate()
            .max_by_key(|(_, value)| *value)
            .unwrap();
        let expected_frequency = input.len() / fft.sample_rate as usize;
        assert_eq!(peak_frequency, expected_frequency);
    }
}
