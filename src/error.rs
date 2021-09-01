use thiserror::Error;

#[derive(Error, Debug)]
pub enum SingleInstanceError {
    #[cfg(unix)]
    #[error("file open or create error")]
    Io(#[from] std::io::Error),

    #[cfg(windows)]
    #[error("wide string null error")]
    Nul(#[from] widestring::NulError<widestring::WideChar>),

    #[cfg(windows)]
    #[error("CreateMutex failed with error code {0}")]
    MutexError(u32),
}

pub type Result<T> = std::result::Result<T, SingleInstanceError>;
