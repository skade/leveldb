use leveldb::database::kv::KV;
use leveldb::options::{ReadOptions, WriteOptions};
use utils::{db_put_simple, open_database, tmpdir};

#[test]
fn test_write_to_database() {
    let tmp = tmpdir("write");
    let database = open_database(tmp.path(), true);
    let write_opts = WriteOptions::new();
    let result = database.put(write_opts, 1, &[1]);
    assert!(result.is_ok());
}

#[test]
fn test_delete_from_database() {
    let tmp = tmpdir("delete_simple");
    let database = &mut open_database(tmp.path(), true);
    db_put_simple(database, 1, &[1]);

    let write2 = WriteOptions::new();
    let res2 = database.delete(write2, 1);
    assert!(res2.is_ok());
}

#[test]
fn test_get_from_empty_database() {
    let tmp = tmpdir("get_simple");
    let database = &mut open_database(tmp.path(), true);
    let read_opts = ReadOptions::new();
    let res = database.get(read_opts, 1);
    match res {
        Ok(data) => assert!(data.is_none()),
        Err(_) => panic!("failed reading data"),
    }
}

#[test]
fn test_get_from_filled_database() {
    let tmp = tmpdir("get_filled");
    let database = &mut open_database(tmp.path(), true);
    db_put_simple(database, 1, &[1]);

    let read_opts = ReadOptions::new();
    let res = database.get(read_opts, 1);
    match res {
        Ok(data) => {
            assert!(data.is_some());
            let data = data.unwrap();
            assert_eq!(data, vec!(1));
        }
        Err(_) => panic!("failed reading data"),
    }
}
