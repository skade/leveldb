//! leveldb iterators
//!
//! Iteration is one of the most important parts of leveldb. This module provides
//! Iterators to iterate over key, values and pairs of both.
use leveldb_sys::{leveldb_iterator_t, leveldb_iter_seek_to_first, leveldb_iter_destroy,
                  leveldb_iter_seek_to_last, leveldb_create_iterator, leveldb_iter_valid,
                  leveldb_iter_next, leveldb_iter_key, leveldb_iter_value,
                  leveldb_readoptions_destroy, leveldb_iter_seek};
use libc::{size_t, c_char};
use std::iter;
use super::Database;
use super::options::{ReadOptions, c_readoptions};
use super::key::{Key, from_u8};
use std::slice::from_raw_parts;

#[allow(missing_docs)]
struct RawIterator {
    ptr: *mut leveldb_iterator_t,
}

#[allow(missing_docs)]
impl Drop for RawIterator {
    fn drop(&mut self) {
        unsafe { leveldb_iter_destroy(self.ptr) }
    }
}

/// An iterator over the leveldb keyspace.
///
/// Returns key and value as a tuple.
pub struct Iterator<'a, K: Key + 'a> {
    start: bool,
    // Iterator accesses the Database through a leveldb_iter_t pointer
    // but needs to hold the reference for lifetime tracking
    #[allow(dead_code)]
    database: &'a Database<K>,
    iter: RawIterator,
    from: Option<&'a K>,
    to: Option<&'a K>,
}

/// An iterator over the leveldb keyspace.
///
/// Returns just the keys.
pub struct KeyIterator<'a, K: Key + 'a> {
    start: bool,
    // Iterator accesses the Database through a leveldb_iter_t pointer
    // but needs to hold the reference for lifetime tracking
    #[allow(dead_code)]
    database: &'a Database<K>,
    iter: RawIterator,
    from: Option<&'a K>,
    to: Option<&'a K>,
}

/// An iterator over the leveldb keyspace.
///
/// Returns just the value.
pub struct ValueIterator<'a, K: Key + 'a> {
    start: bool,
    // Iterator accesses the Database through a leveldb_iter_t pointer
    // but needs to hold the reference for lifetime tracking
    #[allow(dead_code)]
    database: &'a Database<K>,
    iter: RawIterator,
    from: Option<&'a K>,
    to: Option<&'a K>,
}


/// A trait to allow access to the three main iteration styles of leveldb.
pub trait Iterable<'a, K: Key + 'a> {
    /// Return an Iterator iterating over (Key,Value) pairs
    fn iter(&'a self, options: ReadOptions<'a, K>) -> Iterator<K>;
    /// Returns an Iterator iterating over Keys only.
    fn keys_iter(&'a self, options: ReadOptions<'a, K>) -> KeyIterator<K>;
    /// Returns an Iterator iterating over Values only.
    fn value_iter(&'a self, options: ReadOptions<'a, K>) -> ValueIterator<K>;
}

impl<'a, K: Key + 'a> Iterable<'a, K> for Database<K> {
    fn iter(&'a self, options: ReadOptions<'a, K>) -> Iterator<K> {
        Iterator::new(self, options)
    }

    fn keys_iter(&'a self, options: ReadOptions<'a, K>) -> KeyIterator<K> {
        KeyIterator::new(self, options)
    }

    fn value_iter(&'a self, options: ReadOptions<'a, K>) -> ValueIterator<K> {
        ValueIterator::new(self, options)
    }
}

#[allow(missing_docs)]
pub trait LevelDBIterator<'a, K: Key> {
    #[inline]
    fn raw_iterator(&self) -> *mut leveldb_iterator_t;

    #[inline]
    fn start(&self) -> bool;

    #[inline]
    fn started(&mut self);

    fn from(mut self, key: &'a K) -> Self;
    fn to(mut self, key: &'a K) -> Self;

    fn from_key(&self) -> Option<&K>;
    fn to_key(&self) -> Option<&K>;

    fn valid(&self) -> bool {
        unsafe { leveldb_iter_valid(self.raw_iterator()) != 0 }
    }

    fn advance(&mut self) -> bool {
        unsafe {
            if !self.start() {

                leveldb_iter_next(self.raw_iterator());
            } else {
                if let Some(k) = self.from_key() {
                    self.seek(k)
                }
                self.started();
            }
        }
        self.valid()
    }

    fn key(&self) -> K {
        unsafe {
            let length: size_t = 0;
            let value = leveldb_iter_key(self.raw_iterator(), &length) as *const u8;
            from_u8(from_raw_parts(value, length as usize))
        }
    }

    fn value(&self) -> Vec<u8> {
        unsafe {
            let length: size_t = 0;
            let value = leveldb_iter_value(self.raw_iterator(), &length) as *const u8;
            from_raw_parts(value, length as usize).to_vec()
        }
    }

    fn seek_to_first(&self) {
        unsafe { leveldb_iter_seek_to_first(self.raw_iterator()) }
    }

    fn seek_to_last(&self) {
        if let Some(k) = self.to_key() {
            self.seek(k);
        } else {
            unsafe {
                leveldb_iter_seek_to_last(self.raw_iterator());
            }
        }
    }

    fn seek(&self, key: &K) {
        unsafe {
            key.as_slice(|k| {
                leveldb_iter_seek(self.raw_iterator(),
                                  k.as_ptr() as *mut c_char,
                                  k.len() as size_t);
            })
        }
    }
}


impl<'a, K: Key> Iterator<'a, K> {
    fn new(database: &'a Database<K>, options: ReadOptions<'a, K>) -> Iterator<'a, K> {
        unsafe {
            let c_readoptions = c_readoptions(&options);
            let ptr = leveldb_create_iterator(database.database.ptr, c_readoptions);
            leveldb_readoptions_destroy(c_readoptions);
            leveldb_iter_seek_to_first(ptr);
            Iterator {
                start: true,
                iter: RawIterator { ptr: ptr },
                database: database,
                from: None,
                to: None,
            }
        }
    }

    /// return the last element of the iterator
    pub fn last(self) -> Option<(K, Vec<u8>)> {
        self.seek_to_last();
        Some((self.key(), self.value()))
    }
}

impl<'a, K: Key> LevelDBIterator<'a, K> for Iterator<'a,K> {
    #[inline]
    fn raw_iterator(&self) -> *mut leveldb_iterator_t {
        self.iter.ptr
    }

    #[inline]
    fn start(&self) -> bool {
        self.start
    }

    #[inline]
    fn started(&mut self) {
        self.start = false
    }

    fn from(mut self, key: &'a K) -> Self {
        self.from = Some(key);
        self
    }

    fn to(mut self, key: &'a K) -> Self {
        self.to = Some(key);
        self
    }

    fn from_key(&self) -> Option<&K> {
        self.from
    }

    fn to_key(&self) -> Option<&K> {
        self.to
    }
}

impl<'a,K: Key> KeyIterator<'a,K> {
    fn new(database: &'a Database<K>, options: ReadOptions<'a, K>) -> KeyIterator<'a, K> {
        unsafe {
            let c_readoptions = c_readoptions(&options);
            let ptr = leveldb_create_iterator(database.database.ptr, c_readoptions);
            leveldb_readoptions_destroy(c_readoptions);
            leveldb_iter_seek_to_first(ptr);
            KeyIterator {
                start: true,
                iter: RawIterator { ptr: ptr },
                database: database,
                from: None,
                to: None,
            }
        }
    }

    /// return the last element of the iterator
    pub fn last(self) -> Option<K> {
        self.seek_to_last();
        Some(self.key())
    }
}

impl<'a,K: Key> LevelDBIterator<'a, K> for KeyIterator<'a,K> {
    #[inline]
    fn raw_iterator(&self) -> *mut leveldb_iterator_t {
        self.iter.ptr
    }

    #[inline]
    fn start(&self) -> bool {
        self.start
    }

    #[inline]
    fn started(&mut self) {
        self.start = false
    }

    fn from(mut self, key: &'a K) -> Self {
        self.from = Some(key);
        self
    }

    fn to(mut self, key: &'a K) -> Self {
        self.to = Some(key);
        self
    }

    fn from_key(&self) -> Option<&K> {
        self.from
    }

    fn to_key(&self) -> Option<&K> {
        self.to
    }
}

impl<'a,K: Key> ValueIterator<'a,K> {
    fn new(database: &'a Database<K>, options: ReadOptions<'a, K>) -> ValueIterator<'a, K> {
        unsafe {
            let c_readoptions = c_readoptions(&options);
            let ptr = leveldb_create_iterator(database.database.ptr, c_readoptions);
            leveldb_readoptions_destroy(c_readoptions);
            leveldb_iter_seek_to_first(ptr);
            ValueIterator {
                start: true,
                iter: RawIterator { ptr: ptr },
                database: database,
                from: None,
                to: None,
            }
        }
    }

    /// return the last element of the iterator
    pub fn last(self) -> Option<Vec<u8>> {
        self.seek_to_last();
        Some(self.value())
    }
}

impl<'a,K: Key> LevelDBIterator<'a, K> for ValueIterator<'a,K> {
    #[inline]
    fn raw_iterator(&self) -> *mut leveldb_iterator_t {
        self.iter.ptr
    }

    #[inline]
    fn start(&self) -> bool {
        self.start
    }

    #[inline]
    fn started(&mut self) {
        self.start = false
    }

    fn from(mut self, key: &'a K) -> Self {
        self.from = Some(key);
        self
    }

    fn to(mut self, key: &'a K) -> Self {
        self.to = Some(key);
        self
    }

    fn from_key(&self) -> Option<&K> {
        self.from
    }

    fn to_key(&self) -> Option<&K> {
        self.to
    }
}

impl<'a,K: Key> iter::Iterator for Iterator<'a,K> {
  type Item = (K,Vec<u8>);

    fn next(&mut self) -> Option<(K, Vec<u8>)> {
        if self.advance() {
            Some((self.key(), self.value()))
        } else {
            None
        }
    }
}

impl<'a, K: Key> iter::Iterator for KeyIterator<'a,K> {
  type Item = K;

    fn next(&mut self) -> Option<K> {
        if self.advance() {
            Some(self.key())
        } else {
            None
        }
    }
}

impl<'a, K: Key> iter::Iterator for ValueIterator<'a,K> {
  type Item = Vec<u8>;

    fn next(&mut self) -> Option<Vec<u8>> {
        if self.advance() {
            Some(self.value())
        } else {
            None
        }
    }
}
