use utils::{tmpdir};
use leveldb::database::{Database};
use leveldb::options::{Options};
use leveldb::database::cache::{Cache};

#[test]
fn test_open_database_with_cache() {
  let mut opts = Options::new();
  opts.create_if_missing = true;
  opts.cache = Some(Cache::new(20));
  let tmp = tmpdir("testdbs");
  let res: Result<Database<int>,_> = Database::open(tmp.path().join("create_if_missing"), opts);
  assert!(res.is_ok());
}
