use im::Vector;

use crate::{
    inputs::Inputs,
    lowering::{Model, ModelData, Name, ProcBlock, Resource, ResourceData},
};

#[salsa::query_group(CodegenInputsGroup)]
pub trait CodegenInputs: Inputs {
    #[salsa::input]
    fn node_inputs(&self, name: Name) -> crate::lowering::Inputs;
    #[salsa::input]
    fn node_outputs(&self, name: Name) -> crate::lowering::Outputs;

    #[salsa::input]
    fn resource_names(&self) -> Vector<Name>;
    #[salsa::input]
    fn resource_info(&self, name: Name) -> Vector<Resource>;
    #[salsa::input]
    fn resource_data(&self, name: Name) -> ResourceData;
    fn all_resource_data(&self) -> Vector<(Name, ResourceData)>;

    #[salsa::input]
    fn proc_block_names(&self) -> Vector<Name>;
    #[salsa::input]
    fn proc_block_info(&self, name: Name) -> ProcBlock;
    fn all_proc_blocks(&self) -> Vector<ProcBlock>;

    #[salsa::input]
    fn model_names(&self) -> Vector<Name>;
    #[salsa::input]
    fn model_info(&self, name: Name) -> Model;
    #[salsa::input]
    fn model_data(&self, name: Name) -> ModelData;
    fn all_model_data(&self) -> Vector<(Name, ModelData)>;
}

fn all_model_data(db: &dyn CodegenInputs) -> Vector<(Name, ModelData)> {
    db.model_names()
        .into_iter()
        .map(|name| (name.clone(), db.model_data(name)))
        .collect()
}

fn all_resource_data(db: &dyn CodegenInputs) -> Vector<(Name, ResourceData)> {
    db.resource_names()
        .into_iter()
        .map(|name| (name.clone(), db.resource_data(name)))
        .collect()
}

fn all_proc_blocks(db: &dyn CodegenInputs) -> Vector<ProcBlock> {
    db.proc_block_names()
        .into_iter()
        .map(|name| db.proc_block_info(name))
        .collect()
}
