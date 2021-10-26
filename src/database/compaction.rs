//! Compaction
use super::key::Key;
use super::Database;
use leveldb_sys::leveldb_compact_range;
use libc::{c_char, size_t};

pub trait Compaction<'a, K: Key<'a> + 'a> {
    fn compact(&self, start: &'a K, limit: &'a K);
}

impl<'a, K: Key<'a> + 'a> Compaction<'a, K> for Database<'a, K> {
    fn compact(&self, start: &'a K, limit: &'a K) {
        unsafe {
            let s = start.as_ref();
            let l = limit.as_ref();
            leveldb_compact_range(
                self.database.ptr,
                s.as_ptr() as *mut c_char,
                s.len() as size_t,
                l.as_ptr() as *mut c_char,
                l.len() as size_t,
            );
        }
    }
}
