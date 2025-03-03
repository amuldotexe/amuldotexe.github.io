use std::fmt;
use std::io;

pub type Result<T> = std::result::Result<T, ChunkError>;

#[derive(Debug)]
pub enum ChunkError {
    Io(io::Error),
    InvalidInput(&'static str),
}

impl fmt::Display for ChunkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChunkError::Io(err) => write!(f, "IO error: {}", err),
            ChunkError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl std::error::Error for ChunkError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ChunkError::Io(err) => Some(err),
            ChunkError::InvalidInput(_) => None,
        }
    }
}

impl From<io::Error> for ChunkError {
    fn from(err: io::Error) -> Self {
        ChunkError::Io(err)
    }
}
