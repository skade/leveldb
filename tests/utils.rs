extern crate key;
extern crate leveldb;
extern crate serialize;

use leveldb::database::Database;
use leveldb::options::{Options,WriteOptions};
use leveldb::comparator::Comparator;
use std::io::TempDir;
use key::Key;

pub fn open_database<K: Key, C: Comparator<K>>(path: Path, create_if_missing: bool) -> Database<K, C> {
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

pub fn db_put_simple<K: Key, C: Comparator<K>>(database: &mut Database<K, C>, key: K, val: &[u8]) {
  let write_opts = WriteOptions::new();
  match database.put(write_opts, key, val) {
    Ok(_) => { () },
    Err(e) => { panic!("failed to write to database: {}", e) }
  }
}

