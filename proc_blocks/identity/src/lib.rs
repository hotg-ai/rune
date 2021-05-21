#![no_std]
use runic_types::{HasOutputs, Transform};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Identity;

impl<T> Transform<T> for Identity {
    type Output = T;

    fn transform(&mut self, input: T) -> Self::Output { input }
}

impl HasOutputs for Identity {}
