#![feature(globs)]
extern crate key;
extern crate leveldb;
extern crate serialize;

mod utils;

#[cfg(test)]
mod json_tests {
  use super::utils::{open_database,tmpdir};
  use leveldb::database::Interface;
  use leveldb::database::json::*;
  use leveldb::options::{ReadOptions,WriteOptions};
  use serialize::{Encodable};

  #[deriving(Encodable,Decodable)]
  struct ToEncode {
    test: String,
  }

  #[test]
  fn test_write_to_database() {
    let tmp = tmpdir("testdbs");
    let database: &mut Interface<JSON, int, ToEncode> = &mut open_database(tmp.path().join("json_put"), true);
    let write_opts = WriteOptions::new();
    let key = 1;
    let val = ToEncode { test: "string2".to_string() };
    let result = database.put(write_opts,
                              key,
                              val);
    assert!(result.is_ok());
  }

  #[test]
  fn test_read_from_database() {
    let tmp = tmpdir("testdbs");
    let database: &mut Interface<JSON,int,ToEncode> = &mut open_database(tmp.path().join("json_read"), true);
    let write_opts = WriteOptions::new();
    let key = 1;
    let val = ToEncode { test: "string2".to_string() };
    let result = database.put(write_opts,
                              key,
                              val);
    assert!(result.is_ok());
    let read_opts = ReadOptions::new();
    let read = database.get(read_opts,
                            key);
    match read {
      Ok(data) => {
        assert!(data.is_some())
        let decoded_object : ToEncode = data.unwrap();
        assert_eq!(decoded_object.test, "string2".to_string() );
      },
      Err(_) => { panic!("failed reading data") }
    }
  }

}
