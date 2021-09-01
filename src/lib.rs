//! A rust library for single instance application.
//!
//! single-instance provides an API to check if there are any other running instances of the same process.
//!
//! ## Detail
//! On Windows, `SingleInstance` attempts to create a named mutex and checks if it already exists.
//! On POSIX platforms it creates or opens a file with a given path, then attempts to apply
//! an advisory lock on the opened file.
//!
//! ### Examples
//! ```rust
//! extern crate single_instance;
//!
//! use single_instance::SingleInstance;
//!
//! fn main() {
//!     let instance = SingleInstance::new("whatever").unwrap();
//!     assert!(instance.is_single());
//! }
//! ```

pub mod error;

#[cfg(unix)]
extern crate nix;
extern crate thiserror;
#[cfg(windows)]
extern crate widestring;
#[cfg(windows)]
extern crate winapi;

pub use self::inner::*;

#[cfg(windows)]
mod inner {
    use error::{Result, SingleInstanceError};
    use std::ptr;
    use widestring::WideCString;
    use winapi::shared::winerror::{ERROR_ALREADY_EXISTS, ERROR_INVALID_HANDLE};
    use winapi::um::errhandlingapi::GetLastError;
    use winapi::um::handleapi::CloseHandle;
    use winapi::um::synchapi::CreateMutexW;
    use winapi::um::winnt::HANDLE;

    /// A struct representing one running instance.
    pub struct SingleInstance {
        handle: Option<HANDLE>,
    }

    unsafe impl Send for SingleInstance {}
    unsafe impl Sync for SingleInstance {}

    impl SingleInstance {
        /// Returns a new SingleInstance object.
        pub fn new(name: &str) -> Result<Self> {
            let name = WideCString::from_str(name)?;
            unsafe {
                let handle = CreateMutexW(ptr::null_mut(), 0, name.as_ptr());
                let last_error = GetLastError();

                // https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-createmutexexw
                if handle.is_null() || handle == ERROR_INVALID_HANDLE as _ {
                    Err(SingleInstanceError::MutexError(last_error))
                } else if last_error == ERROR_ALREADY_EXISTS {
                    CloseHandle(handle);
                    Ok(SingleInstance { handle: None })
                } else {
                    Ok(SingleInstance {
                        handle: Some(handle),
                    })
                }
            }
        }

        /// Returns whether this instance is single.
        pub fn is_single(&self) -> bool {
            self.handle.is_some()
        }
    }

    impl Drop for SingleInstance {
        fn drop(&mut self) {
            if let Some(handle) = self.handle.take() {
                unsafe {
                    CloseHandle(handle);
                }
            }
        }
    }
}

#[cfg(unix)]
mod inner {
    use std::{fs, io};

    use nix::fcntl::{self, FcntlArg, OFlag};
    use nix::sys::stat::Mode;
    use nix::unistd;

    use error::Result;

    /// A struct representing one running instance.
    pub struct SingleInstance {
        name: String,
        handle: Option<nix::libc::c_int>,
    }

    impl SingleInstance {
        /// Returns a new SingleInstance object.
        pub fn new(name: &str) -> Result<Self> {
            let fd = fcntl::open(
                name,
                OFlag::O_RDWR | OFlag::O_CREAT,
                Mode::from_bits_truncate(0o600),
            )
            .map_err(|e| io::Error::from(e))?;

            let fl = nix::libc::flock {
                l_type: nix::libc::F_WRLCK as _,
                l_whence: nix::libc::SEEK_SET as _,
                l_start: 0,
                l_len: 0,
                l_pid: 0,
            };

            match fcntl::fcntl(fd, FcntlArg::F_SETLK(&fl)) {
                Ok(_) => Ok(SingleInstance {
                    name: name.to_owned(),
                    handle: Some(fd),
                }),
                Err(_) => {
                    let _ = unistd::close(fd);
                    Ok(SingleInstance {
                        name: name.to_owned(),
                        handle: None,
                    })
                }
            }
        }

        /// Returns whether this instance is single.
        pub fn is_single(&self) -> bool {
            self.handle.is_some()
        }
    }

    impl Drop for SingleInstance {
        fn drop(&mut self) {
            if let Some(handle) = self.handle.take() {
                let _ = unistd::close(handle);
                let _ = fs::remove_file(&self.name);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(windows)]
    #[test]
    fn test_single_instance_windows() {
        {
            let instance_a = SingleInstance::new("aa2d0258-ffe9-11e7-ba89-0ed5f89f718b").unwrap();
            assert!(instance_a.is_single());
            let instance_b = SingleInstance::new("aa2d0258-ffe9-11e7-ba89-0ed5f89f718b").unwrap();
            assert!(!instance_b.is_single());
        }
        let instance_c = SingleInstance::new("aa2d0258-ffe9-11e7-ba89-0ed5f89f718b").unwrap();
        assert!(instance_c.is_single());
    }

    // on *nix it works across processes
    #[cfg(unix)]
    #[test]
    fn test_single_instance_unix() {
        let instance = SingleInstance::new("aa2d0258-ffe9-11e7-ba89-0ed5f89f718b").unwrap();
        let is_child = std::env::var("_SI_CHILD").is_ok();
        let is_single = instance.is_single();

        if is_child {
            assert!(!is_single);
        } else {
            assert!(is_single);

            let exe = std::env::current_exe().unwrap();
            let status = std::process::Command::new(&exe)
                .env("_SI_CHILD", "1")
                .status()
                .unwrap();
            assert!(status.success());
        }
    }
}
