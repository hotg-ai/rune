use std::{
    fmt::{self, Debug, Display, Formatter},
    sync::Arc,
};

use uriparse::URI;

use crate::{im::Vector, Text};

/// An abstract filesystem.
pub trait FileSystem {
    /// Read a file's contents from somewhere, with the interpretation changing
    /// depending on the URI's scheme.
    fn read(&self, path: &URI<'_>) -> Result<Vector<u8>, ReadError>;
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ReadError {
    #[error("The \"{}\" scheme isn't supported", scheme)]
    UnsupportedScheme { scheme: Text },
    #[error(transparent)]
    Other(Arc<dyn std::error::Error + Send + Sync + 'static>),
}

impl ReadError {
    pub fn msg(error_message: impl Display + Send + Sync + 'static) -> Self {
        struct Message<T: Display>(T);

        impl<T: Display> Display for Message<T> {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        impl<T: Display> Debug for Message<T> {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.debug_tuple("Message")
                    .field(&format_args!("{}", self.0))
                    .finish()
            }
        }

        impl<T: Display> std::error::Error for Message<T> {}

        ReadError::other(Message(error_message))
    }

    pub fn other(
        error: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        ReadError::Other(Arc::new(error))
    }
}
