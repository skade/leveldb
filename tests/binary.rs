#![feature(globs)]
extern crate key;
extern crate leveldb;
extern crate serialize;

use utils::{open_database,tmpdir};
use leveldb::database::{Database,Interface};
use leveldb::database::binary::*;
use leveldb::iterator::Iterable;
use leveldb::options::{Options,ReadOptions,WriteOptions};

mod utils;

fn db_put_simple(database: &mut Interface<Binary, int, Vec<u8>>, key: int, val: &[u8]) {
  let write_opts = WriteOptions::new();
  match database.put(write_opts, key, val.to_vec()) {
    Ok(_) => { () },
    Err(e) => { panic!("failed to write to database: {}", e) }
  }
}

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

#[test]
fn test_write_to_database() {
  let tmp = tmpdir("testdbs");
  let database: &mut Interface<Binary, int, Vec<u8>> = &mut open_database(tmp.path().join("write"), true);
  let write_opts = WriteOptions::new();
  let result = database.put(write_opts,
                            1,
                            [1].to_vec());
  assert!(result.is_ok());
}

#[test]
fn test_delete_from_database() {
  let tmp = tmpdir("testdbs");
  let database: &mut Interface<Binary, int, Vec<u8>> = &mut open_database(tmp.path().join("delete_simple"), true);
  db_put_simple(database, 1, &[1]);

  let write2 = WriteOptions::new();
  let res2 = database.delete(write2,
                             1);
  assert!(res2.is_ok());
}

#[test]
fn test_get_from_empty_database() {
  let tmp = tmpdir("testdbs");
  let database: &mut Interface<Binary, int, Vec<u8>> = &mut open_database(tmp.path().join("get_simple"), true);
  let read_opts = ReadOptions::new();
  let res = database.get(read_opts, 1);
  match res {
    Ok(data) => { assert!(data.is_none()) },
    Err(_) => { panic!("failed reading data") }
  }
}

#[test]
fn test_get_from_filled_database() {
  let tmp = tmpdir("testdbs");
  let database: &mut Interface<Binary, int, Vec<u8>> = &mut open_database(tmp.path().join("get_filled"), true);
  db_put_simple(database, 1, &[1]);

  let read_opts = ReadOptions::new();
  let res = database.get(read_opts,
                         1);
  match res {
    Ok(data) => {
      assert!(data.is_some())
      let data = data.unwrap();
      assert_eq!(data, vec!(1));
    },
    Err(_) => { panic!("failed reading data") }
  }
}

#[test]
fn test_iterator() {
  let tmp = tmpdir("testdbs");
  let database = &mut open_database(tmp.path().join("iter"), true);
  db_put_simple(database, 1, &[1]);
  db_put_simple(database, 2, &[2]);

  let read_opts = ReadOptions::new();
  let mut iter = database.iter(read_opts);
  assert!(iter.valid());
  let entry = iter.next();
  assert!(entry.is_some());
  assert_eq!(entry.unwrap(), (1, vec![1]));
  let entry2 = iter.next();
  assert!(entry2.is_some());
  assert_eq!(entry2.unwrap(), (2, vec![2]));
  assert!(iter.next().is_none());
}

