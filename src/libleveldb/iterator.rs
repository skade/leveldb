use cbits::leveldb::*;
use std::libc::{size_t};
use std::slice::*;
use std::iter;
use std::ptr;
use super::database::Database;
use super::options::ReadOptions;

pub struct Iterator {
  priv iter: *leveldb_iterator_t,
}

pub trait Iterable {
  fn iter(self, options: ReadOptions) -> Iterator;
}

impl Iterable for Database {
  fn iter(self, options: ReadOptions) -> Iterator {
    Iterator::new(self, options)
  }
}

impl Iterator {
  fn new(database: Database,
         options: ReadOptions) -> Iterator {
    unsafe {
      let iter = leveldb_create_iterator(database.database,
                                         options.options);
      leveldb_iter_seek_to_first(iter);
      Iterator { iter: iter }
    }
  }

  pub fn valid(self) -> bool {
    unsafe { leveldb_iter_valid(self.iter) != 0 }
  }

  pub fn seek_to_first(self) {
    unsafe { leveldb_iter_seek_to_first(self.iter) }
  }
}

impl iter::Iterator<~[i8]> for Iterator {
  fn next(&mut self) -> Option<~[i8]> {
    unsafe {
      let length: size_t = 0;
      if self.valid() {
        let value = leveldb_iter_value(self.iter,
                                       &length);
        let vec: ~[i8] = from_buf(value, length as uint);
        leveldb_iter_next(self.iter);
        Some(vec)
      } else {
        None
      }
    }
  }
}

