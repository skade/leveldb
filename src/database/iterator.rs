use cbits::leveldb::*;
use libc::{size_t};
use std::slice::*;
use std::vec::raw::*;
use std::vec::*;
use std::iter;
use super::Database;
use comparator::Comparator;
use super::options::ReadOptions;
use super::key::{Key,from_u8};

struct RawIterator {
  ptr: *mut leveldb_iterator_t,
}

impl Drop for RawIterator {
  fn drop(&mut self) {
    unsafe { leveldb_iter_destroy(self.ptr) }
  }
}

pub struct Iterator<K: Key, V> {
  start: bool,
  iter: RawIterator
}

pub struct KeyIterator<K: Key> {
  start: bool,
  iter: RawIterator
}

pub struct ValueIterator<V> {
  start: bool,
  iter: RawIterator
}

pub trait Iterable<K: Key,V> {
  fn iter(&self, options: ReadOptions) -> Iterator<K,V>;
  fn keys_iter(&self, options: ReadOptions) -> KeyIterator<K>;
  fn value_iter(&self, options: ReadOptions) -> ValueIterator<V>;
}

impl<K: Key, C: Comparator<K>, V> Iterable<K, V> for Database<C> {
  fn iter(&self, options: ReadOptions) -> Iterator<K,V> {
    Iterator::new::<C>(self, options)
  }
  fn keys_iter(&self, options: ReadOptions) -> KeyIterator<K> {
    KeyIterator::new::<C>(self, options)
  }

  fn value_iter(&self, options: ReadOptions) -> ValueIterator<V> {
    ValueIterator::new::<C>(self, options)
  }
}

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

trait KeyValueAccess<K: Key, V> : KeyAccess<K> + ValueAccess<V> {}

impl<K: Key, V> Iterator<K, V> {
  fn new<C>(database: &Database<C>,
         options: ReadOptions) -> Iterator<K,V> {
    unsafe {
      let ptr = leveldb_create_iterator(database.database.ptr,
                                        options.options());
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
  fn new<C>(database: &Database<C>,
         options: ReadOptions) -> KeyIterator<K> {
    unsafe {
      let ptr = leveldb_create_iterator(database.database.ptr,
                                         options.options());
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

impl<V> ValueIterator<V> {
  fn new<C>(database: &Database<C>,
         options: ReadOptions) -> ValueIterator<V> {
    unsafe {
      let ptr = leveldb_create_iterator(database.database.ptr,
                                         options.options());
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
