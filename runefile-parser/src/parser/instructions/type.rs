

pub enum Type {
    F32,
    F64,
    U8,
    U16,
    U32,
    I8,
    I16,
    I32,
    I64,
    UTF8
    NONE
}
pub struct InterfaceType {
    pub dimensions: Vec<u32>,
    pub type: Type
}