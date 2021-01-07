//! Compaction
use super::Database;
use super::key::Key;
use leveldb_sys::leveldb_approximate_sizes;
use libc::{c_char, size_t};

pub trait Size<'a, K: Key + 'a> {
    fn approximate_size(&self, start: &'a K, stop: &'a K) -> u64;
}

impl<'a, K: Key + 'a> Size<'a, K> for Database<K> {
    fn approximate_size(&self, start: &'a K, stop: &'a K) -> u64 {
        // TODO: Find a way to get rid of these un-necessary copies, while keeping
        // the code simple. If I put the unsafe call inside the closures, the borrow
        // checker complains that size is borrowed inside the closures.
        let start = start.as_slice(|s| Vec::from(s));
        let stop = stop.as_slice(|s| Vec::from(s));

        let start_ptr = start.as_ptr() as *const c_char;
        let stop_ptr = stop.as_ptr() as *const c_char;

        let mut size: u64 = 0;
        unsafe {
            leveldb_approximate_sizes(
                self.database.ptr,
                1,
                &start_ptr as *const *const c_char,
                &start.len() as *const size_t,
                &stop_ptr as *const *const c_char,
                &stop.len() as *const size_t,
                &mut size as *mut u64);
        }
        size
    }
}
