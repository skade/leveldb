#[crate_type = "lib"];
#[crate_id = "leveldb"];
#[feature(globs,phase)];
#[phase(syntax, link)] extern crate log;

use cbits::leveldb::*;
use std::ptr;
use std::libc::{c_char, size_t, c_int};
use std::c_str::*;
use std::str::raw::*;
use std::bool::to_bit;
use std::slice::*;

mod cbits;

pub struct Database {
  priv database: *leveldb_t,
}

pub struct Error {
  message: ~str
}

// I am  pretty sure this is a memory leak
//impl Drop for Error {
//  fn drop(&mut self) {
//    unsafe { leveldb_free(self.message) }
//  }
//}

impl Database {
  fn new(database: *leveldb_t) -> Database {
    Database { database: database }
  }

  pub fn open(name: ~str, options: Options) -> Result<Database,Error> {
    name.with_c_str(|c_string| {
      unsafe {
        let error = ptr::null();
        let database = leveldb_open(options.opts, c_string, &error);
        if error == ptr::null() {
          Ok(Database::new(database))
        } else {
          Err(Error { message: from_c_str(error) }) 
        }
      }
    }) 
  }

  pub fn put(&mut self,
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

  pub fn delete(&mut self,
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

  pub fn get(&mut self,
             options: ReadOptions,
             key: &[u8]) -> Result<Option<~[i8]>, Error> {
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
           let vec: ~[i8] = from_buf(result, length as uint);
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

pub struct Options {
  priv opts: *leveldb_options_t,
}

impl Options {
  pub fn new() -> Options {
    unsafe {
      let opts = leveldb_options_create();
      Options { opts: opts }
    }
  }

  pub fn create_if_missing(&mut self, create: bool) {
    unsafe { leveldb_options_set_create_if_missing(self.opts, to_bit::<i8>(create)) }
  }

  pub fn error_if_exists(&mut self, error: bool) {
    unsafe { leveldb_options_set_error_if_exists(self.opts, to_bit::<i8>(error)) }
  }

  pub fn paranoid_checks(&mut self, paranoid: bool) {
    unsafe { leveldb_options_set_paranoid_checks(self.opts, to_bit::<i8>(paranoid)) }
  }

  pub fn write_buffer_size(&mut self, buffer_size: size_t) {
    unsafe { leveldb_options_set_write_buffer_size(self.opts, buffer_size) }
  }

  pub fn max_open_files(&mut self, number: int) {
    unsafe { leveldb_options_set_max_open_files(self.opts, number as c_int) }
  }

  pub fn block_size(&mut self, block_size: size_t) {
    unsafe { leveldb_options_set_block_size(self.opts, block_size) }
  }

  pub fn block_restart_interval(&mut self, block_restart_interval: int) {
    unsafe { leveldb_options_set_block_restart_interval(self.opts, block_restart_interval as c_int) }
  }

  pub fn compression(&mut self, compression: Compression) {
    unsafe { leveldb_options_set_compression(self.opts, compression) }
  }
}

impl Drop for Options {
  fn drop(&mut self) {
    unsafe {
      leveldb_options_destroy(self.opts);
    }
  }
}

pub struct WriteOptions {
  priv options: *leveldb_writeoptions_t,
}

impl WriteOptions {
  pub fn new() -> WriteOptions {
    unsafe {
      let options = leveldb_writeoptions_create();
      WriteOptions { options: options }
    }
  }

  pub fn sync(&mut self, sync: bool) {
    unsafe { leveldb_writeoptions_set_sync(self.options, to_bit::<i8>(sync)) }
  }
}

impl Drop for WriteOptions {
  fn drop(&mut self) {
    unsafe {
      leveldb_writeoptions_destroy(self.options);
    }
  }
}

pub struct ReadOptions {
  priv options: *leveldb_readoptions_t,
}

impl ReadOptions {
  pub fn new() -> ReadOptions {
    unsafe {
      let options = leveldb_readoptions_create();
      ReadOptions { options: options }
    }
  }

  pub fn verify_checksums(&mut self, verify_checksums: bool) {
    unsafe { leveldb_readoptions_set_verify_checksums(self.options, to_bit::<i8>(verify_checksums)); }
  }

  pub fn fill_cache(&mut self, fill_cache: bool) {
     unsafe { leveldb_readoptions_set_fill_cache(self.options, to_bit::<i8>(fill_cache)); }
  }
}

impl Drop for ReadOptions {
  fn drop(&mut self) {
    unsafe {
      leveldb_readoptions_destroy(self.options);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{Options,ReadOptions,WriteOptions,Database};

  #[test]
  fn test_create_options() {
    Options::new();
  }

  #[test]
  fn test_open_database() {
    let mut opts = Options::new();
    opts.create_if_missing(true);
    let res = Database::open(~"testdbs/create_if_missing", opts);
    assert!(res.is_ok());
  }

  #[test]
  fn test_open_non_existant_database_without_create() {
    let mut opts = Options::new();
    opts.create_if_missing(false);
    let res = Database::open(~"testdbs/missing", opts);
    assert!(res.is_err());
  }

  #[test]
  fn test_write_to_database() {
    let mut opts = Options::new();
    opts.create_if_missing(true);
    let mut database = match Database::open(~"testdbs/put_simple", opts) {
      Ok(db) => { db },
      Err(_) => { fail!("failed to open database") }
    };
    let write_opts = WriteOptions::new();
    let result = database.put(write_opts,
                              &[1],
                              &[1]);
    assert!(result.is_ok());
  }

  #[test]
  fn test_delete_from_database() {
    let mut opts = Options::new();
    opts.create_if_missing(true);
    let mut database = match Database::open(~"testdbs/delete_simple", opts) {
      Ok(db) => { db },
      Err(_) => { fail!("failed to open database") }
    };
    let write_opts = WriteOptions::new();
    let result = database.put(write_opts,
                              &[1],
                              &[1]);
    assert!(result.is_ok());
    let write2 = WriteOptions::new();
    let res2 = database.delete(write2,
                               &[1]);
    assert!(res2.is_ok());
  }

  #[test]
  fn test_get_from_empty_database() {
    let mut opts = Options::new();
    opts.create_if_missing(true);
    let mut database = match Database::open(~"testdbs/get_simple", opts) {
      Ok(db) => { db },
      Err(_) => { fail!("failed to open database") }
    };
    let read_opts = ReadOptions::new();
    let res = database.get(read_opts, [1,2,3]);
    match res {
      Ok(data) => { assert!(data.is_none()) },
      Err(_) => { fail!("failed reading data") }
    }
  }

  #[test]
  fn test_get_from_filled_database() {
    let mut opts = Options::new();
    opts.create_if_missing(true);
    let mut database = match Database::open(~"testdbs/get_filled", opts) {
      Ok(db) => { db },
      Err(_) => { fail!("failed to open database") }
    };
    let write_opts = WriteOptions::new();
    let result = database.put(write_opts,
                              &[1],
                              &[1]);
    assert!(result.is_ok());
    let read_opts = ReadOptions::new();
    let res = database.get(read_opts,
                            &[1]);
    match res {
      Ok(data) => { 
        assert!(data.is_some())
        let data = data.unwrap();
        assert_eq!(data, ~[1]);
      },
      Err(_) => { fail!("failed reading data") }
    }
  }
}
