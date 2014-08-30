#![crate_type = "lib"]
#![crate_name = "leveldb"]
#![feature(globs,phase)]
#![phase(syntax, link)] extern crate log;

extern crate serialize;
extern crate libc;

pub use database::options as options;
pub use database::error as error;
pub use database::iterator as iterator;

mod cbits;
pub mod database;
