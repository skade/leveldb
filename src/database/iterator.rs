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

pub struct Iterator<K> {
  iter: *mut leveldb_iterator_t,
  start: bool
}

pub trait Iterable<K> {
  fn iter(&self, options: ReadOptions) -> Iterator<K>;
}

impl<K: Key, C: Comparator<K>> Iterable<K> for Database<C> {
  fn iter(&self, options: ReadOptions) -> Iterator<K> {
    Iterator::new::<C>(self, options)
  }
}

impl<K: Key> Iterator<K> {
  fn new<C: Comparator<K>>(database: &Database<C>,
         options: ReadOptions) -> Iterator<K> {
    unsafe {
      let iter = leveldb_create_iterator(database.database.ptr,
                                         options.options());
      leveldb_iter_seek_to_first(iter);
      Iterator { iter: iter, start: true }
    }
  }

  pub fn valid(&self) -> bool {
    unsafe { leveldb_iter_valid(self.iter) != 0 }
  }

  pub fn start(&self) -> bool {
    self.start
  }

  pub fn seek_to_first(self) {
    unsafe { leveldb_iter_seek_to_first(self.iter) }
  }

}

#[unsafe_destructor]
impl<K: Key> Drop for Iterator<K> {
  fn drop(&mut self) {
    unsafe { leveldb_iter_destroy(self.iter) }
  }
}

pub struct Entry<K: Key> {
  iter: *mut leveldb_iterator_t
}

impl<K: Key> Entry<K> {
  pub fn value(self) -> Vec<u8> {
    unsafe {
      let length: size_t = 0;
      let value = leveldb_iter_value(self.iter,
                                     &length) as *const u8;
      from_buf(value, length as uint)
    }
  }

  pub fn key(self) -> K {
    unsafe {
      let length: size_t = 0;
      let value = leveldb_iter_key(self.iter,
                                   &length) as *const u8;
      from_u8(from_buf(value, length as uint).as_slice())
    }
  }
}

impl<K: Key> iter::Iterator<Entry<K>> for Iterator<K> {
  fn next(&mut self) -> Option<Entry<K>> {
    unsafe {
      if !self.start() {
        leveldb_iter_next(self.iter);
      } else {
        self.start = false;
      }
      if self.valid() {
        let entry = Entry { iter: self.iter };
        Some(entry)
      } else {
        None
      }
    }
  }

  fn last(&mut self) -> Option<Entry<K>> {
    unsafe {
      leveldb_iter_seek_to_last(self.iter);
      Some(Entry { iter: self.iter })
    }
  }

}

