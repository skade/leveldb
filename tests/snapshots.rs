use utils::{open_database,tmpdir,db_put_simple};
use leveldb::snapshots::Snapshots;
use leveldb::options::{ReadOptions};

#[test]
fn test_snapshots() {
  let tmp = tmpdir("testdbs");
  let database = &mut open_database(tmp.path().join("iter"), true);
  db_put_simple(database, 1, &[1]);
  let snapshot = database.snapshot();
  db_put_simple(database, 2, &[2]);
  let mut read_opts = ReadOptions::new();
  let res = snapshot.get(read_opts, 2);
  assert!(res.is_ok());
  assert_eq!(None, res.unwrap());
}
