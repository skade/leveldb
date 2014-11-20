#![feature(globs)]
extern crate db_key;
extern crate leveldb;
extern crate serialize;

use utils::{tmpdir};
use leveldb::database::{Database};
use leveldb::options::{Options};

#[allow(dead_code)]
mod utils;

#[test]
fn test_create_options() {
  Options::new();
}

#[test]
fn test_open_database() {
  let mut opts = Options::new();
  opts.create_if_missing = true;
  let tmp = tmpdir("testdbs");
  let res = Database::open(tmp.path().join("create_if_missing"), opts, None);
  assert!(res.is_ok());
}

#[test]
fn test_open_non_existant_database_without_create() {
  let mut opts = Options::new();
  opts.create_if_missing = false;
  let tmp = tmpdir("testdbs");
  let res = Database::open(tmp.path().join("missing"), opts, None);
  assert!(res.is_err());
}
