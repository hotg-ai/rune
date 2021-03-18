#[derive(Debug)]
pub struct BufWriter<'buf> {
    buffer: &'buf mut [u8],
    bytes_written: usize,
}

impl<'buf> BufWriter<'buf> {
    pub fn new(buffer: &'buf mut [u8]) -> Self {
        BufWriter {
            buffer,
            bytes_written: 0,
        }
    }

    pub fn written(self) -> &'buf [u8] {
        let BufWriter {
            buffer,
            bytes_written,
        } = self;
        &buffer[..bytes_written]
    }

    fn rest(&mut self) -> &mut [u8] { &mut self.buffer[self.bytes_written..] }
}

impl<'buf> core::fmt::Write for BufWriter<'buf> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let rest = self.rest();

        if rest.len() < s.len() {
            return Err(core::fmt::Error);
        }

        rest[..s.len()].copy_from_slice(s.as_bytes());
        self.bytes_written += s.len();

        Ok(())
    }
}
