//! The main database module, allowing to interface with leveldb on
//! a key-value basis.
extern crate db_key;

use cbits::leveldb::*;
use libc::{c_void};
use libc::funcs::c95::stdlib::{free};

use self::options::{Options,WriteOptions,ReadOptions,c_options,c_writeoptions,c_readoptions};
use self::error::Error;
use std::ffi::CString;

use std::ptr;
use libc::{c_char,size_t};
use comparator::{Comparator,create_comparator};
use self::db_key::Key;

pub mod options;
pub mod error;
pub mod iterator;
pub mod comparator;
pub mod snapshots;
pub mod cache;

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

impl Drop for RawComparator {
  fn drop(&mut self) {
    unsafe {
      leveldb_comparator_destroy(self.ptr);
    }
  }
}

/// The main database object.
///
/// leveldb databases are based on ordered keys. By default, leveldb orders
/// by the binary value of the key. Additionally, a custom `Comparator` can
/// be passed when opening the database. This library ships with an Comparator
/// implementation for keys that are `Ord`.
///
/// When re-opening a database, you must use the same key type `K` and
/// comparator type `C`.
///
/// Multiple Database objects can be kept around, as leveldb synchronises
/// internally.
pub struct Database<K: Key> {
  database: RawDB,
  // this holds a reference passed into leveldb
  // it is never read from Rust, but must be kept around
  #[allow(dead_code)]
  comparator: Option<RawComparator>,
  // these hold multiple references that are used by the leveldb library
  // and should survive as long as the database lives
  #[allow(dead_code)]
  options: Options
}

impl<K: Key> Database<K> {
  fn new(database: *mut leveldb_t, options: Options, comparator: Option<*mut leveldb_comparator_t>) -> Database<K> {
    let raw_comp = match comparator {
      Some(p) => Some(RawComparator { ptr: p }),
      None => None
    };
    Database { database: RawDB { ptr: database }, comparator: raw_comp, options: options }
  }

  /// Open a new database
  ///
  /// If the database is missing, the behaviour depends on `options.create_if_missing`.
  /// The database will be created using the settings given in `options`.
  pub fn open(name: Path, options: Options) -> Result<Database<K>,Error> {
    let mut error = ptr::null();
    let res = unsafe {
      let c_string = CString::from_vec(name.into_vec());
      let c_options = c_options(&options, None);
      let db = leveldb_open(c_options as *const leveldb_options_t, c_string.as_slice_with_nul().as_ptr(), &mut error);
      leveldb_options_destroy(c_options);
      db
    };

    if error == ptr::null() {
      Ok(Database::new(res, options, None))
    } else {
      Err(to_error(error))
    }
  }

  /// Open a new database with a custom comparator
  ///
  /// If the database is missing, the behaviour depends on `options.create_if_missing`.
  /// The database will be created using the settings given in `options`.
  ///
  /// The comparator must implement a total ordering over the keyspace.
  ///
  /// For keys that implement Ord, consider the `OrdComparator`.
  pub fn open_with_comparator<C: Comparator<K>>(name: Path, options: Options, comparator: C) -> Result<Database<K>,Error> {
    let mut error = ptr::null();
    let comp_ptr = create_comparator(box comparator);
    let res = unsafe {
      let c_string = CString::from_vec(name.into_vec());
      let c_options = c_options(&options, Some(comp_ptr));
      let db = leveldb_open(c_options as *const leveldb_options_t, c_string.as_slice_with_nul().as_ptr(), &mut error);
      leveldb_options_destroy(c_options);
      db
    };

    if error == ptr::null() {
      Ok(Database::new(res, options, Some(comp_ptr)))
    } else {
      Err(to_error(error))
    }
  }

  /// put a binary value into the database.
  ///
  /// If the key is already present in the database, it will be overwritten.
  ///
  /// The passed key will be compared using the comparator.
  ///
  /// The database will be synced to disc if `options.sync == true`. This is
  /// NOT the default.
  pub fn put(&self,
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
          Err(to_error(error))
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
  pub fn delete(&self,
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
          Err(to_error(error))
        }
      })
    }
  }

  /// get a value from the database.
  ///
  /// The passed key will be compared using the comparator.
  pub fn get<'a>(&self,
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
            let vec: Vec<u8> = Vec::from_raw_buf(result, length as uint);
            Ok(Some(vec))
          }
        } else {
          Err(to_error(error))
        }
      })
    }
  }
}

fn to_error(leveldb_error: *const i8) -> Error {
  use std::str::from_utf8;
  use std::ffi::c_str_to_bytes;

  let err_string = unsafe { from_utf8(c_str_to_bytes(&leveldb_error)).to_string() };
  unsafe { free(leveldb_error as *mut c_void) };
  Error::new(err_string)
}
