use alloc::{
    borrow::Cow,
    string::{String, ToString},
    vec::Vec,
};
use core::{
    fmt::{self, Formatter, Display},
    num::ParseIntError,
    str::FromStr,
};
use crate::reflect::Type;

/// A tensor's shape.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Shape<'a> {
    element_type: Type,
    dimensions: Cow<'a, [usize]>,
}

impl<'a> Shape<'a> {
    pub fn new(
        element_type: Type,
        dimensions: impl Into<Cow<'a, [usize]>>,
    ) -> Self {
        Shape {
            element_type,
            dimensions: dimensions.into(),
        }
    }

    pub fn element_type(&self) -> &Type { &self.element_type }

    pub fn dimensions(&self) -> &[usize] { &self.dimensions }

    /// The number of bytes this tensor would take up.
    pub fn size(&self) -> usize {
        self.dimensions.iter().product::<usize>()
            * self.element_type.size_of().unwrap()
    }

    pub fn to_owned(&self) -> Shape<'static> {
        let Shape {
            element_type,
            dimensions,
        } = self;

        Shape::new(element_type.clone(), dimensions.clone().into_owned())
    }

    /// Get a "simplified" version of this [`Shape`] which ignores dimensions
    /// with a single length.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hotg_rune_core::Shape;
    /// let complex: Shape = "f32[1, 1, 3, 256, 256, 1]".parse().unwrap();
    /// assert_eq!(complex.simplified_dimensions(), &[3, 256, 256]);
    /// ```
    pub fn simplified_dimensions(&self) -> &[usize] {
        let mut dimensions = self.dimensions.as_ref();

        while dimensions.len() > 1 {
            dimensions = match dimensions {
                [1, rest @ ..] => rest,
                [rest @ .., 1] => rest,
                _ => break,
            };
        }

        dimensions
    }
}

impl<'a> Display for Shape<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Shape {
            element_type,
            dimensions,
        } = self;
        let element_type_name = element_type.rust_name().ok_or(fmt::Error)?;
        write!(f, "{}[", element_type_name)?;

        for (i, dim) in dimensions.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }

            write!(f, "{}", dim)?;
        }

        write!(f, "]")?;
        Ok(())
    }
}

impl FromStr for Shape<'static> {
    type Err = FormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let opening_bracket = s.find('[').ok_or(FormatError::Malformed)?;
        let element_type = s[..opening_bracket].trim();
        let ty = Type::from_rust_name(element_type).ok_or_else(|| {
            FormatError::UnknownElementType {
                found: element_type.to_string(),
            }
        })?;

        let closing_bracket = s.rfind(']').ok_or(FormatError::Malformed)?;

        let between_brackets = &s[opening_bracket + 1..closing_bracket];

        let mut dimensions = Vec::new();

        for word in between_brackets.split(',') {
            let word = word.trim();
            let dimension = word.parse::<usize>().map_err(|e| {
                FormatError::BadDimension {
                    found: word.to_string(),
                    reason: e,
                }
            })?;
            dimensions.push(dimension);
        }

        Ok(Shape {
            element_type: ty,
            dimensions: dimensions.into(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FormatError {
    Malformed,
    UnknownElementType {
        found: String,
    },
    BadDimension {
        found: String,
        reason: ParseIntError,
    },
}

impl Display for FormatError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FormatError::Malformed => write!(f, "Malformed shape"),
            FormatError::UnknownElementType { found } => {
                write!(f, "Couldn't recognise the \"{}\" element type", found)
            },
            FormatError::BadDimension { found, .. } => {
                write!(f, "\"{}\" isn't a valid dimension", found)
            },
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for FormatError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FormatError::BadDimension { reason, .. } => Some(reason),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::prelude::v1::*;

    const SHAPES: &[(Shape, &str)] = &[
        (
            Shape {
                element_type: Type::f32,
                dimensions: Cow::Borrowed(&[1, 2, 3]),
            },
            "f32[1, 2, 3]",
        ),
        (
            Shape {
                element_type: Type::u8,
                dimensions: Cow::Borrowed(&[42]),
            },
            "u8[42]",
        ),
    ];

    #[test]
    fn shape_format() {
        for (shape, should_be) in SHAPES.iter().cloned() {
            let got = shape.to_string();
            assert_eq!(got, should_be);
        }
    }

    #[test]
    fn parse() {
        for (should_be, src) in SHAPES.iter().cloned() {
            let got: Shape = src.parse().unwrap();
            assert_eq!(got, should_be);
        }
    }
}
