extern crate key;
extern crate leveldb;
extern crate serialize;

pub mod utils {
  use std::io::TempDir;

  pub fn tmpdir(name: &str) -> TempDir {
    TempDir::new(name)
             .unwrap()
  }
}

#[cfg(test)]
mod comparator {
  use key::Key;
  use super::utils::{tmpdir};
  use leveldb::database::{Database,Interface};
  use leveldb::database::binary::Binary;
  use leveldb::iterator::Iterable;
  use leveldb::options::{Options,ReadOptions,WriteOptions};
  use leveldb::comparator::Comparator;
  
  struct ReverseComparator<K>;

  impl<K: Key> Comparator<K> for ReverseComparator<K> {
    fn name(&self) -> *const u8 {
      "reverse".as_ptr()
    }
  
    fn compare(&self, a: &K, b: &K) -> Ordering {
      b.compare(a)
    }

  }

  fn db_put_simple(database: &mut Interface<Binary, int, Vec<u8>>, key: int, val: &[u8]) {
    let write_opts = WriteOptions::new();
    match database.put(write_opts, key, val.to_vec()) {
      Ok(_) => { () },
      Err(e) => { panic!("failed to write to database: {}", e) }
    }
  }
  
  #[test]
  fn test_comparator() {
    let comparator: ReverseComparator<int> = ReverseComparator::<int>;
    let mut opts = Options::new();
    opts.create_if_missing = true;
    let tmp = tmpdir("testdbs");
    let database = &mut Database::open(tmp.path().join("reverse_comparator"), opts, Some(comparator)).unwrap();
    db_put_simple(database, 1, &[1]);
    db_put_simple(database, 2, &[2]);

    let read_opts = ReadOptions::new();
    let mut iter = database.iter(read_opts);

    assert!(iter.valid());
    assert_eq!((2, vec![2]), iter.next().unwrap())
  }
}
