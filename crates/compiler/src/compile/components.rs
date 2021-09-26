use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    ops::Deref,
    path::PathBuf,
    process::ExitStatus,
    sync::Arc,
};

#[derive(Debug, Clone, PartialEq)]
pub struct CompiledBinary(pub Arc<[u8]>);

impl From<Vec<u8>> for CompiledBinary {
    fn from(bytes: Vec<u8>) -> Self { CompiledBinary(bytes.into()) }
}

impl AsRef<[u8]> for CompiledBinary {
    fn as_ref(&self) -> &[u8] { &self.0 }
}

impl Deref for CompiledBinary {
    type Target = Arc<[u8]>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

/// The result from compiling... Essentially a newtype'd `Result`.
#[derive(Debug)]
pub struct CompilationResult(pub Result<CompiledBinary, CompileError>);

#[derive(Debug)]
pub enum CompileError {
    BuildFailed(ExitStatus),
    DidntStart(std::io::Error),
    UnableToReadBinary {
        path: PathBuf,
        error: std::io::Error,
    },
}

impl Display for CompileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CompileError::BuildFailed(exit) => match exit.code() {
                Some(code) => {
                    write!(f, "Compilation failed with exit code {}", code,)
                },
                None => f.write_str("Compilation failed"),
            },
            CompileError::DidntStart(_) => {
                f.write_str("Unable to run the compiler. Is cargo installed?")
            },
            CompileError::UnableToReadBinary { path, .. } => {
                write!(f, "Unable to read \"{}\"", path.display())
            },
        }
    }
}

impl Error for CompileError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CompileError::BuildFailed(_) => None,
            CompileError::DidntStart(e) => Some(e),
            CompileError::UnableToReadBinary { error, .. } => Some(error),
        }
    }
}
