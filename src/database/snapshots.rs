//! leveldb snapshots
//!
//! Snapshots give you a reference to the database at a certain
//! point in time and won't change while you work with them.
use leveldb_sys::{leveldb_create_snapshot, leveldb_release_snapshot};
use leveldb_sys::{leveldb_snapshot_t, leveldb_t};

use database::key::Key;
use database::kv::KV;
use database::Database;

use database::error::Error;
use database::iterator::{Iterable, Iterator, KeyIterator, ValueIterator};
use database::options::ReadOptions;

use std::borrow::Borrow;

#[allow(missing_docs)]
struct RawSnapshot {
    db_ptr: *mut leveldb_t,
    ptr: *mut leveldb_snapshot_t,
}

impl Drop for RawSnapshot {
    fn drop(&mut self) {
        unsafe { leveldb_release_snapshot(self.db_ptr, self.ptr) };
    }
}

/// A database snapshot
///
/// Represents a database at a certain point in time,
/// and allows for all read operations (get and iteration).
pub struct Snapshot<'a, K: Key + 'a> {
    raw: RawSnapshot,
    database: &'a Database<K>,
}

/// Structs implementing the Snapshots trait can be
/// snapshotted.
pub trait Snapshots<K: Key> {
    /// Creates a snapshot and returns a struct
    /// representing it.
    fn snapshot<'a>(&'a self) -> Snapshot<'a, K>;
}

impl<K: Key> Snapshots<K> for Database<K> {
    fn snapshot<'a>(&'a self) -> Snapshot<'a, K> {
        let db_ptr = self.database.ptr;
        let snap = unsafe { leveldb_create_snapshot(db_ptr) };

        let raw = RawSnapshot {
            db_ptr: db_ptr,
            ptr: snap,
        };
        Snapshot {
            raw: raw,
            database: self,
        }
    }
}

impl<'a, K: Key> Snapshot<'a, K> {
    /// fetches a key from the database
    ///
    /// Inserts this snapshot into ReadOptions before reading
    pub fn get<BK: Borrow<K>>(
        &'a self,
        mut options: ReadOptions<'a, K>,
        key: BK,
    ) -> Result<Option<Vec<u8>>, Error> {
        options.snapshot = Some(self);
        self.database.get(options, key)
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn raw_ptr(&self) -> *mut leveldb_snapshot_t {
        self.raw.ptr
    }
}

impl<'a, K: Key + 'a> Iterable<'a, K> for Snapshot<'a, K> {
    fn iter(&'a self, mut options: ReadOptions<'a, K>) -> Iterator<K> {
        options.snapshot = Some(self);
        self.database.iter(options)
    }
    fn keys_iter(&'a self, mut options: ReadOptions<'a, K>) -> KeyIterator<K> {
        options.snapshot = Some(self);
        self.database.keys_iter(options)
    }
    fn value_iter(&'a self, mut options: ReadOptions<'a, K>) -> ValueIterator<K> {
        options.snapshot = Some(self);
        self.database.value_iter(options)
    }
}
