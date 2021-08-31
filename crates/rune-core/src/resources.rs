use core::fmt::{self, Formatter, Debug};

/// Read an [`InlineResource`]'s data from its binary form in memory.
///
/// # Examples
///
/// ```rust
/// # use hotg_rune_core::{inline_resource_from_bytes, InlineResource};
/// let resource = InlineResource::new(*b"Name", *b"Some Value");
/// let bytes = resource.as_bytes();
///
/// let (name, value, rest) = inline_resource_from_bytes(bytes).unwrap();
///
/// assert_eq!(name, "Name");
/// assert_eq!(value, b"Some Value");
/// assert!(rest.is_empty());
/// ```
pub fn inline_resource_from_bytes(
    bytes: &[u8],
) -> Option<(&str, &[u8], &[u8])> {
    let (name_len, rest) = read_u32(bytes)?;
    if rest.len() < name_len {
        return None;
    }
    let (name, bytes) = rest.split_at(name_len);
    let name = core::str::from_utf8(name).ok()?;

    let (data_len, bytes) = read_u32(bytes)?;
    if bytes.len() < data_len {
        return None;
    }
    let (data, bytes) = bytes.split_at(data_len as usize);

    Some((name, data, bytes))
}

fn read_u32(buffer: &[u8]) -> Option<(usize, &[u8])> {
    if buffer.len() < 4 {
        return None;
    }

    let (head, tail) = buffer.split_at(4);

    let mut number = [0; 4];
    number.copy_from_slice(head);

    Some((u32::from_be_bytes(number) as usize, tail))
}

/// A `(&str, &[u8])` which stores the first field (name) and second field
/// (data) inline and can be read out of memory using
/// [`inline_resource_from_bytes()`].
#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct InlineResource<const NAME_LEN: usize, const DATA_LEN: usize> {
    _name_len: [u8; 4],
    name: [u8; NAME_LEN],
    _data_len: [u8; 4],
    data: [u8; DATA_LEN],
}

impl<const NAME_LEN: usize, const DATA_LEN: usize>
    InlineResource<NAME_LEN, DATA_LEN>
{
    pub const fn new(name: [u8; NAME_LEN], data: [u8; DATA_LEN]) -> Self {
        InlineResource {
            _name_len: (NAME_LEN as u32).to_be_bytes(),
            name,
            _data_len: (DATA_LEN as u32).to_be_bytes(),
            data,
        }
    }

    fn name(&self) -> Option<&str> {
        core::str::from_utf8(self.name_raw()).ok()
    }

    fn name_raw(&self) -> &[u8] { &self.name }

    fn data(&self) -> &[u8] { &self.data }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self as *const Self as *const u8,
                core::mem::size_of::<Self>(),
            )
        }
    }
}

impl<const NAME_LEN: usize, const DATA_LEN: usize> Debug
    for InlineResource<NAME_LEN, DATA_LEN>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("InlineResource");

        match self.name() {
            Some(n) => {
                d.field("name", &n);
            },
            None => {
                d.field("name", &"(non-utf8)");
            },
        }

        d.field(
            "data",
            &format_args!("({} bytes hidden)", self.data().len()),
        );

        d.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_from_binary() {
        let resource = InlineResource::new(*b"name", *b"value");
        let as_bytes = resource.as_bytes();

        let (got_name, got_value, rest) =
            inline_resource_from_bytes(as_bytes).unwrap();

        assert_eq!(got_name, resource.name().unwrap());
        assert_eq!(got_value, resource.data());
        assert!(rest.is_empty());
    }
}
