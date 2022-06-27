use indexmap::IndexMap;
use std::{collections::HashMap, ops::Index};

use anyhow::{anyhow, Context, Error};
use rand::random;
use wasmer::{ImportObject, Module, Store};

use crate::zune::{
    DimensionsConstraint, ElementType, ElementTypeConstraint, GraphNode,
    Tensor, TensorConstraint, TensorConstraints,
};

use self::proc_block_v2::{CreateError, ProcBlockV2};
pub use self::{
    proc_block_v2::{
        Argument, Dimensions as WDimensions, ElementType as WElementType,
        ElementTypeConstraint as WElementTypeConstraint, Node,
        TensorConstraint as WTensorConstraint,
        TensorConstraints as WTensorConstraints, TensorParam as WTensorParam,
        TensorResult as WTensorResult,
    },
    runtime_v2::*,
};

wit_bindgen_wasmer::export!("../../wit-files/rune/runtime-v2.wit");
wit_bindgen_wasmer::import!("../../wit-files/rune/proc-block-v2.wit");

#[derive(Debug, Default, Clone, wasmer::WasmerEnv)]
struct Runtime {}

pub(crate) struct ProcBlockNode {
    node_id: String,
    pbv2: ProcBlockV2,
    node: Node,
    tensor_constraints: TensorConstraints,
}

impl GraphNode for ProcBlockNode {
    #[tracing::instrument(skip_all, level = "debug", fields(%node_id))]
    fn load(
        node_id: &str,
        args: &HashMap<String, String>,
        node_data: &[u8],
    ) -> Result<Box<dyn GraphNode>, Error>
    where
        Self: Sized,
    {
        let store = Store::default();
        let module = Module::new(&store, node_data)
            .context("Unable to load the module")?;

        let mut imports = ImportObject::default();
        add_to_imports(&store, &mut imports, Runtime::default());
        let (pbv2, _) = ProcBlockV2::instantiate(&store, &module, &mut imports)
            .context("Unable to instantiate the WebAssembly module")?;

        let args: Vec<Argument> = args
            .iter()
            .map(|(name, value)| Argument { name, value })
            .collect();
        let node = ProcBlockV2::create_node(&pbv2, &args)
            .map_err(Error::from)?
            .map_err(|x| anyhow!("Unable to create Node: {node_id} : {:?}", x))?;
        let tensor_constraints: TensorConstraints =
            ProcBlockV2::node_tensor_constraints(&pbv2, &node)
                .map_err(Error::from)?
                .into();

        Ok(Box::new(ProcBlockNode {
            node_id: node_id.to_string(),
            pbv2,
            node,
            tensor_constraints,
        }))
    }

    fn node_id(&self) -> &str {
        return &self.node_id;
    }

    fn tensor_constraints(&self) -> &TensorConstraints {
        return &self.tensor_constraints;
    }

    #[tracing::instrument(skip_all, level = "debug")]
    fn run(
        &mut self,
        inputs: HashMap<&str, &Tensor>,
    ) -> Result<HashMap<&str, Tensor>, Error> {
        let inputs: Vec<WTensorParam> = inputs
            .iter()
            .map(|(&name, &t)| WTensorParam {
                name,
                element_type: t.element_type.into(),
                dimensions: &t.dimensions,
                buffer: &t.buffer,
            })
            .collect();

        let mut result = ProcBlockV2::node_run(&self.pbv2, &self.node, &inputs)
            .map_err(Error::from)?
            .map_err(|x| anyhow!("Runtime Error: {:?}", x))?;

        let mut extract_result = |name: &str| -> Option<Tensor> {
            let t = result.iter_mut().find(|t| t.name == name);
            match t {
                Some(t) => {
                    let t = std::mem::replace(t, WTensorResult{ name: String::new(), element_type: WElementType::U8, dimensions: Vec::new(), buffer: Vec::new() });
                    Some(Tensor { element_type: t.element_type.into(), dimensions: t.dimensions, buffer: t.buffer })
                },
                None => None
            }
        };

        // TODO: Make sure to check for errors when trying to extract_result
        let outputs: Result<HashMap<&str, Tensor>, Error> =
            self.tensor_constraints
                .outputs
                .iter()
                .map(|(name, _)| -> Result<(&str, Tensor), Error> {
                    let t = extract_result(name);
                    match t {
                        Some(t) => Ok((name.as_str(), t)),
                        None => Err(anyhow!("Unable to find output tensor: {}", name))
                    }
                })
                .collect();

        Ok(outputs?)
    }
}

impl runtime_v2::RuntimeV2 for Runtime {
    #[tracing::instrument(skip_all, level = "debug")]
    fn is_enabled(&mut self, _metadata: LogMetadata) -> bool {
        true
    }

    fn abort(&mut self, msg: &str) {
        panic!("Node panicked: {}", msg);
    }

    fn get_random(&mut self, buffer: &mut [u8]) {
        buffer.iter_mut().for_each(|x| *x = rand::random::<u8>());
    }

    fn log(
        &mut self,
        metadata: LogMetadata,
        message: &str,
        data: Vec<(&'_ str, LogValue<'_>)>,
    ) {
        let level = match metadata.level {
            LogLevel::Trace => tracing::Level::TRACE,
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Warn => tracing::Level::WARN,
            LogLevel::Error => tracing::Level::ERROR,
        };

        let LogMetadata {
            name,
            target,
            level: _,
            file,
            line,
            module,
        } = metadata;

        tracing::event!(
            tracing::Level::INFO,
            meta.level = %level,
            meta.name = %name,
            meta.target = target,
            meta.file = file,
            meta.line = line,
            meta.module = module,
            ?data,
            message,
        );
    }
}

impl From<WElementType> for ElementType {
    fn from(w: WElementType) -> ElementType {
        match w {
            WElementType::U8 => ElementType::U8,
            WElementType::I8 => ElementType::I8,
            WElementType::U16 => ElementType::U16,
            WElementType::I16 => ElementType::I16,
            WElementType::U32 => ElementType::U32,
            WElementType::I32 => ElementType::I32,
            WElementType::F32 => ElementType::F32,
            WElementType::U64 => ElementType::U64,
            WElementType::I64 => ElementType::I64,
            WElementType::F64 => ElementType::F64,
            WElementType::Complex64 => ElementType::Complex64,
            WElementType::Complex128 => ElementType::Complex128,
            WElementType::Utf8 => ElementType::Utf8,
        }
    }
}

impl From<ElementType> for WElementType {
    fn from(e: ElementType) -> WElementType {
        match e {
            ElementType::U8 => WElementType::U8,
            ElementType::I8 => WElementType::I8,
            ElementType::U16 => WElementType::U16,
            ElementType::I16 => WElementType::I16,
            ElementType::U32 => WElementType::U32,
            ElementType::I32 => WElementType::I32,
            ElementType::F32 => WElementType::F32,
            ElementType::U64 => WElementType::U64,
            ElementType::I64 => WElementType::I64,
            ElementType::F64 => WElementType::F64,
            ElementType::Complex64 => WElementType::Complex64,
            ElementType::Complex128 => WElementType::Complex128,
            ElementType::Utf8 => WElementType::Utf8,
        }
    }
}

impl From<&WElementTypeConstraint> for ElementTypeConstraint {
    fn from(w: &WElementTypeConstraint) -> ElementTypeConstraint {
        ElementTypeConstraint::from_bits_truncate(w.bits() as u32)
    }
}

impl From<&WDimensions> for DimensionsConstraint {
    fn from(w: &WDimensions) -> DimensionsConstraint {
        match w {
            WDimensions::Dynamic => DimensionsConstraint::Dynamic,
            WDimensions::Fixed(s) => DimensionsConstraint::Fixed(s.clone()),
        }
    }
}

impl From<&WTensorConstraint> for TensorConstraint {
    fn from(w: &WTensorConstraint) -> TensorConstraint {
        TensorConstraint {
            element_types: (&w.element_type).into(),
            dimensions: (&w.dimensions).into(),
        }
    }
}

impl From<WTensorConstraints> for TensorConstraints {
    fn from(w: WTensorConstraints) -> TensorConstraints {
        TensorConstraints {
            inputs: w
                .inputs
                .iter()
                .map(|x| (x.name.to_owned(), x.into()))
                .collect(),
            outputs: w
                .outputs
                .iter()
                .map(|x| (x.name.to_owned(), x.into()))
                .collect(),
        }
    }
}
