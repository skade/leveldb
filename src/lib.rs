//! A database access library for leveldb
//!
//! Usage:
//!
//! ```rust,ignore
//! extern crate tempdir;
//! extern crate leveldb;
//!
//! use tempdir::TempDir;
//! use leveldb::database::Database;
//! use leveldb::kv::KV;
//! use leveldb::options::{Options,WriteOptions,ReadOptions};
//!
//! let tempdir = TempDir::new("demo").unwrap();
//! let path = tempdir.path();
//! 
//! let mut options = Options::new();
//! options.create_if_missing = true;
//! let mut database = match Database::open(path, options) {
//!     Ok(db) => { db },
//!     Err(e) => { panic!("failed to open database: {:?}", e) }
//! };
//!
//! let write_opts = WriteOptions::new();
//! match database.put(write_opts, 1, &[1]) {
//!     Ok(_) => { () },
//!     Err(e) => { panic!("failed to write to database: {:?}", e) }
//! };
//!
//! let read_opts = ReadOptions::new();
//! let res = database.get(read_opts,
//!                        1);
//! match res {
//!   Ok(data) => {
//!     assert!(data.is_some());
//!     assert_eq!(data, Some(vec![1]));
//!   }
//!   Err(e) => { panic!("failed reading data: {:?}", e) }
//! }
//! ```
 
#![crate_type = "lib"]
#![crate_name = "leveldb"]
#![deny(warnings)]
#![deny(missing_docs)]

extern crate libc;

use cbits::leveldb::{leveldb_major_version,leveldb_minor_version};
pub use database::options as options;
pub use database::error as error;
pub use database::iterator as iterator;
pub use database::snapshots as snapshots;
pub use database::comparator as comparator;
pub use database::kv as kv;
pub use database::batch as batch;
pub use database::management as management;

#[allow(missing_docs)]
pub mod cbits;
pub mod database;

/// Library version information
///
/// Need a recent version of leveldb to be used.

pub trait Version {
    /// The major version.
    fn major() -> isize {
        unsafe { leveldb_major_version() as isize }
    }

    /// The minor version
    fn minor() -> isize {
        unsafe { leveldb_minor_version() as isize }
    }
}
