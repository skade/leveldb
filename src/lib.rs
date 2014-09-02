#![crate_type = "lib"]
#![crate_name = "leveldb"]
#![feature(globs)]
#![feature(unsafe_destructor)]

extern crate serialize;
extern crate libc;

pub use database::options as options;
pub use database::error as error;
pub use database::iterator as iterator;
pub use database::comparator as comparator;

pub mod cbits;
pub mod database;
