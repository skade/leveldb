use utils::{open_database,tmpdir,db_put_simple};
use leveldb::iterator::Iterable;
use leveldb::options::{ReadOptions};

#[test]
fn test_iterator() {
  let tmp = tmpdir("testdbs");
  let database = &mut open_database(tmp.path().join("iter"), true);
  db_put_simple(database, 1, &[1]);
  db_put_simple(database, 2, &[2]);

  let read_opts = ReadOptions::new();
  let mut iter = database.iter(read_opts);

  let entry = iter.next();
  assert!(entry.is_some());
  assert_eq!(entry.unwrap(), (1, vec![1]));
  let entry2 = iter.next();
  assert!(entry2.is_some());
  assert_eq!(entry2.unwrap(), (2, vec![2]));
  assert!(iter.next().is_none());
}

#[test]
fn test_iterator_last() {
  let tmp = tmpdir("testdbs");
  let database = &mut open_database(tmp.path().join("iter"), true);
  db_put_simple(database, 1, &[1]);
  db_put_simple(database, 2, &[2]);

  let read_opts = ReadOptions::new();
  let iter = database.iter(read_opts);

  assert!(iter.last().is_some());
}

#[test]
fn test_key_iterator() {
  let tmp = tmpdir("testdbs");
  let database = &mut open_database(tmp.path().join("iter"), true);
  db_put_simple(database, 1, &[1]);
  db_put_simple(database, 2, &[2]);

  let iterable: &mut Iterable<i32> = database;

  let read_opts = ReadOptions::new();
  let mut iter = iterable.keys_iter(read_opts);
  let value = iter.next().unwrap();
  assert_eq!(value, 1);
}

#[test]
fn test_value_iterator() {
  let tmp = tmpdir("testdbs");
  let database = &mut open_database(tmp.path().join("iter"), true);
  db_put_simple(database, 1, &[1]);
  db_put_simple(database, 2, &[2]);

  let iterable: &mut Iterable<i32> = database;

  let read_opts = ReadOptions::new();
  let mut iter = iterable.value_iter(read_opts);
  let value = iter.next().unwrap();
  assert_eq!(value, vec![1]);
}
