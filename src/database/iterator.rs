//! leveldb iterators
//!
//! Iteration is one of the most important parts of leveldb. This module provides
//! Iterators to iterate over key, values and pairs of both.
use cbits::leveldb::{leveldb_iterator_t,leveldb_iter_seek_to_first,leveldb_iter_destroy,leveldb_iter_seek_to_last,
leveldb_create_iterator,leveldb_iter_valid,leveldb_iter_next,leveldb_iter_key,leveldb_iter_value,leveldb_readoptions_destroy};
use libc::{size_t};
use std::vec::raw::from_buf;
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
pub struct Iterator<K: Key, V> {
  start: bool,
  iter: RawIterator
}

/// An iterator over the leveldb keyspace.
///
/// Returns just the keys.
pub struct KeyIterator<K: Key> {
  start: bool,
  iter: RawIterator
}

/// An iterator over the leveldb keyspace.
///
/// Returns just the value.
pub struct ValueIterator<V> {
  start: bool,
  iter: RawIterator
}


/// A trait to allow access to the three main iteration styles of leveldb.
pub trait Iterable<K: Key,V> {
  /// Return an Iterator iterating over (Key,Value) pairs
  fn iter(&self, options: ReadOptions) -> Iterator<K,V>;
  /// Returns an Iterator iterating over Keys only.
  fn keys_iter(&self, options: ReadOptions) -> KeyIterator<K>;
  /// Returns an Iterator iterating over Values only.
  fn value_iter(&self, options: ReadOptions) -> ValueIterator<V>;
}

impl<K: Key + Ord, V> Iterable<K, V> for Database<K> {
  fn iter(&self, options: ReadOptions) -> Iterator<K,V> {
    Iterator::new(self, options)
  }
  fn keys_iter(&self, options: ReadOptions) -> KeyIterator<K> {
    KeyIterator::new(self, options)
  }

  fn value_iter(&self, options: ReadOptions) -> ValueIterator<V> {
    ValueIterator::new(self, options)
  }
}

#[allow(missing_docs)]
trait LevelDBIterator {
  #[inline]
  fn raw_iterator(&self) -> *mut leveldb_iterator_t;
  
  #[inline]
  fn start(&self) -> bool;

  #[inline]
  fn started(&mut self);

  fn valid(&self) -> bool {
    unsafe { leveldb_iter_valid(self.raw_iterator()) != 0 }
  }

  fn seek_to_first(&self) {
    unsafe { leveldb_iter_seek_to_first(self.raw_iterator()) }
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

  fn seek_last(&mut self) {
    unsafe {
      leveldb_iter_seek_to_last(self.raw_iterator());
    }
  }
}

#[allow(missing_docs)]
trait ValueAccess<V> : LevelDBIterator {
  fn value(&self) -> V {
    unsafe {
      let length: size_t = 0;
      let value = leveldb_iter_value(self.raw_iterator(),
                                     &length) as *const u8;
      let vec = from_buf(value, length as uint);
      self.convert_value(vec)
    }
  }

  fn convert_value(&self, v: Vec<u8>) -> V;
}

#[allow(missing_docs)]
trait KeyAccess<K: Key> : LevelDBIterator {
  fn key(&self) -> K {
    unsafe {
      let length: size_t = 0;
      let value = leveldb_iter_key(self.raw_iterator(),
                                   &length) as *const u8;
      from_u8(from_buf(value, length as uint).as_slice())
    }
  }
}

impl<K: Key, V> Iterator<K, V> {
  fn new(database: &Database<K>,
         options: ReadOptions) -> Iterator<K,V> {
    unsafe {
      let c_readoptions = c_readoptions(options);
      let ptr = leveldb_create_iterator(database.database.ptr,
                                        c_readoptions);
      leveldb_readoptions_destroy(c_readoptions);
      leveldb_iter_seek_to_first(ptr);
      Iterator { start: true, iter: RawIterator { ptr: ptr } }
    }
  }
}

impl<K: Key, V> LevelDBIterator for Iterator<K,V> {
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

impl<K: Key, V> KeyAccess<K> for Iterator<K,V> {}
impl<K: Key> ValueAccess<Vec<u8>> for Iterator<K,Vec<u8>> {
  fn convert_value(&self, val: Vec<u8>) -> Vec<u8> {
    val
  }
}

impl<K: Key> KeyIterator<K> {
  fn new(database: &Database<K>,
         options: ReadOptions) -> KeyIterator<K> {
    unsafe {
      let c_readoptions = c_readoptions(options);
      let ptr = leveldb_create_iterator(database.database.ptr,
                                        c_readoptions);
      leveldb_readoptions_destroy(c_readoptions);
      leveldb_iter_seek_to_first(ptr);
      KeyIterator { start: true, iter: RawIterator { ptr: ptr } }
    }
  }
}

impl<K: Key> LevelDBIterator for KeyIterator<K> {
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

impl<K: Key> KeyAccess<K> for KeyIterator<K> { }

impl<K, V> ValueIterator<V> {
  fn new(database: &Database<K>,
         options: ReadOptions) -> ValueIterator<V> {
    unsafe {
      let c_readoptions = c_readoptions(options);
      let ptr = leveldb_create_iterator(database.database.ptr,
                                        c_readoptions);
      leveldb_readoptions_destroy(c_readoptions);
      leveldb_iter_seek_to_first(ptr);
      ValueIterator { start: true, iter: RawIterator { ptr: ptr } }
    }
  }
}

impl<V> LevelDBIterator for ValueIterator<V> {
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

impl ValueAccess<Vec<u8>> for ValueIterator<Vec<u8>> {
  fn convert_value(&self, val: Vec<u8>) -> Vec<u8> {
    val
  }
}

impl<K: Key> iter::Iterator<(K,Vec<u8>)> for Iterator<K,Vec<u8>> {
  fn next(&mut self) -> Option<(K,Vec<u8>)> {
    if self.advance() {
      Some((self.key(), self.value()))
    } else {
      None
    }
  }

  fn last(&mut self) -> Option<(K,Vec<u8>)> {
    self.seek_last();
    Some((self.key(), self.value()))
  }
}

impl<K: Key> iter::Iterator<K> for KeyIterator<K> {
  fn next(&mut self) -> Option<K> {
    if self.advance() {
      Some(self.key())
    } else {
      None
    }
  }

  fn last(&mut self) -> Option<K> {
     self.seek_last();
     Some(self.key())
  }
}

impl iter::Iterator<Vec<u8>> for ValueIterator<Vec<u8>> {
  fn next(&mut self) -> Option<Vec<u8>> {
    if self.advance() {
      Some(self.value())
    } else {
      None
    }
  }

  fn last(&mut self) -> Option<Vec<u8>> {
     self.seek_last();
     Some(self.value())
  }
}
