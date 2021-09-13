use std::ops::Range;

use codespan::Span;

use crate::hir::{HirId, Primitive};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct HirIds {
    last_id: HirId,
}

impl HirIds {
    pub(crate) fn new() -> Self {
        HirIds {
            last_id: HirId::ERROR,
        }
    }

    pub(crate) fn next(&mut self) -> HirId {
        let id = self.last_id.next();
        self.last_id = id;
        id
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct Builtins {
    pub(crate) unknown_type: HirId,
    pub(crate) u8: HirId,
    pub(crate) i8: HirId,
    pub(crate) u16: HirId,
    pub(crate) i16: HirId,
    pub(crate) u32: HirId,
    pub(crate) i32: HirId,
    pub(crate) u64: HirId,
    pub(crate) i64: HirId,
    pub(crate) f32: HirId,
    pub(crate) f64: HirId,
    pub(crate) string: HirId,
}

impl Builtins {
    pub(crate) fn new(ids: &mut HirIds) -> Self {
        Builtins {
            unknown_type: ids.next(),
            u8: ids.next(),
            i8: ids.next(),
            u16: ids.next(),
            i16: ids.next(),
            u32: ids.next(),
            i32: ids.next(),
            u64: ids.next(),
            i64: ids.next(),
            f32: ids.next(),
            f64: ids.next(),
            string: ids.next(),
        }
    }

    pub(crate) fn get_id(&self, primitive: Primitive) -> HirId {
        match primitive {
            Primitive::U8 => self.u8,
            Primitive::I8 => self.i8,
            Primitive::U16 => self.u16,
            Primitive::I16 => self.i16,
            Primitive::U32 => self.u32,
            Primitive::I32 => self.i32,
            Primitive::U64 => self.u64,
            Primitive::I64 => self.i64,
            Primitive::F32 => self.f32,
            Primitive::F64 => self.f64,
            Primitive::String => self.string,
        }
    }
}

pub(crate) fn range_span(span: Span) -> Range<usize> {
    span.start().to_usize()..span.end().to_usize()
}

#[cfg(test)]
macro_rules! map {
    // map-like
    ($($k:ident : $v:expr),* $(,)?) => {
        std::iter::Iterator::collect(std::array::IntoIter::new([
            $(
                (String::from(stringify!($k)), $v)
            ),*
        ]))
    };
    // set-like
    ($($v:expr),* $(,)?) => {
        std::iter::Iterator::collect(std::array::IntoIter::new([$($v,)*]))
    };
}

#[cfg(test)]
macro_rules! ty {
        ($type:ident [$($dim:expr),*]) => {
            crate::yaml::Type {
                name: String::from(stringify!($type)),
                dimensions: vec![ $($dim),*],
            }
        };
        ($type:ident) => {
            crate::yaml::Type {
                name: String::from(stringify!($type)),
                dimensions: vec![],
            }
        }
    }

#[cfg(test)]
pub fn dummy_document() -> crate::yaml::Document {
    use indexmap::IndexMap;
    use crate::yaml::{Document, DocumentV1, Path, Stage, Value};

    Document::V1(DocumentV1 {
        image: Path::new("runicos/base".to_string(), None, None),
        pipeline: map! {
            audio: Stage::Capability {
                capability: String::from("SOUND"),
                outputs: vec![
                    ty!(i16[16000]),
                ],
                args: map! {
                    hz: Value::from(16000),
                },
            },
            fft: Stage::ProcBlock {
                proc_block: "hotg-ai/rune#proc_blocks/fft".parse().unwrap(),
                inputs: vec!["audio".parse().unwrap()],
                outputs: vec![
                    ty!(i8[1960]),
                ],
                args: IndexMap::new(),
            },
            model: Stage::Model {
                model: "./model.tflite".into(),
                inputs: vec!["fft".parse().unwrap()],
                outputs: vec![
                    ty!(i8[6]),
                ],
            },
            label: Stage::ProcBlock {
                proc_block: "hotg-ai/rune#proc_blocks/ohv_label".parse().unwrap(),
                inputs: vec!["model".parse().unwrap()],
                outputs: vec![
                    ty!(utf8),
                ],
                args: map! {
                    labels: Value::List(vec![
                        Value::from("silence"),
                        Value::from("unknown"),
                        Value::from("up"),
                    ]),
                },
            },
            output: Stage::Out {
                out: String::from("SERIAL"),
                inputs: vec!["label".parse().unwrap()],
                args: IndexMap::default(),
            }
        },
        resources: map![],
    })
}
