
use cbits::leveldb::*;

use self::options::{Options,WriteOptions,ReadOptions};
use self::error::Error;

use std::ptr;
use std::libc::{c_char, size_t};
use std::str::raw::*;
use std::slice::*;

pub mod options;
pub mod error;
pub mod iterator;
pub mod binary;
pub mod json;

pub struct Database {
  database: *leveldb_t,
}

impl Database {
  fn new(database: *leveldb_t) -> Database {
    Database { database: database }
  }

  pub fn open(name: ~str, options: Options) -> Result<Database,Error> {
    name.with_c_str(|c_string| {
      unsafe {
        let error = ptr::null();
        let database = leveldb_open(options.options, c_string, &error);
        if error == ptr::null() {
          Ok(Database::new(database))
        } else {
          Err(Error { message: from_c_str(error) })
        }
      }
    })
  }

  fn put_binary(&mut self,
                options: WriteOptions,
                key: &[u8],
                value: &[u8]) -> Result<(), Error> {
    unsafe {
      let error = ptr::null();
      leveldb_put(self.database,
                  options.options,
                  key.as_ptr() as *c_char,
                  key.len() as size_t,
                  value.as_ptr() as *c_char,
                  value.len() as size_t,
                  &error);

      if error == ptr::null() {
        Ok(())
      } else {
        Err( Error { message: from_c_str(error) } )
      }
    }
  }

  fn delete_binary(&mut self,
                   options: WriteOptions,
                   key: &[u8]) -> Result<(), Error> {
    unsafe {
      let error = ptr::null();
      leveldb_delete(self.database,
                     options.options,
                     key.as_ptr() as *c_char,
                     key.len() as size_t,
                     &error);
      if error == ptr::null() {
        Ok(())
      } else {
        Err( Error { message: from_c_str(error) } )
      }
    }
  }

  fn get_binary(&mut self,
                options: ReadOptions,
                key: &[u8]) -> Result<Option<~[u8]>, Error> {
    unsafe {
      let error = ptr::null();
      let length: size_t = 0;
      let result = leveldb_get(self.database,
                               options.options,
                               key.as_ptr() as *c_char,
                               key.len() as size_t,
                               &length,
                               &error);

       if error == ptr::null() {
         if result == ptr::null() {
           Ok(None)
         } else {
           let vec: ~[u8] = from_buf(result, length as uint);
           Ok(Some(vec))
         }
       } else {
         Err( Error { message: from_c_str(error) } )
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
