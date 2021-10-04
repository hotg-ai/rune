#![no_std]
use hotg_rune_proc_blocks::{Transform, ProcBlock};

#[derive(Debug, Default, Copy, Clone, PartialEq, ProcBlock)]
#[transform(inputs = u8, outputs = u8)]
#[transform(inputs = i8, outputs = i8)]
#[transform(inputs = u16, outputs = u16)]
#[transform(inputs = i16, outputs = i16)]
#[transform(inputs = u32, outputs = u32)]
#[transform(inputs = i32, outputs = i32)]
#[transform(inputs = f32, outputs = f32)]
pub struct Identity;

impl<T> Transform<T> for Identity {
    type Output = T;

    fn transform(&mut self, input: T) -> Self::Output { input }
}
