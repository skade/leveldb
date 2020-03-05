//! Key-Value semantics.

use super::Database;

use options::{WriteOptions, ReadOptions, c_writeoptions, c_readoptions};
use super::error::Error;
use database::key::Key;
use std::ptr;
use std::borrow::Borrow;
use libc::{c_char, size_t};
use leveldb_sys::*;
use super::bytes::Bytes;

/// Key-Value-Access to the leveldb database, providing
/// a basic interface.
pub trait KV<K: Key> {
    /// get a value from the database.
    ///
    /// The passed key will be compared using the comparator.
    fn get<'a, BK: Borrow<K>>(&self, options: ReadOptions<'a, K>, key: BK) -> Result<Option<Vec<u8>>, Error>;

    /// get a value from the database.
    ///
    /// The passed key will be compared using the comparator.
    ///
    /// This version returns bytes allocated by leveldb without converting to `Vec<u8>`, which may
    /// lead to better performance.
    fn get_bytes<'a, BK: Borrow<K>>(&self, options: ReadOptions<'a, K>, key: BK) -> Result<Option<Bytes>, Error>;
    /// put a binary value into the database.
    ///
    /// If the key is already present in the database, it will be overwritten.
    ///
    /// The passed key will be compared using the comparator.
    ///
    /// The database will be synced to disc if `options.sync == true`. This is
    /// NOT the default.
    fn put<BK: Borrow<K>>(&self, options: WriteOptions, key: BK, value: &[u8]) -> Result<(), Error>;
    /// delete a value from the database.
    ///
    /// The passed key will be compared using the comparator.
    ///
    /// The database will be synced to disc if `options.sync == true`. This is
    /// NOT the default.
    fn delete<BK: Borrow<K>>(&self, options: WriteOptions, key: BK) -> Result<(), Error>;
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
    fn put<BK: Borrow<K>>(&self, options: WriteOptions, key: BK, value: &[u8]) -> Result<(), Error> {
        unsafe {
            key.borrow().as_slice(|k| {
                let mut error = ptr::null_mut();
                let c_writeoptions = c_writeoptions(options);
                leveldb_put(self.database.ptr,
                            c_writeoptions,
                            k.as_ptr() as *mut c_char,
                            k.len() as size_t,
                            value.as_ptr() as *mut c_char,
                            value.len() as size_t,
                            &mut error);
                leveldb_writeoptions_destroy(c_writeoptions);

                if error == ptr::null_mut() {
                    Ok(())
                } else {
                    Err(Error::new_from_char(error))
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
    fn delete<BK: Borrow<K>>(&self, options: WriteOptions, key: BK) -> Result<(), Error> {
        unsafe {
            key.borrow().as_slice(|k| {
                let mut error = ptr::null_mut();
                let c_writeoptions = c_writeoptions(options);
                leveldb_delete(self.database.ptr,
                               c_writeoptions,
                               k.as_ptr() as *mut c_char,
                               k.len() as size_t,
                               &mut error);
                leveldb_writeoptions_destroy(c_writeoptions);
                if error == ptr::null_mut() {
                    Ok(())
                } else {
                    Err(Error::new_from_char(error))
                }
            })
        }
    }

    fn get_bytes<'a, BK: Borrow<K>>(&self, options: ReadOptions<'a, K>, key: BK) -> Result<Option<Bytes>, Error> {
        unsafe {
            key.borrow().as_slice(|k| {
                let mut error = ptr::null_mut();
                let mut length: size_t = 0;
                let c_readoptions = c_readoptions(&options);
                let result = leveldb_get(self.database.ptr,
                                         c_readoptions,
                                         k.as_ptr() as *mut c_char,
                                         k.len() as size_t,
                                         &mut length,
                                         &mut error);
                leveldb_readoptions_destroy(c_readoptions);

                if error == ptr::null_mut() {
                    Ok(Bytes::from_raw(result as *mut u8, length))
                } else {
                    Err(Error::new_from_char(error))
                }
            })
        }
    }

    fn get<'a, BK: Borrow<K>>(&self, options: ReadOptions<'a, K>, key: BK) -> Result<Option<Vec<u8>>, Error> {
        self.get_bytes(options, key).map(|val| val.map(Into::into))
    }
}
