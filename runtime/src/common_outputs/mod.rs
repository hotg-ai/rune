mod serial;

use std::fmt::Debug;

pub use self::serial::Serial;

use anyhow::Error;

/// Something a Rune can send output to.
pub trait Output: Send + Debug + 'static {
    fn consume(&mut self, buffer: &[u8]) -> Result<(), Error>;
}
