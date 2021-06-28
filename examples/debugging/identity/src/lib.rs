#![no_std]
use runic_types::HasOutputs;
use rune_proc_blocks::{Transform, ProcBlock};

#[derive(Debug, Default, Copy, Clone, PartialEq, ProcBlock)]
#[transform(input=u8, output=u8)]
#[transform(input=i8, output=i8)]
#[transform(input=u16, output=u16)]
#[transform(input=i16, output=i16)]
#[transform(input=u32, output=u32)]
#[transform(input=i32, output=i32)]
#[transform(input=f32, output=f32)]
pub struct Identity;

impl<T> Transform<T> for Identity {
    type Output = T;

    fn transform(&mut self, input: T) -> Self::Output { input }
}

impl HasOutputs for Identity {}
