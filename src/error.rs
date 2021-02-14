use thiserror::Error;

#[derive(Error, Debug)]
pub enum SingleInstanceError {
    #[cfg(target_os = "linux")]
    #[error("new abstract addr error")]
    Nix(#[from] nix::Error),

    #[cfg(target_os = "macos")]
    #[error("file open or create error")]
    Io(#[from] std::io::Error),

    #[cfg(target_os = "windows")]
    #[error("wide string null error")]
    Nul(#[from] widestring::NulError<widestring::WideChar>),
}

pub type Result<T> = std::result::Result<T, SingleInstanceError>;