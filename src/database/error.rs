//! The module defining custom leveldb error type.

use libc::{c_void,free};

/// A leveldb error, just containing the error string
/// provided by leveldb.
#[derive(Debug)]
pub struct Error {
  message: String
}

impl Error {
  /// create a new Error, using the String provided
  pub fn new(message: String) -> Error {
    Error { message: message }
  }

  /// create an error from a c-string buffer.
  pub fn new_from_i8(message: *const i8) -> Error {
     use std::str::from_utf8;
     use std::ffi::CStr;

     let err_string = unsafe { from_utf8( CStr::from_ptr(message).to_bytes()).unwrap().to_string() };
     unsafe { free(message as *mut c_void) };
     Error::new(err_string)
  }
}
