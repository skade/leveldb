//! Module providing write batches

use cbits::leveldb::*;
use libc::{c_char,size_t,c_void};
use core::marker::PhantomData;
use database::db_key::Key;
use std::slice;
use database::db_key::from_u8;
use options::{WriteOptions,c_writeoptions};
use super::error::Error;
use std::ptr;
use super::Database;

#[allow(missing_docs)]
struct RawWritebatch {
    ptr: *mut leveldb_writebatch_t
}

impl Drop for RawWritebatch {
    fn drop(&mut self) {
        unsafe { leveldb_writebatch_destroy(self.ptr); }
    }
}

#[allow(missing_docs)]
pub struct Writebatch<K: Key> {
    #[allow(dead_code)] //only used for holding the internal pointer
    writebatch: RawWritebatch,
    marker: PhantomData<K>
}

/// Batch access to the database
pub trait Batch<K: Key> {
    /// Write a batch to the database, ensuring success for all items or an error
    fn write(&self,
             options: WriteOptions,
             batch: Writebatch<K>) -> Result<(), Error>;
}

impl<K: Key> Batch<K> for Database<K> {
    fn write(&self,
             options: WriteOptions,
             batch: Writebatch<K>) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null();
            let c_writeoptions = c_writeoptions(options);

            leveldb_write(self.database.ptr,
                          c_writeoptions,
                          batch.writebatch.ptr,
                          &mut error);
            leveldb_writeoptions_destroy(c_writeoptions);

            if error == ptr::null() {
              Ok(())
            } else {
              Err(Error::new_from_i8(error))
            }
        }
    }
}

impl<K: Key> Writebatch<K> {
    /// Create a new writebatch
    pub fn new() -> Writebatch<K> {
        let ptr = unsafe { leveldb_writebatch_create() };
        let raw = RawWritebatch { ptr: ptr };
        Writebatch { writebatch: raw, marker: PhantomData }
    }

    /// Clear the writebatch
    pub fn clear(&mut self) {
        unsafe { leveldb_writebatch_clear(self.writebatch.ptr) };
    }

    /// Batch a put operation
    pub fn put(&mut self,
               key: K,
               value: &[u8]) {
        unsafe {
            key.as_slice(|k| {
                leveldb_writebatch_put(self.writebatch.ptr,
                            k.as_ptr() as *mut c_char,
                            k.len() as size_t,
                            value.as_ptr() as *mut c_char,
                            value.len() as size_t);
            })
        }
    }

    /// Batch a delete operation
    pub fn delete(&mut self,
                  key: K) {
        unsafe {
            key.as_slice(|k| {
                leveldb_writebatch_delete(self.writebatch.ptr,
                            k.as_ptr() as *mut c_char,
                            k.len() as size_t);
            })
        }
    }

    /// Iterate over the writebatch
    pub fn iterate<T: WritebatchIterator<K = K>>(&mut self,
                   iterator: Box<T>) {
        use std::mem;

        unsafe {
            leveldb_writebatch_iterate(self.writebatch.ptr,
                                       mem::transmute(iterator),
                                       put_callback::<K,T>,
                                       deleted_callback::<K,T>);
        }
    }
}

/// A trait for iterators to iterate over written batches and check their validity.
pub trait WritebatchIterator {
    /// The database key type this iterates over
    type K: Key;

    /// Callback for put items
    fn put(&mut self,
           key: Self::K,
           value: &[u8]);

    /// Callback for deleted items
    fn deleted(&mut self,
               key: Self::K);
}

extern "C" fn put_callback<K: Key, T: WritebatchIterator<K = K>>(
  state: *mut c_void,
  key: *const u8,
  keylen: size_t,
  val: *const u8,
  vallen: size_t
) {
    unsafe {
        let iter: &mut T = &mut *(state as *mut T);
        let key_slice = slice::from_raw_parts::<u8>(key, keylen as usize);
        let val_slice = slice::from_raw_parts::<u8>(val, vallen as usize);
        let k = from_u8::<<T as WritebatchIterator>::K>(key_slice);
        iter.put(k, val_slice);
    }
}

extern "C" fn deleted_callback<K: Key, T: WritebatchIterator<K = K>>(
  state: *mut c_void,
  key: *const u8,
  keylen: size_t
) {
    unsafe {
        let iter: &mut T = &mut *(state as *mut T);
        let key_slice = slice::from_raw_parts::<u8>(key, keylen as usize);
        let k = from_u8::<<T as WritebatchIterator>::K>(key_slice);
        iter.deleted(k);
    }
}
