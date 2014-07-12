#![crate_type = "lib"]
#![crate_name = "leveldb"]
#![feature(globs,phase)]
#![phase(syntax, link)] extern crate log;

extern crate serialize;
extern crate libc;

pub use options = database::options;
pub use error = database::error;
pub use iterator = database::iterator;

mod cbits;
pub mod database;
