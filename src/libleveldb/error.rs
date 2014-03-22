
pub struct Error {
  message: ~str
}

// I am  pretty sure this is a memory leak
//impl Drop for Error {
//  fn drop(&mut self) {
//    unsafe { leveldb_free(self.message) }
//  }
//}