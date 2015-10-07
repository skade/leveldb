//! Compaction
use super::Database;
use super::key::Key;
use leveldb_sys::leveldb_compact_range;
use libc::{c_char, size_t};

pub trait Compaction<'a, K: Key + 'a> {
    fn compact(&self, start: &'a K, limit: &'a K);
}

impl<'a, K: Key + 'a> Compaction<'a, K> for Database<K> {
    fn compact(&self, start: &'a K, limit: &'a K) {
        unsafe {
            start.as_slice(|s| {
                limit.as_slice(|l| {
                    leveldb_compact_range(self.database.ptr,
                                          s.as_ptr() as *mut c_char,
                                          s.len() as size_t,
                                          l.as_ptr() as *mut c_char,
                                          l.len() as size_t);
                });
            });
        }
    }
}
