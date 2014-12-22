use utils::{open_database,tmpdir,db_put_simple};
use leveldb::options::{ReadOptions,WriteOptions};

#[test]
fn test_write_to_database() {
  let tmp = tmpdir("testdbs");
  let mut database = open_database(tmp.path().join("write"), true);
  let write_opts = WriteOptions::new();
  let result = database.put(write_opts,
                            1,
                            &[1]);
  assert!(result.is_ok());
}

#[test]
fn test_delete_from_database() {
  let tmp = tmpdir("testdbs");
  let database = &mut open_database(tmp.path().join("delete_simple"), true);
  db_put_simple(database, 1, &[1]);

  let write2 = WriteOptions::new();
  let res2 = database.delete(write2,
                             1);
  assert!(res2.is_ok());
}

#[test]
fn test_get_from_empty_database() {
  let tmp = tmpdir("testdbs");
  let database = &mut open_database(tmp.path().join("get_simple"), true);
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
  let database = &mut open_database(tmp.path().join("get_filled"), true);
  db_put_simple(database, 1, &[1]);

  let read_opts = ReadOptions::new();
  let res = database.get(read_opts,
                         1);
  match res {
    Ok(data) => {
      assert!(data.is_some());
      let data = data.unwrap();
      assert_eq!(data, vec!(1));
    },
    Err(_) => { panic!("failed reading data") }
  }
}
