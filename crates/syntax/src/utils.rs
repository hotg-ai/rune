use std::ops::Range;

use codespan::Span;

use crate::hir::{HirId, Primitive, Rune, Type};

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

    pub(crate) fn copy_into(&self, rune: &mut Rune) {
        self.for_each(|id, ty| {
            rune.types.insert(id, ty);
        });
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

    pub(crate) fn for_each(&self, mut f: impl FnMut(HirId, Type)) {
        let Builtins {
            unknown_type,
            u8,
            i8,
            u16,
            i16,
            u32,
            i32,
            u64,
            i64,
            f32,
            f64,
            string,
        } = *self;

        f(unknown_type, Type::Unknown);
        f(u8, Type::Primitive(Primitive::U8));
        f(i8, Type::Primitive(Primitive::I8));
        f(u16, Type::Primitive(Primitive::U16));
        f(i16, Type::Primitive(Primitive::I16));
        f(u32, Type::Primitive(Primitive::U32));
        f(i16, Type::Primitive(Primitive::I16));
        f(i32, Type::Primitive(Primitive::I32));
        f(u64, Type::Primitive(Primitive::U64));
        f(i64, Type::Primitive(Primitive::I64));
        f(f32, Type::Primitive(Primitive::F32));
        f(f64, Type::Primitive(Primitive::F64));
        f(string, Type::Primitive(Primitive::String));
    }
}

pub(crate) fn range_span(span: Span) -> Range<usize> {
    span.start().to_usize()..span.end().to_usize()
}
