//! leveldb iterators
//!
//! Iteration is one of the most important parts of leveldb. This module provides
//! Iterators to iterate over key, values and pairs of both.
use cbits::leveldb::{leveldb_iterator_t,leveldb_iter_seek_to_first,leveldb_iter_destroy,leveldb_iter_seek_to_last,
leveldb_create_iterator,leveldb_iter_valid,leveldb_iter_next,leveldb_iter_key,leveldb_iter_value,leveldb_readoptions_destroy, leveldb_iter_seek};
use libc::{size_t,c_char};
use std::iter;
use super::Database;
use super::options::{ReadOptions,c_readoptions};
use super::db_key::{Key,from_u8};

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
pub struct Iterator<'a, K: Key> {
  start: bool,
  // Iterator accesses the Database through a leveldb_iter_t pointer
  // but needs to hold the reference for lifetime tracking
  #[allow(dead_code)]
  database: &'a Database<K>,
  iter: RawIterator
}

/// An iterator over the leveldb keyspace.
///
/// Returns just the keys.
pub struct KeyIterator<'a, K: Key> {
  start: bool,
  // Iterator accesses the Database through a leveldb_iter_t pointer
  // but needs to hold the reference for lifetime tracking
  #[allow(dead_code)]
  database: &'a Database<K>,
  iter: RawIterator
}

/// An iterator over the leveldb keyspace.
///
/// Returns just the value.
pub struct ValueIterator<'a,K: Key> {
  start: bool,
  // Iterator accesses the Database through a leveldb_iter_t pointer
  // but needs to hold the reference for lifetime tracking
  #[allow(dead_code)]
  database: &'a Database<K>,
  iter: RawIterator
}


/// A trait to allow access to the three main iteration styles of leveldb.
pub trait Iterable<K: Key> {
  /// Return an Iterator iterating over (Key,Value) pairs
  fn iter(&self, options: ReadOptions) -> Iterator<K>;
  /// Returns an Iterator iterating over Keys only.
  fn keys_iter(&self, options: ReadOptions) -> KeyIterator<K>;
  /// Returns an Iterator iterating over Values only.
  fn value_iter(&self, options: ReadOptions) -> ValueIterator<K>;
}

impl<K: Key> Iterable<K> for Database<K> {
  fn iter(&self, options: ReadOptions) -> Iterator<K> {
    Iterator::new(self, options)
  }

  fn keys_iter(&self, options: ReadOptions) -> KeyIterator<K> {
    KeyIterator::new(self, options)
  }

  fn value_iter(&self, options: ReadOptions) -> ValueIterator<K> {
    ValueIterator::new(self, options)
  }
}

#[allow(missing_docs)]
pub trait LevelDBIterator<K: Key> {
  #[inline]
  fn raw_iterator(&self) -> *mut leveldb_iterator_t;
  
  #[inline]
  fn start(&self) -> bool;

  #[inline]
  fn started(&mut self);

  fn valid(&self) -> bool {
    unsafe { leveldb_iter_valid(self.raw_iterator()) != 0 }
  }

  fn advance(&mut self) -> bool {
    unsafe {
      if !self.start() {
        leveldb_iter_next(self.raw_iterator());
      } else {
        self.started();
      }
    }
    self.valid()
  }

  fn key(&self) -> K {
    unsafe {
      let length: size_t = 0;
      let value = leveldb_iter_key(self.raw_iterator(),
                                   &length) as *const u8;
      from_u8(Vec::from_raw_buf(value, length as uint).as_slice())
    }
  }

  fn value(&self) -> Vec<u8> {
    unsafe {
      let length: size_t = 0;
      let value = leveldb_iter_value(self.raw_iterator(),
                                     &length) as *const u8;
      Vec::from_raw_buf(value, length as uint)
    }
  }
}

#[allow(missing_docs)]
pub trait LevelDBIteratorExt<K: Key> : LevelDBIterator<K> {
  fn seek_to_first(&self) {
    unsafe { leveldb_iter_seek_to_first(self.raw_iterator()) }
  }
  
  fn seek_to_last(&self) {
    unsafe { leveldb_iter_seek_to_last(self.raw_iterator()); }
  }

  fn seek(&self, key: K) {
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
  fn new(database: &'a Database<K>,
         options: ReadOptions) -> Iterator<'a,K> {
    unsafe {
      let c_readoptions = c_readoptions(&options);
      let ptr = leveldb_create_iterator(database.database.ptr,
                                        c_readoptions);
      leveldb_readoptions_destroy(c_readoptions);
      leveldb_iter_seek_to_first(ptr);
      Iterator { start: true, iter: RawIterator { ptr: ptr }, database: database }
    }
  }

  /// return the last element of the iterator
  pub fn last(self) -> Option<(K,Vec<u8>)> {
    self.seek_to_last();
    Some((self.key(), self.value()))
  }
}

impl<'a, K: Key> LevelDBIterator<K> for Iterator<'a,K> {
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
}

impl<'a, K: Key> LevelDBIteratorExt<K> for Iterator<'a,K> { }

impl<'a,K: Key> KeyIterator<'a,K> {
  fn new(database: &'a Database<K>,
         options: ReadOptions) -> KeyIterator<K> {
    unsafe {
      let c_readoptions = c_readoptions(&options);
      let ptr = leveldb_create_iterator(database.database.ptr,
                                        c_readoptions);
      leveldb_readoptions_destroy(c_readoptions);
      leveldb_iter_seek_to_first(ptr);
      KeyIterator { start: true, iter: RawIterator { ptr: ptr }, database: database }
    }
  }

  /// return the last element of the iterator
  pub fn last(self) -> Option<K> {
    self.seek_to_last();
    Some(self.key())
  }
}

impl<'a,K: Key> LevelDBIterator<K> for KeyIterator<'a,K> {
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
}

impl<'a, K: Key> LevelDBIteratorExt<K> for KeyIterator<'a,K> { }

impl<'a,K: Key> ValueIterator<'a,K> {
  fn new(database: &'a Database<K>,
         options: ReadOptions) -> ValueIterator<K> {
    unsafe {
      let c_readoptions = c_readoptions(&options);
      let ptr = leveldb_create_iterator(database.database.ptr,
                                        c_readoptions);
      leveldb_readoptions_destroy(c_readoptions);
      leveldb_iter_seek_to_first(ptr);
      ValueIterator { start: true, iter: RawIterator { ptr: ptr }, database: database }
    }
  }

  /// return the last element of the iterator
  pub fn last(self) -> Option<Vec<u8>> {
     self.seek_to_last();
     Some(self.value())
  }
}

impl<'a,K: Key> LevelDBIterator<K> for ValueIterator<'a,K> {
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
}

impl<'a, K: Key> LevelDBIteratorExt<K> for ValueIterator<'a,K> { }

impl<'a,K: Key> iter::Iterator<(K,Vec<u8>)> for Iterator<'a,K> {
  fn next(&mut self) -> Option<(K,Vec<u8>)> {
    if self.advance() {
      Some((self.key(), self.value()))
    } else {
      None
    }
  }
}

impl<'a, K: Key> iter::Iterator<K> for KeyIterator<'a,K> {
  fn next(&mut self) -> Option<K> {
    if self.advance() {
      Some(self.key())
    } else {
      None
    }
  }
}

impl<'a, K: Key> iter::Iterator<Vec<u8>> for ValueIterator<'a,K> {
  fn next(&mut self) -> Option<Vec<u8>> {
    if self.advance() {
      Some(self.value())
    } else {
      None
    }
  }
}
