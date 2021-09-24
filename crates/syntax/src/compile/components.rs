use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    path::PathBuf,
    process::ExitStatus,
    sync::Arc,
};

#[derive(Debug, Clone, PartialEq)]
pub struct CompiledBinary(pub Arc<[u8]>);

impl From<Vec<u8>> for CompiledBinary {
    fn from(bytes: Vec<u8>) -> Self { CompiledBinary(bytes.into()) }
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
