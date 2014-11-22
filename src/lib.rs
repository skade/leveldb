//! A database access library for leveldb
//!
//! Usage:
//!
//! ```rust
//!  use std::io::TempDir;
//!  use leveldb::database::Database;
//!  use leveldb::options::{Options,WriteOptions,ReadOptions};
//!
//!  let tempdir = TempDir::new("demo").unwrap();
//!  let path = tempdir.path().join("simple");
//!  
//!  let mut options = Options::new();
//!  options.create_if_missing = true;
//!  let mut database = match Database::open(path, options) {
//!      Ok(db) => { db },
//!      Err(e) => { panic!("failed to open database: {}", e) }
//!  };
//!
//!  let write_opts = WriteOptions::new();
//!  match database.put(write_opts, 1, &[1]) {
//!      Ok(_) => { () },
//!      Err(e) => { panic!("failed to write to database: {}", e) }
//!  };
//!
//!  let read_opts = ReadOptions::new();
//!  let res = database.get(read_opts,
//!                         1);
//!  match res {
//!    Ok(data) => {
//!      assert!(data.is_some());
//!      assert_eq!(data, Some(vec![1]));
//!    }
//!    Err(_) => { panic!("failed reading data") }
//!  }
//! ```
 
#![crate_type = "lib"]
#![crate_name = "leveldb"]
#![feature(globs)]
#![deny(warnings)]

extern crate serialize;
extern crate libc;

pub use database::options as options;
pub use database::error as error;
pub use database::iterator as iterator;
pub use database::comparator as comparator;

#[allow(missing_docs)]
pub mod cbits;
pub mod database;
