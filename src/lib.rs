//! A rust library for single instance application.
//!
//! single-instance provides a single API to check if there are any other running instance.
//!
//! ## Usage
//! On Windows, init `SingleInstance` will create a *Mutex* named by given `&str`
//! then check error code by calling `GetLastError`. On Linux, init will create or open a file
//! which path is given `&str`, then call `flock` to apply an advisory lock on the open file.
//!
//! ### Examples
//! ```rust
//! extern crate single_instance;
//!
//! use std::thread;
//! use single_instance::SingleInstance;
//!
//! fn main() {
//!     let instance = SingleInstance::new("whatever").unwrap();
//!     assert!(instance.is_single());
//! }
//! ```


extern crate failure;
#[cfg(windows)]
extern crate winapi;
#[cfg(unix)]
extern crate libc;

pub use self::inner::*;

#[cfg(windows)]
mod inner {
    use std::ptr;
    use std::ffi::CString;
    use failure::Error;
    use winapi::um::winnt::HANDLE;
    use winapi::um::synchapi::CreateMutexA;
    use winapi::shared::minwindef::DWORD;
    use winapi::um::errhandlingapi::GetLastError;
    use winapi::shared::winerror::ERROR_ALREADY_EXISTS;
    use winapi::um::handleapi::CloseHandle;

    /// A struct representing one running instance.
    pub struct SingleInstance {
        handle: HANDLE,
        last_error: DWORD,
    }

    impl SingleInstance {
        /// Returns a new SingleInstance object.
        pub fn new(name: &str) -> Result<Self, Error> {
            let name = CString::new(name)?;
            unsafe {
                let handle = CreateMutexA(ptr::null_mut(), 0, name.as_ptr());
                let last_error = GetLastError();
                Ok(Self { handle, last_error })
            }
        }

        /// Returns whether this instance is single.
        pub fn is_single(&self) -> bool {
            self.last_error != ERROR_ALREADY_EXISTS
        }
    }

    impl Drop for SingleInstance {
        fn drop(&mut self) {
            unsafe {
                CloseHandle(self.handle);
            }
        }
    }
}

#[cfg(unix)]
mod inner {
    use std::fs::File;
    use std::path::Path;
    use std::os::unix::io::AsRawFd;
    use failure::Error;
    use libc::{flock, LOCK_EX, LOCK_NB, EWOULDBLOCK, __errno_location};

    /// A struct representing one running instance.
    pub struct SingleInstance {
        _file: File,
        is_single: bool,
    }

    impl SingleInstance {
        /// Returns a new SingleInstance object.
        pub fn new(name: &str) -> Result<Self, Error> {
            let path = Path::new(name);
            let file = if path.exists() {
                File::open(path)?
            } else {
                File::create(path)?
            };
            unsafe {
                let rc = flock(file.as_raw_fd(), LOCK_EX | LOCK_NB);
                let is_single = rc == 0 || EWOULDBLOCK != *__errno_location();
                Ok(Self { _file: file, is_single })
            }
        }

        /// Returns whether this instance is single.
        pub fn is_single(&self) -> bool {
            self.is_single
        }
    }
}

#[test]
fn test_single_instance() {
    {
        let instance_a = SingleInstance::new("aa2d0258-ffe9-11e7-ba89-0ed5f89f718b").unwrap();
        assert!(instance_a.is_single());
        let instance_b = SingleInstance::new("aa2d0258-ffe9-11e7-ba89-0ed5f89f718b").unwrap();
        assert!(!instance_b.is_single());
    }
    let instance_c = SingleInstance::new("aa2d0258-ffe9-11e7-ba89-0ed5f89f718b").unwrap();
    assert!(instance_c.is_single());
}
