use core::{
    convert::{TryFrom, TryInto},
    fmt::{self, Display, Formatter},
};

/// A normal distribution.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Distribution {
    pub mean: f32,
    pub standard_deviation: f32,
}

impl Distribution {
    pub const fn new(mean: f32, standard_deviation: f32) -> Self {
        Distribution {
            mean,
            standard_deviation,
        }
    }

    pub fn z_score(&self, value: f32) -> f32 {
        (value - self.mean) / self.standard_deviation
    }
}

impl Default for Distribution {
    fn default() -> Self {
        Distribution {
            mean: 0.0,
            standard_deviation: 1.0,
        }
    }
}

impl From<[f32; 2]> for Distribution {
    fn from([mean, standard_deviation]: [f32; 2]) -> Self {
        Distribution {
            mean,
            standard_deviation,
        }
    }
}

impl From<[i32; 2]> for Distribution {
    fn from([mean, standard_deviation]: [i32; 2]) -> Self {
        Distribution::new(mean as f32, standard_deviation as f32)
    }
}

impl<'a> TryFrom<[&'a str; 2]> for Distribution {
    type Error = DistributionConversionError<'a>;

    fn try_from([mean, std_dev]: [&'a str; 2]) -> Result<Self, Self::Error> {
        let mean: f32 = mean.parse().map_err(|err| {
            DistributionConversionError::ParseFloat { value: mean, err }
        })?;
        let standard_deviation: f32 = std_dev.parse().map_err(|err| {
            DistributionConversionError::ParseFloat {
                value: std_dev,
                err,
            }
        })?;

        Ok(Distribution {
            mean,
            standard_deviation,
        })
    }
}

impl<'a, T> TryFrom<&'a [T; 2]> for Distribution
where
    Distribution: TryFrom<[T; 2]>,
    T: Copy,
{
    type Error = <Distribution as TryFrom<[T; 2]>>::Error;

    fn try_from(input: &'a [T; 2]) -> Result<Self, Self::Error> {
        Distribution::try_from(*input)
    }
}

impl<'a, T> TryFrom<&'a [T]> for Distribution
where
    Distribution: TryFrom<[T; 2], Error = DistributionConversionError<'a>>,
    T: Copy,
{
    type Error = DistributionConversionError<'a>;

    fn try_from(input: &'a [T]) -> Result<Self, Self::Error> {
        match *input {
            [mean, std_dev] => [mean, std_dev].try_into(),
            _ => Err(DistributionConversionError::IncorrectArrayLength {
                actual_length: input.len(),
            }),
        }
    }
}

impl<T> TryFrom<(T, T)> for Distribution
where
    Distribution: TryFrom<[T; 2]>,
{
    type Error = <Distribution as TryFrom<[T; 2]>>::Error;

    fn try_from((mean, std_dev): (T, T)) -> Result<Self, Self::Error> {
        [mean, std_dev].try_into()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DistributionConversionError<'a> {
    ParseFloat {
        value: &'a str,
        err: core::num::ParseFloatError,
    },
    IncorrectArrayLength {
        actual_length: usize,
    },
}

impl<'a> Display for DistributionConversionError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DistributionConversionError::ParseFloat { value, err } => {
                write!(f, "Unable to parse \"{}\" as a number: {}", value, err)
            },
            DistributionConversionError::IncorrectArrayLength {
                actual_length,
            } => write!(
                f,
                "Expected an array with 2 elements but found {}",
                actual_length
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_z_score() {
        let distribution = Distribution::new(100.0, 15.0);
        let value = 110.0;

        let got = distribution.z_score(value);

        assert_eq!(got, 0.6666667);
    }

    #[test]
    fn parse_distribution_from_two_strings() {
        let src = &["1.75", "5"];
        let should_be = Distribution::new(1.75, 5.0);

        let got: Distribution = src.try_into().unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn incorrect_slice_length() {
        let src = &["1.75", "5"];
        let should_be = Distribution::new(1.75, 5.0);

        let got: Distribution = src.try_into().unwrap();

        assert_eq!(got, should_be);
    }
}
