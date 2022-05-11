#[cfg(test)]
mod comparator {
  use libc::c_char;
  use leveldb::database::key::Key;
  use utils::{tmpdir, db_put_simple};
  use leveldb::database::{Database};
  use leveldb::iterator::Iterable;
  use leveldb::options::{Options,ReadOptions};
  use leveldb::comparator::{Comparator,OrdComparator};
  use std::cmp::Ordering;
  use std::marker::PhantomData;
  
  struct ReverseComparator<K> {
      marker: PhantomData<K>
  }

  impl<K: Key + Ord> Comparator for ReverseComparator<K> {
    type K = K;

    fn name(&self) -> *const c_char {
      "reverse".as_ptr() as *const c_char
    }
  
    fn compare(&self, a: &K, b: &K) -> Ordering {
      b.cmp(a)
    }
  }

  #[test]
  fn test_comparator() {
    let comparator: ReverseComparator<i32> = ReverseComparator { marker: PhantomData };
    let mut opts = Options::new();
    opts.create_if_missing = true;
    let tmp = tmpdir("reverse_comparator");
    let database = &mut Database::open_with_comparator(tmp.path(), opts, comparator).unwrap();
    db_put_simple(database, 1, &[1]);
    db_put_simple(database, 2, &[2]);

    let read_opts = ReadOptions::new();
    let mut iter = database.iter(read_opts);

    assert_eq!((2, vec![2]), iter.next().unwrap());
    assert_eq!((1, vec![1]), iter.next().unwrap());
  }

  #[test]
  fn test_ord_comparator() {
    let comparator: OrdComparator<i32> = OrdComparator::new("foo");
    let mut opts = Options::new();
    opts.create_if_missing = true;
    let tmp = tmpdir("ord_comparator");
    let database = &mut Database::open_with_comparator(tmp.path(), opts, comparator).unwrap();
    db_put_simple(database, 1, &[1]);
    db_put_simple(database, 2, &[2]);

    let read_opts = ReadOptions::new();
    let mut iter = database.iter(read_opts);

    assert_eq!((1, vec![1]), iter.next().unwrap());
    assert_eq!((2, vec![2]), iter.next().unwrap());
  }
}
