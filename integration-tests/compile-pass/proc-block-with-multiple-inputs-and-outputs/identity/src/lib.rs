#![no_std]
use hotg_rune_core::HasOutputs;
use hotg_rune_proc_blocks::{Transform, ProcBlock};

#[derive(Debug, Default, Copy, Clone, PartialEq, ProcBlock)]
#[transform(inputs = ([f32; _], [f32; _]), outputs = ([f32; _], [f32; _]))]
pub struct Identity;

impl<T> Transform<T> for Identity {
    type Output = T;

    fn transform(&mut self, input: T) -> Self::Output { input }
}

impl HasOutputs for Identity {}
