extern crate key;

use cbits::leveldb::*;

use self::options::{Options,WriteOptions,ReadOptions};
use self::error::Error;

use std::ptr;
use std::vec::raw::*;
use libc::{c_char, size_t};
use std::slice::*;
use std::string;
use comparator::{Comparator,create_comparator};
use self::key::Key;

pub mod options;
pub mod error;
pub mod iterator;
pub mod comparator;
pub mod binary;
pub mod json;

unsafe fn c_options(options: Options, comparator: Option<*mut leveldb_comparator_t>) -> *mut leveldb_options_t {
  let c_options = leveldb_options_create();
  leveldb_options_set_create_if_missing(c_options, options.create_if_missing as i8);
  leveldb_options_set_error_if_exists(c_options, options.error_if_exists as i8);
  leveldb_options_set_paranoid_checks(c_options, options.paranoid_checks as i8);
  if options.write_buffer_size.is_some() {
    leveldb_options_set_write_buffer_size(c_options, options.write_buffer_size.unwrap());
  }
  if options.max_open_files.is_some() {
    leveldb_options_set_max_open_files(c_options, options.max_open_files.unwrap());
  }
  if options.block_size.is_some() {
    leveldb_options_set_block_size(c_options, options.block_size.unwrap());
  }
  if options.block_restart_interval.is_some() {
    leveldb_options_set_block_restart_interval(c_options, options.block_restart_interval.unwrap());
  }
  leveldb_options_set_compression(c_options, options.compression);
  if comparator.is_some() {
    leveldb_options_set_comparator(c_options, comparator.unwrap());
  }
  c_options
}

struct RawDB {
  ptr: *mut leveldb_t
}

impl Drop for RawDB {
  fn drop(&mut self) {
    unsafe {
      leveldb_close(self.ptr);
    }
  }
}

struct RawComparator {
  ptr: *mut leveldb_comparator_t
}

impl Drop for RawComparator {
  fn drop(&mut self) {
    unsafe {
      leveldb_comparator_destroy(self.ptr);
    }
  }
}

pub struct Database<C> {
  database: RawDB,
  #[allow(dead_code)] // this holds a reference passed into leveldb
  comparator: Option<RawComparator>,
}

pub trait RawInterface<K: Key> {
  fn put_binary(&mut self,
                options: WriteOptions,
                key: K,
                value: &[u8]) -> Result<(), Error>;
  fn get_binary(&self,
                options: ReadOptions,
                key: K) -> Result<Option<Vec<u8>>, Error>;
  fn delete_binary(&mut self,
                   options: WriteOptions,
                   key: K) -> Result<(), Error>;
}

impl<K: Key, C: Comparator<K>> Database<C> {
  fn new(database: *mut leveldb_t, comparator: Option<*mut leveldb_comparator_t>) -> Database<C> {
    let raw_comp = match comparator {
      Some(p) => Some(RawComparator { ptr: p }),
      None => None
    };
    Database { database: RawDB { ptr: database }, comparator: raw_comp }
  }

  pub fn open(name: Path, options: Options, comparator: Option<C>) -> Result<Database<C>,Error> {
    let mut error = ptr::null();
    let comp_ptr = match comparator {
      Some(c) => Some(create_comparator(box c)),
      None => None
    };
    let res = name.with_c_str(|c_string| {
      unsafe {
        let c_options = c_options(options, comp_ptr);
        let db = leveldb_open(c_options as *const leveldb_options_t, c_string, &mut error);
        leveldb_options_destroy(c_options);
        db
      }
    });

    if error == ptr::null() {
      Ok(Database::new(res, comp_ptr))
    } else {
      Err(Error::new(unsafe { string::raw::from_buf(error as *const u8) }))
    }
  }
}

impl<K: Key, C: Comparator<K>> RawInterface<K> for Database<C> {
  fn put_binary(&mut self,
                options: WriteOptions,
                key: K,
                value: &[u8]) -> Result<(), Error> {
    unsafe {
      key.as_slice(|k| {
        let mut error = ptr::null();
        leveldb_put(self.database.ptr,
                    options.options(),
                    k.as_ptr() as *mut c_char,
                    k.len() as size_t,
                    value.as_ptr() as *mut c_char,
                    value.len() as size_t,
                    &mut error);

        if error == ptr::null() {
          Ok(())
        } else {
          Err(Error::new(string::raw::from_buf(error as *const u8)))
        }
      })
    }
  }

  fn delete_binary(&mut self,
                   options: WriteOptions,
                   key: K) -> Result<(), Error> {
    unsafe {
      key.as_slice(|k| {
        let mut error = ptr::null();
        leveldb_delete(self.database.ptr,
                       options.options(),
                       k.as_ptr() as *mut c_char,
                       k.len() as size_t,
                       &mut error);
        if error == ptr::null() {
          Ok(())
        } else {
          Err(Error::new(string::raw::from_buf(error as *const u8)))
        }
      })
    }
  }

  fn get_binary(&self,
                options: ReadOptions,
                key: K) -> Result<Option<Vec<u8>>, Error> {
    unsafe {
      key.as_slice(|k| {
        let mut error = ptr::null();
        let length: size_t = 0;
        let result = leveldb_get(self.database.ptr,
                                 options.options(),
                                 k.as_ptr() as *mut c_char,
                                 k.len() as size_t,
                                 &length,
                                 &mut error);

         if error == ptr::null() {
           if result == ptr::null() {
             Ok(None)
           } else {
             let vec: Vec<u8> = from_buf(result, length as uint);
             Ok(Some(vec))
           }
         } else {
           Err(Error::new(string::raw::from_buf(error as *const u8)))
         }
      })
    }
  }
}

pub trait Interface<T, K: Key, V> : RawInterface<K> {
  fn put(&mut self,
        options: WriteOptions,
        key: K,
        value: V) -> Result<(), Error> {
    let binary = self.to_binary(value);
    self.put_binary(options, key, binary.as_slice())
  }
  fn delete(&mut self,
            options: WriteOptions,
            key: K) -> Result<(), Error> {
    self.delete_binary(options, key)
  }
  fn get(&self,
         options: ReadOptions,
         key: K) -> Result<Option<V>, Error> {
    let result = self.get_binary(options, key);
    match result {
      Err(error) => { Err(error) },
      Ok(opt) => {
        match opt {
          None => { Ok(None) },
          Some(binary) => {
            self.from_binary(binary)
          }
        }
      }
    }
  }
  fn from_binary(&self, binary: Vec<u8>) -> Result<Option<V>, Error>;
  fn to_binary(&mut self, val: V) -> Vec<u8>;
}
