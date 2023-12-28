use thiserror::Error;

#[derive(Error, Debug)]
pub enum SingleInstanceError {
    #[cfg(any(target_os = "linux", target_os="android"))]
    #[error("new abstract addr error")]
    Nix(#[from] nix::Error),

    #[cfg(target_os = "macos")]
    #[error("file open or create error")]
    Io(#[from] std::io::Error),

    #[cfg(target_os = "windows")]
    #[error("wide string null error")]
    Nul(#[from] widestring::NulError<widestring::WideChar>),

    #[cfg(target_os = "windows")]
    #[error("CreateMutex failed with error code {0}")]
    MutexError(u32),
}

pub type Result<T> = std::result::Result<T, SingleInstanceError>;