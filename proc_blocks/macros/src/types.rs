use proc_macro2::Ident;
use syn::{Path, Type};
use crate::descriptor::ProcBlockDescriptor;

#[derive(Debug)]
pub(crate) struct DeriveOutput {
    pub trait_impl: ProcBlockImpl,
    pub custom_section: CustomSection,
    pub setters: Setters,
    pub assertions: Assertions,
}

#[derive(Debug)]
pub(crate) struct ProcBlockImpl {
    pub type_name: Ident,
    pub exports: Path,
    pub descriptor: ProcBlockDescriptor<'static>,
}

#[derive(Debug)]
pub(crate) struct CustomSection {
    pub type_name: Ident,
    pub payload: Vec<u8>,
}

#[derive(Debug)]
pub(crate) struct Setters(Vec<Setter>);

#[derive(Debug)]
pub(crate) struct Setter {
    pub property: Ident,
    pub property_type: syn::Type,
}

#[derive(Debug)]
pub(crate) struct Assertions {
    pub set: SetterAssertions,
    pub transform: TransformAssertions,
}

#[derive(Debug)]
pub(crate) struct SetterAssertions(Vec<SetterAssertion>);

#[derive(Debug)]
pub(crate) struct SetterAssertion {
    pub proc_block_type: Ident,
    pub property: Ident,
    pub property_type: Type,
}

#[derive(Debug)]
pub(crate) struct TransformAssertions {
    pub proc_block_type: Ident,
    pub exports: Path,
    pub assertions: Vec<TransformAssertion>,
}

#[derive(Debug)]
pub(crate) struct TransformAssertion {
    pub input: Type,
    pub output: Type,
}
