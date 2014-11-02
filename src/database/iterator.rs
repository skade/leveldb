use cbits::leveldb::*;
use libc::{size_t};
use std::slice::*;
use std::vec::raw::*;
use std::vec::*;
use std::iter;
use super::Database;
use comparator::Comparator;
use super::options::ReadOptions;

pub struct Iterator {
  iter: *mut leveldb_iterator_t,
  start: bool
}

pub trait Iterable {
  fn iter(&self, options: ReadOptions) -> Iterator;
}

impl<K, C: Comparator<K>> Iterable for Database<C> {
  fn iter(&self, options: ReadOptions) -> Iterator {
    Iterator::new(self, options)
  }
}

impl Iterator {
  fn new<K, C: Comparator<K>>(database: &Database<C>,
         options: ReadOptions) -> Iterator {
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

impl Drop for Iterator {
  fn drop(&mut self) {
    unsafe { leveldb_iter_destroy(self.iter) }
  }
}

pub struct Entry {
  iter: *mut leveldb_iterator_t
}

impl Entry {
  pub fn value(self) -> Vec<u8> {
    unsafe {
      let length: size_t = 0;
      let value = leveldb_iter_value(self.iter,
                                     &length) as *const u8;
      from_buf(value, length as uint)
    }
  }

  pub fn key(self) -> Vec<u8> {
    unsafe {
      let length: size_t = 0;
      let value = leveldb_iter_key(self.iter,
                                   &length) as *const u8;
      from_buf(value, length as uint)
    }
  }
}

impl iter::Iterator<Entry> for Iterator {
  fn next(&mut self) -> Option<Entry> {
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

  fn last(&mut self) -> Option<Entry> {
    unsafe {
      leveldb_iter_seek_to_last(self.iter);
      Some(Entry { iter: self.iter })
    }
  }

}

