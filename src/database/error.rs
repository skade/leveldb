//! The module defining custom leveldb error type.

use leveldb_sys::leveldb_free;
use libc::c_char;
use libc::c_void;
use std;

/// A leveldb error, just containing the error string
/// provided by leveldb.
#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    /// create a new Error, using the String provided
    pub fn new(message: String) -> Error {
        Error { message: message }
    }

    /// create an error from a c-string buffer.
    ///
    /// This method is `unsafe` because the pointer must be valid and point to heap.
    /// The pointer will be passed to `free`!
    pub unsafe fn new_from_char(message: *const c_char) -> Error {
        use std::ffi::CStr;
        use std::str::from_utf8;

        let err_string = from_utf8(CStr::from_ptr(message).to_bytes())
            .unwrap()
            .to_string();
        leveldb_free(message as *mut c_void);
        Error::new(err_string)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "LevelDB error: {}", self.message)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
    fn cause(&self) -> Option<&std::error::Error> {
        None
    }
}
