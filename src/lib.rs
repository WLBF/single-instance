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

    pub struct SingleInstance {
        handle: HANDLE,
        last_error: DWORD,
    }

    impl SingleInstance {
        pub fn new(name: &str) -> Result<Self, Error> {
            let name = CString::new(name)?;
            unsafe {
                let handle = CreateMutexA(ptr::null_mut(), 0, name.as_ptr());
                let last_error = GetLastError();
                Ok(Self { handle, last_error })
            }
        }

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

    pub struct SingleInstance {
        _file: File,
        is_single: bool,
    }

    impl SingleInstance {
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

        pub fn is_single(&self) -> bool {
            self.is_single
        }
    }
}
