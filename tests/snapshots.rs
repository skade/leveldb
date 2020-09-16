use leveldb::iterator::Iterable;
use leveldb::options::ReadOptions;
use leveldb::snapshots::Snapshots;
use utils::{db_put_simple, open_database, tmpdir};

#[test]
fn test_snapshots() {
    let tmp = tmpdir("snapshots");
    let database = &mut open_database(tmp.path(), true);
    db_put_simple(database, 1, &[1]);
    let snapshot = database.snapshot();
    db_put_simple(database, 2, &[2]);
    let read_opts = ReadOptions::new();
    let res = snapshot.get(read_opts, 2);
    assert!(res.is_ok());
    assert_eq!(None, res.unwrap());
}

#[test]
fn test_snapshot_iterator() {
    let tmp = tmpdir("snap_iterator");
    let database = &mut open_database(tmp.path(), true);
    db_put_simple(database, 1, &[1]);
    let snapshot = database.snapshot();
    db_put_simple(database, 2, &[2]);
    let read_opts = ReadOptions::new();
    let mut iter = snapshot.keys_iter(read_opts);
    let key = iter.next();
    assert_eq!(Some(1), key);
    let next = iter.next();
    assert_eq!(None, next);
}
