//! leveldb snapshots
//!
//! Snapshots give you a reference to the database at a certain
//! point in time and won't change while you work with them.
use cbits::leveldb::{leveldb_t, leveldb_snapshot_t};
use cbits::leveldb::{leveldb_release_snapshot,leveldb_create_snapshot};

use database::db_key::Key;
use database::Database;

use database::error::Error;
use database::options::ReadOptions;

#[allow(missing_docs)]
struct RawSnapshot {
  db_ptr: *mut leveldb_t,
  ptr: *mut leveldb_snapshot_t
}

impl Drop for RawSnapshot {
  fn drop(&mut self) {
    unsafe { leveldb_release_snapshot(self.db_ptr, self.ptr) };
  }
}

#[allow(missing_docs)]
pub struct Snapshot<'a, K: Key> {
  #[allow(dead_code)]
  raw: RawSnapshot,
  #[allow(dead_code)]
  database: &'a Database<K>
}

#[allow(missing_docs)]
pub trait Snapshots<K> {
  fn snapshot<'a>(&'a self) -> Snapshot<'a, K>;
}

impl<K: Key> Snapshots<K> for Database<K> {
  fn snapshot<'a>(&'a self) -> Snapshot<'a, K> {
    let db_ptr = self.database.ptr;
    let snap = unsafe { leveldb_create_snapshot(db_ptr) };

    let raw = RawSnapshot { db_ptr: db_ptr, ptr: snap };
    Snapshot { raw: raw, database: self }
  }
}

impl<'a, K: Key> Snapshot<'a, K> {
  #[allow(missing_docs)]
  pub fn get(&self,
             mut options: ReadOptions,
             key: K) -> Result<Option<Vec<u8>>, Error> {
    options.snapshot = Some(self.raw.ptr);
    self.database.get(options, key)
  }
}
