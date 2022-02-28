use std::{collections::HashMap, str::FromStr};

use anyhow::{Context, Error};

/// Helper methods for reading arguments.
#[derive(Debug, Clone, PartialEq)]
#[repr(transparent)]
pub struct Arguments(pub HashMap<String, String>);

impl Arguments {
    pub fn parse<T>(&self, name: &str) -> Result<T, Error>
    where
        T: FromStr,
        T::Err: std::error::Error + Send + Sync + 'static,
    {
        match self.0.get(name) {
            Some(value) => value.parse::<T>().with_context(|| {
                format!(
                    "Unable to parse {:?} as the \"{}\" argument",
                    value, name
                )
            }),
            None => {
                Err(anyhow::anyhow!("The \"{}\" argument wasn't set", name))
            },
        }
    }

    pub fn parse_or_default<T>(
        &self,
        name: &str,
        default: T,
    ) -> Result<T, Error>
    where
        T: FromStr,
        T::Err: std::error::Error + Send + Sync + 'static,
    {
        match self.0.get(name) {
            Some(value) => value.parse::<T>().with_context(|| {
                format!(
                    "Unable to parse {:?} as the \"{}\" argument",
                    value, name
                )
            }),
            None => Ok(default),
        }
    }
}
