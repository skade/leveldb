extern crate db_key;

use cbits::leveldb::*;

use self::options::{Options,WriteOptions,ReadOptions,c_options,c_writeoptions,c_readoptions};
use self::error::Error;

use std::ptr;
use std::vec::raw::*;
use libc::{c_char,size_t};
use std::slice::*;
use std::string;
use comparator::{Comparator,create_comparator};
use self::db_key::Key;

pub mod options;
pub mod error;
pub mod iterator;
pub mod comparator;

#[allow(missing_docs)]
struct RawDB {
  ptr: *mut leveldb_t
}

#[allow(missing_docs)]
impl Drop for RawDB {
  fn drop(&mut self) {
    unsafe {
      leveldb_close(self.ptr);
    }
  }
}

#[allow(missing_docs)]
struct RawComparator {
  ptr: *mut leveldb_comparator_t
}

#[allow(missing_docs)]
impl Drop for RawComparator {
  fn drop(&mut self) {
    unsafe {
      leveldb_comparator_destroy(self.ptr);
    }
  }
}

/// The main database object.
///
/// leveldb databases are based on ordered keys, the `Database` struct
/// requires the `Key` `K` also to be `Ord`. Additionally, a `Comparator`
/// can be supplied, which allows to implement custom comparison logic.
///
/// When re-opening a database, you should use the same key type `K` and
/// comparator type `C`.
///
/// Multiple Database objects can be kept around, as leveldb synchronises
/// internally.
pub struct Database<K: Key + Ord> {
  database: RawDB,
  // this holds a reference passed into leveldb
  // it is never read from Rust, but must be kept around
  #[allow(dead_code)]
  comparator: Option<RawComparator>,
}

impl<K: Key + Ord> Database<K> {
  fn new(database: *mut leveldb_t, comparator: Option<*mut leveldb_comparator_t>) -> Database<K> {
    let raw_comp = match comparator {
      Some(p) => Some(RawComparator { ptr: p }),
      None => None
    };
    Database { database: RawDB { ptr: database }, comparator: raw_comp }
  }

  /// Open a new database
  ///
  /// If the database is missing, the behaviour depends on `options.create_if_missing`.
  /// The database will be created using the settings given in `options`.
  pub fn open(name: Path, options: Options) -> Result<Database<K>,Error> {
    let mut error = ptr::null();
    let res = name.with_c_str(|c_string| {
      unsafe {
        let c_options = c_options(options, None);
        let db = leveldb_open(c_options as *const leveldb_options_t, c_string, &mut error);
        leveldb_options_destroy(c_options);
        db
      }
    });

    if error == ptr::null() {
      Ok(Database::new(res, None))
    } else {
      Err(Error::new(unsafe { string::raw::from_buf(error as *const u8) }))
    }
  }

  pub fn open_with_comparator<C: Comparator<K>>(name: Path, options: Options, comparator: C) -> Result<Database<K>,Error> {
    let mut error = ptr::null();
    let comp_ptr = create_comparator(box comparator);
    let res = name.with_c_str(|c_string| {
      unsafe {
        let c_options = c_options(options, Some(comp_ptr));
        let db = leveldb_open(c_options as *const leveldb_options_t, c_string, &mut error);
        leveldb_options_destroy(c_options);
        db
      }
    });

    if error == ptr::null() {
      Ok(Database::new(res, Some(comp_ptr)))
    } else {
      Err(Error::new(unsafe { string::raw::from_buf(error as *const u8) }))
    }
  }

  // put a binary value into the database.
  //
  // If the key is already present in the database, it will be overwritten.
  //
  // The passed key will be compared using the comparator.
  //
  // The database will be synced to disc if `options.sync == true`. This is
  // NOT the default.
  pub fn put(&mut self,
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
          Err(Error::new(string::raw::from_buf(error as *const u8)))
        }
      })
    }
  }

  // delete a value from the database.
  //
  // The passed key will be compared using the comparator.
  //
  // The database will be synced to disc if `options.sync == true`. This is
  // NOT the default.
  pub fn delete(&mut self,
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
          Err(Error::new(string::raw::from_buf(error as *const u8)))
        }
      })
    }
  }

  // get a value from the database.
  //
  // The passed key will be compared using the comparator.
  pub fn get(&self,
             options: ReadOptions,
             key: K) -> Result<Option<Vec<u8>>, Error> {
    unsafe {
      key.as_slice(|k| {
        let mut error = ptr::null();
        let length: size_t = 0;
        let c_readoptions = c_readoptions(options);
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
