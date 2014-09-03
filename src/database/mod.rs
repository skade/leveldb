use cbits::leveldb::*;

use self::options::{Options,WriteOptions,ReadOptions};
use self::error::Error;

use std::ptr;
use std::vec::raw::*;
use libc::{c_char, size_t};
use std::slice::*;
use std::string;

pub mod options;
pub mod error;
pub mod iterator;
pub mod comparator;
pub mod binary;
pub mod json;

pub struct Database {
  database: *mut leveldb_t,
  #[allow(dead_code)]
  options: Options
}

impl Database {
  fn new(database: *mut leveldb_t, options: Options) -> Database {
    Database { database: database, options: options }
  }

  pub fn open(name: Path, options: Options) -> Result<Database,Error> {
    let mut error = ptr::null();
    let res = name.with_c_str(|c_string| {
      unsafe { leveldb_open(options.options(), c_string, &mut error) }
    });

    if error == ptr::null() {
      Ok(Database::new(res, options))
    } else {
      Err(Error::new(unsafe { string::raw::from_buf(error as *const u8) }))
    }
  }

  fn put_binary(&mut self,
                options: WriteOptions,
                key: &[u8],
                value: &[u8]) -> Result<(), Error> {
    unsafe {
      let mut error = ptr::null();
      leveldb_put(self.database,
                  options.options(),
                  key.as_ptr() as *mut c_char,
                  key.len() as size_t,
                  value.as_ptr() as *mut c_char,
                  value.len() as size_t,
                  &mut error);

      if error == ptr::null() {
        Ok(())
      } else {
        Err(Error::new(string::raw::from_buf(error as *const u8)))
      }
    }
  }

  fn delete_binary(&mut self,
                   options: WriteOptions,
                   key: &[u8]) -> Result<(), Error> {
    unsafe {
      let mut error = ptr::null();
      leveldb_delete(self.database,
                     options.options(),
                     key.as_ptr() as *mut c_char,
                     key.len() as size_t,
                     &mut error);
      if error == ptr::null() {
        Ok(())
      } else {
        Err(Error::new(string::raw::from_buf(error as *const u8)))
      }
    }
  }

  fn get_binary(&mut self,
                options: ReadOptions,
                key: &[u8]) -> Result<Option<Vec<u8>>, Error> {
    unsafe {
      let mut error = ptr::null();
      let length: size_t = 0;
      let result = leveldb_get(self.database,
                               options.options(),
                               key.as_ptr() as *mut c_char,
                               key.len() as size_t,
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
    }
  }
}

impl Drop for Database {
  fn drop(&mut self) {
    unsafe {
      leveldb_close(self.database);
    }
  }
}
