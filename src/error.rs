use std::{ffi::NulError, fmt};

use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum EzTransError {
    #[error("Failed to load library: {0}")]
    LibraryLoadError(String),
    #[error("Failed to get symbol: {0}")]
    SymbolLoadError(String),
    #[error("Failed to initialize")]
    InitializationError,
    #[error("Failed to translate")]
    TranslationError(TransErr),
    #[error("Failed to terminate")]
    TerminationError,
    #[error("DLL path not set")]
    DllPathNotSet,
    #[error("Invalid string: {0}")]
    InvalidString(NulError),
    #[error("OnceLock error: {0}")]
    OnceLockError(String),
    #[error("Shared Memory Error: {0}")]
    SharedMemoryError(String),
    #[error("{0}")]
    Utf16Error(String),
}

#[derive(Error, Debug, Clone)]
pub enum TransErr {
    ///TRANSLATE_MMNTW or MMNT returned a null pointer
    NullPointer,
    ///Translation failed
    Failed,
    ///EUC-KR decoding failed
    EucKrDecodeFailed,
}
impl fmt::Display for TransErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransErr::NullPointer => write!(f, "TRANSLATE func returned a null pointer"),
            TransErr::Failed => write!(f, "Translation failed"),
            TransErr::EucKrDecodeFailed => write!(f, "EUC-KR decoding failed"),
        }
    }
}
