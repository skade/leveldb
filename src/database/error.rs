
pub struct Error {
  message: ~str
}

impl Error {
  pub fn new(message: ~str) -> Error {
    Error { message: message }
  }
}

// I am  pretty sure this is a memory leak
//impl Drop for Error {
//  fn drop(&mut self) {
//    unsafe { leveldb_free(self.message) }
//  }
//}