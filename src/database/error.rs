#[deriving(Show)]
pub struct Error {
  message: String
}

impl Error {
  pub fn new(message: String) -> Error {
    Error { message: message }
  }
}
