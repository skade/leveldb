//! The module defining custom leveldb error type.

use std::fmt;

/// A leveldb error, just containing the error string
/// provided by leveldb.
#[derive(Show)]
pub struct Error {
  message: String
}

impl Error {
  /// create a new Error, using the String provided
  pub fn new(message: String) -> Error {
    Error { message: message }
  }
}

impl fmt::String for Error {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    self.message.fmt(formatter)
  }
}
