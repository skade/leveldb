use cbits::leveldb::*;
use libc::{size_t};
use std::slice::*;
use std::vec::raw::*;
use std::vec::*;
use std::iter;
use super::Database;
use super::RawInterface;
use comparator::Comparator;
use super::options::ReadOptions;
use super::key::{Key,from_u8};

struct IteratorPtr {
  ptr: *mut leveldb_iterator_t,
}

impl Drop for IteratorPtr {
  fn drop(&mut self) {
    unsafe { leveldb_iter_destroy(self.ptr) }
  }
}

pub struct Iterator<K: Key, V> {
  start: bool,
  iter: IteratorPtr
}

pub struct KeyIterator<K: Key> {
  start: bool,
  iter: IteratorPtr
}

pub struct ValueIterator<V> {
  start: bool,
  iter: IteratorPtr
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

impl<K: Key, V> Iterator<K, V> {
  fn new<C>(database: &Database<C>,
         options: ReadOptions) -> Iterator<K,V> {
    unsafe {
      let ptr = leveldb_create_iterator(database.database.ptr,
                                         options.options());
      leveldb_iter_seek_to_first(ptr);
      Iterator { start: true, iter: IteratorPtr { ptr: ptr } }
    }
  }

  pub fn valid(&self) -> bool {
    unsafe { leveldb_iter_valid(self.iter.ptr) != 0 }
  }

  pub fn start(&self) -> bool {
    self.start
  }

  pub fn seek_to_first(self) {
    unsafe { leveldb_iter_seek_to_first(self.iter.ptr) }
  }

  fn value(&self) -> Vec<u8> {
    unsafe {
      let length: size_t = 0;
      let value = leveldb_iter_value(self.iter.ptr,
                                     &length) as *const u8;
      let vec = from_buf(value, length as uint);
      vec
    }
  }

  fn key(&self) -> K {
    unsafe {
      let length: size_t = 0;
      let value = leveldb_iter_key(self.iter.ptr,
                                   &length) as *const u8;
      from_u8(from_buf(value, length as uint).as_slice())
    }
  }
}

impl<K: Key> KeyIterator<K> {
  fn new<C>(database: &Database<C>,
         options: ReadOptions) -> KeyIterator<K> {
    unsafe {
      let ptr = leveldb_create_iterator(database.database.ptr,
                                         options.options());
      leveldb_iter_seek_to_first(ptr);
      KeyIterator { start: true, iter: IteratorPtr { ptr: ptr } }
    }
  }

  pub fn valid(&self) -> bool {
    unsafe { leveldb_iter_valid(self.iter.ptr) != 0 }
  }

  pub fn start(&self) -> bool {
    self.start
  }

  pub fn seek_to_first(self) {
    unsafe { leveldb_iter_seek_to_first(self.iter.ptr) }
  }
}

impl<V> ValueIterator<V> {
  fn new<C>(database: &Database<C>,
         options: ReadOptions) -> ValueIterator<V> {
    unsafe {
      let ptr = leveldb_create_iterator(database.database.ptr,
                                         options.options());
      leveldb_iter_seek_to_first(ptr);
      ValueIterator { start: true, iter: IteratorPtr { ptr: ptr } }
    }
  }

  pub fn valid(&self) -> bool {
    unsafe { leveldb_iter_valid(self.iter.ptr) != 0 }
  }

  pub fn start(&self) -> bool {
    self.start
  }

  pub fn seek_to_first(self) {
    unsafe { leveldb_iter_seek_to_first(self.iter.ptr) }
  }
}

impl<K: Key> iter::Iterator<(K,Vec<u8>)> for Iterator<K,Vec<u8>> {
  fn next(&mut self) -> Option<(K,Vec<u8>)> {
    unsafe {
      if !self.start() {
        leveldb_iter_next(self.iter.ptr);
      } else {
        self.start = false;
      }
      if self.valid() {
        Some((self.key(), self.value()))
      } else {
        None
      }
    }
  }

  fn last(&mut self) -> Option<(K,Vec<u8>)> {
    unsafe {
      leveldb_iter_seek_to_last(self.iter.ptr);
      Some((self.key(), self.value()))
    }
  }
}
