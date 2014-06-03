use cbits::leveldb::leveldb_free;

pub struct Error {
  message: String
}

impl Error {
  pub fn new(message: String) -> Error {
    Error { message: message }
  }
}

// I am  pretty sure this is a memory leak
//impl Drop for Error {
//  fn drop(&mut self) {
//    unsafe { leveldb_free(self.message) }
//  }
//}