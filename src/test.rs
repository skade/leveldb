#![feature(globs,phase)]

extern crate leveldb;
extern crate serialize;

pub mod utils {
  use leveldb::database::Database;
  use leveldb::options::{Options};

  pub fn open_database(name: &str, create_if_missing: bool) -> Database {
    let mut opts = Options::new();
    opts.create_if_missing(create_if_missing);
    match Database::open(name, opts) {
      Ok(db) => { db },
      Err(_) => { fail!("failed to open database") }
    }
  }
}

#[cfg(test)]
mod binary_tests {
  use super::utils::{open_database};
  use leveldb::database::Database;
  use leveldb::database::binary::Interface;
  use leveldb::iterator::Iterable;
  use leveldb::options::{Options,ReadOptions,WriteOptions};

  fn db_put_simple(database: &mut Database, key: &[u8], val: &[u8]) {
    let write_opts = WriteOptions::new();
    match database.put(write_opts, key, val) {
      Ok(_) => { () },
      Err(_) => { fail!("failed to write to database") }
    }
  }

  #[test]
  fn test_create_options() {
    Options::new();
  }

  #[test]
  fn test_open_database() {
    let mut opts = Options::new();
    opts.create_if_missing(true);
    let res = Database::open("testdbs/create_if_missing", opts);
    assert!(res.is_ok());
  }

  #[test]
  fn test_open_non_existant_database_without_create() {
    let mut opts = Options::new();
    opts.create_if_missing(false);
    let res = Database::open("testdbs/missing", opts);
    assert!(res.is_err());
  }

  #[test]
  fn test_write_to_database() {
    let mut database = open_database("testdbs/put_simple", true);
    let write_opts = WriteOptions::new();
    let result = database.put(write_opts,
                              &[1],
                              &[1]);
    assert!(result.is_ok());
  }

  #[test]
  fn test_delete_from_database() {
    let mut database = open_database("testdbs/delete_simple", true);
    db_put_simple(&mut database, &[1], &[1]);

    let write2 = WriteOptions::new();
    let res2 = database.delete(write2,
                               &[1]);
    assert!(res2.is_ok());
  }

  #[test]
  fn test_get_from_empty_database() {
    let mut database = open_database("testdbs/get_simple", true);
    let read_opts = ReadOptions::new();
    let res = database.get(read_opts, [1,2,3]);
    match res {
      Ok(data) => { assert!(data.is_none()) },
      Err(_) => { fail!("failed reading data") }
    }
  }

  #[test]
  fn test_get_from_filled_database() {
    let mut database = open_database("testdbs/get_filled", true);
    db_put_simple(&mut database, &[1], &[1]);

    let read_opts = ReadOptions::new();
    let res = database.get(read_opts,
                            &[1]);
    match res {
      Ok(data) => {
        assert!(data.is_some())
        let data = data.unwrap();
        assert_eq!(data, vec!(1));
      },
      Err(_) => { fail!("failed reading data") }
    }
  }

  #[test]
  fn test_iterator() {
    let mut database = &mut open_database("testdbs/iter", true);
    db_put_simple(database, &[1], &[1]);
    db_put_simple(database, &[2], &[2]);

    let read_opts = ReadOptions::new();
    let mut iter = database.iter(read_opts);
    assert!(iter.valid());
    assert!(iter.next().is_some());
    assert!(iter.next().is_some());
    assert!(iter.next().is_none());
  }
}

#[cfg(test)]
mod json_tests {
  use super::utils::{open_database};
  use leveldb::database::json::Interface;
  use leveldb::options::{ReadOptions,WriteOptions};
  use serialize::{Encodable,Decodable};
  use serialize::json;

  #[deriving(Encodable,Decodable)]
  struct ToEncode {
    test: String,
  }

  #[test]
  fn test_write_to_database() {
    let mut database = open_database("testdbs/json_put", true);
    let write_opts = WriteOptions::new();
    let key = ToEncode { test: "string".to_string() };
    let val = ToEncode { test: "string2".to_string() };
    let result = database.put(write_opts,
                              &key,
                              &val);
    assert!(result.is_ok());
  }

  #[test]
  fn test_read_from_database() {
    let mut database = open_database("testdbs/json_read", true);
    let write_opts = WriteOptions::new();
    let key = ToEncode { test: "string".to_string() };
    let val = ToEncode { test: "string2".to_string() };
    let result = database.put(write_opts,
                              &key,
                              &val);
    assert!(result.is_ok());
    let read_opts = ReadOptions::new();
    let read = database.get(read_opts,
                            &key);
    match read {
      Ok(data) => {
        assert!(data.is_some())
        let data = data.unwrap();
        assert!(data.is_object());
        let mut decoder = json::Decoder::new(data);
        let result = Decodable::decode(&mut decoder);
        assert!(result.is_ok())
        let decoded_object : ToEncode = result.unwrap();
        assert_eq!(decoded_object.test, "string2".to_string() );
      },
      Err(_) => { fail!("failed reading data") }
    }
  }

}
