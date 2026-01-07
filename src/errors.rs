use std::fmt;

#[derive(Debug)]
pub enum CompressionError {
    FileSizeError(String),
    IoError(String, std::io::Error),
    DateError(String, std::io::Error),
    CompressionFailed(String),
}

impl fmt::Display for CompressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompressionError::FileSizeError(msg) => write!(f, "{}", msg),
            CompressionError::IoError(msg, err) => write!(f, "{} an IO error occurred: {}", msg, err),
            CompressionError::DateError(msg, err) => write!(f, "{} a date error occurred: {}", msg, err),
            CompressionError::CompressionFailed(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for CompressionError {}
