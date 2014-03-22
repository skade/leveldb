extern crate leveldb;

#[cfg(test)]
mod tests {
  use leveldb::{Options,ReadOptions,WriteOptions,Database};

  #[test]
  fn test_create_options() {
    Options::new();
  }

  #[test]
  fn test_open_database() {
    let mut opts = Options::new();
    opts.create_if_missing(true);
    let res = Database::open(~"testdbs/create_if_missing", opts);
    assert!(res.is_ok());
  }

  #[test]
  fn test_open_non_existant_database_without_create() {
    let mut opts = Options::new();
    opts.create_if_missing(false);
    let res = Database::open(~"testdbs/missing", opts);
    assert!(res.is_err());
  }

  #[test]
  fn test_write_to_database() {
    let mut opts = Options::new();
    opts.create_if_missing(true);
    let mut database = match Database::open(~"testdbs/put_simple", opts) {
      Ok(db) => { db },
      Err(_) => { fail!("failed to open database") }
    };
    let write_opts = WriteOptions::new();
    let result = database.put(write_opts,
                              &[1],
                              &[1]);
    assert!(result.is_ok());
  }

  #[test]
  fn test_delete_from_database() {
    let mut opts = Options::new();
    opts.create_if_missing(true);
    let mut database = match Database::open(~"testdbs/delete_simple", opts) {
      Ok(db) => { db },
      Err(_) => { fail!("failed to open database") }
    };
    let write_opts = WriteOptions::new();
    let result = database.put(write_opts,
                              &[1],
                              &[1]);
    assert!(result.is_ok());
    let write2 = WriteOptions::new();
    let res2 = database.delete(write2,
                               &[1]);
    assert!(res2.is_ok());
  }

  #[test]
  fn test_get_from_empty_database() {
    let mut opts = Options::new();
    opts.create_if_missing(true);
    let mut database = match Database::open(~"testdbs/get_simple", opts) {
      Ok(db) => { db },
      Err(_) => { fail!("failed to open database") }
    };
    let read_opts = ReadOptions::new();
    let res = database.get(read_opts, [1,2,3]);
    match res {
      Ok(data) => { assert!(data.is_none()) },
      Err(_) => { fail!("failed reading data") }
    }
  }

  #[test]
  fn test_get_from_filled_database() {
    let mut opts = Options::new();
    opts.create_if_missing(true);
    let mut database = match Database::open(~"testdbs/get_filled", opts) {
      Ok(db) => { db },
      Err(_) => { fail!("failed to open database") }
    };
    let write_opts = WriteOptions::new();
    let result = database.put(write_opts,
                              &[1],
                              &[1]);
    assert!(result.is_ok());
    let read_opts = ReadOptions::new();
    let res = database.get(read_opts,
                            &[1]);
    match res {
      Ok(data) => {
        assert!(data.is_some())
        let data = data.unwrap();
        assert_eq!(data, ~[1]);
      },
      Err(_) => { fail!("failed reading data") }
    }
  }
}
