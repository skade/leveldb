#![feature(globs)]

extern crate leveldb;
extern crate serialize;

pub mod utils {
  use leveldb::database::Database;
  use leveldb::options::{Options};
  use std::io::TempDir;

  pub fn open_database(path: Path, create_if_missing: bool) -> Database {
    let mut opts = Options::new();
    opts.create_if_missing(create_if_missing);
    match Database::open(path, opts) {
      Ok(db) => { db },
      Err(e) => { fail!("failed to open database: {}", e) }
    }
  }

  pub fn tmpdir(name: &str) -> TempDir {
    TempDir::new(name)
             .unwrap()
  }
}

#[cfg(test)]
mod comparator {
  use super::utils::{tmpdir};
  use leveldb::database::Database;
  use leveldb::database::binary::Interface;
  use leveldb::iterator::Iterable;
  use leveldb::options::{Options,ReadOptions,WriteOptions};
  use leveldb::comparator::*;
  
  struct ReverseComparator {
    name: &'static str,
  }
  
  impl Comparator for ReverseComparator {
    fn name(&self) -> *const u8 {
      self.name.as_ptr()
    }
  
    fn compare(&self, a: &[u8], b: &[u8]) -> Ordering {
      if a[0] < b[0] {
        Greater
      } else {
        Less
      }
    }
  }

  fn db_put_simple(database: &mut Database, key: &[u8], val: &[u8]) {
    let write_opts = WriteOptions::new();
    match database.put(write_opts, key, val) {
      Ok(_) => { () },
      Err(e) => { fail!("failed to write to database: {}", e) }
    }
  }
  
  #[test]
  fn test_comparator() {
    let comparator = box ReverseComparator { name: "test" };
    let mut opts = Options::new();
    opts.create_if_missing(true);
    opts.set_comparator(comparator);
    let tmp = tmpdir("testdbs");
    let database = &mut Database::open(tmp.path().join("reverse_comparator"), opts).unwrap();
    db_put_simple(database, &[1], &[1]);
    db_put_simple(database, &[2], &[2]);

    let read_opts = ReadOptions::new();
    let mut iter = database.iter(read_opts);

    assert!(iter.valid());
    assert_eq!(vec![2], iter.next().unwrap().value())
  }
}
