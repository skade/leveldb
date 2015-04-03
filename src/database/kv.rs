//! Key-Value semantics.

use super::Database;

use options::{WriteOptions,ReadOptions,c_writeoptions,c_readoptions};
use super::error::Error;
use database::key::Key;
use std::ptr;
use std::slice::from_raw_parts;
use libc::{c_char,size_t};
use cbits::leveldb::*;

/// Key-Value-Access to the leveldb database, providing
/// a basic interface.
pub trait KV<K: Key> {
    /// get a value from the database.
    ///
    /// The passed key will be compared using the comparator.
    fn get<'a>(&self,
           options: ReadOptions<'a,K>,
           key: K) -> Result<Option<Vec<u8>>, Error>;
    /// put a binary value into the database.
    ///
    /// If the key is already present in the database, it will be overwritten.
    ///
    /// The passed key will be compared using the comparator.
    ///
    /// The database will be synced to disc if `options.sync == true`. This is
    /// NOT the default.
    fn put(&self,
           options: WriteOptions,
           key: K,
           value: &[u8]) -> Result<(), Error>;
    /// delete a value from the database.
    ///
    /// The passed key will be compared using the comparator.
    ///
    /// The database will be synced to disc if `options.sync == true`. This is
    /// NOT the default.
    fn delete(&self,
              options: WriteOptions,
              key: K) -> Result<(), Error>;
}

impl<K: Key> KV<K> for Database<K> {
  /// put a binary value into the database.
  ///
  /// If the key is already present in the database, it will be overwritten.
  ///
  /// The passed key will be compared using the comparator.
  ///
  /// The database will be synced to disc if `options.sync == true`. This is
  /// NOT the default.
  fn put(&self,
             options: WriteOptions,
             key: K,
             value: &[u8]) -> Result<(), Error> {
    unsafe {
      key.as_slice(|k| {
        let mut error = ptr::null();
        let c_writeoptions = c_writeoptions(options);
        leveldb_put(self.database.ptr,
                    c_writeoptions,
                    k.as_ptr() as *mut c_char,
                    k.len() as size_t,
                    value.as_ptr() as *mut c_char,
                    value.len() as size_t,
                    &mut error);
        leveldb_writeoptions_destroy(c_writeoptions);

        if error == ptr::null() {
          Ok(())
        } else {
          Err(Error::new_from_i8(error))
        }
      })
    }
  }

  /// delete a value from the database.
  ///
  /// The passed key will be compared using the comparator.
  ///
  /// The database will be synced to disc if `options.sync == true`. This is
  /// NOT the default.
  fn delete(&self,
                options: WriteOptions,
                key: K) -> Result<(), Error> {
    unsafe {
      key.as_slice(|k| {
        let mut error = ptr::null();
        let c_writeoptions = c_writeoptions(options);
        leveldb_delete(self.database.ptr,
                       c_writeoptions,
                       k.as_ptr() as *mut c_char,
                       k.len() as size_t,
                       &mut error);
        leveldb_writeoptions_destroy(c_writeoptions);
        if error == ptr::null() {
          Ok(())
        } else {
          Err(Error::new_from_i8(error))
        }
      })
    }
  }

  /// get a value from the database.
  ///
  /// The passed key will be compared using the comparator.
  fn get<'a>(&self,
             options: ReadOptions<'a,K>,
             key: K) -> Result<Option<Vec<u8>>, Error> {
    unsafe {
      key.as_slice(|k| {
        let mut error = ptr::null();
        let length: size_t = 0;
        let c_readoptions = c_readoptions(&options);
        let result = leveldb_get(self.database.ptr,
                                 c_readoptions,
                                 k.as_ptr() as *mut c_char,
                                 k.len() as size_t,
                                 &length,
                                 &mut error);
        leveldb_readoptions_destroy(c_readoptions);

        if error == ptr::null() {
          if result == ptr::null() {
            Ok(None)
          } else {
            let vec: Vec<u8> = from_raw_parts(result, length as usize).to_vec();
            Ok(Some(vec))
          }
        } else {
          Err(Error::new_from_i8(error))
        }
      })
    }
  }
}
