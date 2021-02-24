pub static mut DEBUG_BUFFER: [u8; 1024] = [0; 1024];

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

    pub fn written(&self) -> &[u8] { &self.buffer[..self.bytes_written] }

    fn rest(&mut self) -> &mut [u8] { &mut self.buffer[self.bytes_written..] }

    pub fn flush(&mut self) {
        let msg = self.written();

        unsafe {
            crate::wasm32::intrinsics::_debug(msg.as_ptr(), msg.len() as u32);
        }

        self.bytes_written = 0;
    }
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

#[macro_export]
macro_rules! debug {
    ($fmt:literal $(, $arg:expr)*) => {
        {
            use core::fmt::Write as _;
            // SAFETY: This WebAssembly code will only ever be used by a single
            // thread at a time.
            unsafe {
                let mut buffer = $crate::wasm32::debug::BufWriter::new(&mut $crate::wasm32::debug::DEBUG_BUFFER);

                if write!(buffer,
                    concat!("[{}] ", $fmt),
                    core::panic::Location::caller(),
                    $($arg),*
                ).is_ok() {
                    buffer.flush();
                }
            }
        }
    };
}
