//! The main database module, allowing to interface with leveldb on
//! a key-value basis.
extern crate db_key as key;

use leveldb_sys::*;

use self::options::{Options, c_options};
use self::error::Error;
use std::ffi::CString;

use std::path::Path;

use std::ptr;
use comparator::{Comparator, create_comparator};
use self::key::Key;

use std::marker::PhantomData;
use libc::c_char;

pub mod options;
pub mod error;
pub mod iterator;
pub mod comparator;
pub mod snapshots;
pub mod cache;
pub mod kv;
pub mod batch;
pub mod management;
pub mod compaction;
pub mod bytes;

#[allow(missing_docs)]
struct RawDB {
    ptr: *mut leveldb_t,
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
    ptr: *mut leveldb_comparator_t,
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
/// When re-CString a database, you must use the same key type `K` and
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
    options: Options,
    marker: PhantomData<K>,
}

unsafe impl<K: Key> Sync for Database<K> {}
unsafe impl<K: Key> Send for Database<K> {}

impl<K: Key> Database<K> {
    fn new(database: *mut leveldb_t,
           options: Options,
           comparator: Option<*mut leveldb_comparator_t>)
           -> Database<K> {
        let raw_comp = match comparator {
            Some(p) => Some(RawComparator { ptr: p }),
            None => None,
        };
        Database {
            database: RawDB { ptr: database },
            comparator: raw_comp,
            options: options,
            marker: PhantomData,
        }
    }

    /// Open a new database
    ///
    /// If the database is missing, the behaviour depends on `options.create_if_missing`.
    /// The database will be created using the settings given in `options`.
    pub fn open(name: &Path, options: Options) -> Result<Database<K>, Error> {
        let mut error = ptr::null_mut();
        unsafe {
            let c_string = CString::new(name.to_str().unwrap()).unwrap();
            let c_options = c_options(&options, None);
            let db = leveldb_open(c_options as *const leveldb_options_t,
                                  c_string.as_bytes_with_nul().as_ptr() as *const c_char,
                                  &mut error);
            leveldb_options_destroy(c_options);

            if error == ptr::null_mut() {
                Ok(Database::new(db, options, None))
            } else {
                Err(Error::new_from_char(error))
            }
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
    pub fn open_with_comparator<C: Comparator<K = K>>(name: &Path,
                                                      options: Options,
                                                      comparator: C)
                                                      -> Result<Database<K>, Error> {
        let mut error = ptr::null_mut();
        let comp_ptr = create_comparator(Box::new(comparator));
        unsafe {
            let c_string = CString::new(name.to_str().unwrap()).unwrap();
            let c_options = c_options(&options, Some(comp_ptr));
            let db = leveldb_open(c_options as *const leveldb_options_t,
                                  c_string.as_bytes_with_nul().as_ptr() as *const c_char,
                                  &mut error);
            leveldb_options_destroy(c_options);

            if error == ptr::null_mut() {
                Ok(Database::new(db, options, Some(comp_ptr)))
            } else {
                Err(Error::new_from_char(error))
            }
        }
    }
}
