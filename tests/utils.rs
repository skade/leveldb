extern crate key;
extern crate leveldb;
extern crate serialize;

use leveldb::database::Database;
use leveldb::options::{Options};
use leveldb::comparator::Comparator;
use std::io::TempDir;
use key::Key;

pub fn open_database<K: Key, C: Comparator<K>>(path: Path, create_if_missing: bool) -> Database<C> {
  let mut opts = Options::new();
  opts.create_if_missing = create_if_missing;
  match Database::open(path, opts, None) {
    Ok(db) => { db },
    Err(e) => { panic!("failed to open database: {}", e) }
  }
}

pub fn tmpdir(name: &str) -> TempDir {
  TempDir::new(name)
           .unwrap()
}


