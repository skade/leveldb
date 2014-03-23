#[crate_type = "lib"];
#[crate_id = "leveldb#0.0.1"];
#[feature(globs,phase)];
#[phase(syntax, link)] extern crate log;

extern crate serialize;

pub use options = database::options;
pub use error = database::error;
pub use iterator = database::iterator;

mod cbits;
pub mod database;
