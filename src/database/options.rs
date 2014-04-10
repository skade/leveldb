use cbits::leveldb::*;

use libc::{size_t, c_int};
use std::bool::to_bit;

pub struct Options {
  options: *leveldb_options_t,
}

impl Options {
  pub fn new() -> Options {
    unsafe {
      let options = leveldb_options_create();
      Options { options: options }
    }
  }

  pub fn options(&self) -> *leveldb_options_t {
    self.options
  }

  pub fn create_if_missing(&mut self, create: bool) {
    unsafe { leveldb_options_set_create_if_missing(self.options, to_bit::<i8>(create)) }
  }

  pub fn error_if_exists(&mut self, error: bool) {
    unsafe { leveldb_options_set_error_if_exists(self.options, to_bit::<i8>(error)) }
  }

  pub fn paranoid_checks(&mut self, paranoid: bool) {
    unsafe { leveldb_options_set_paranoid_checks(self.options, to_bit::<i8>(paranoid)) }
  }

  pub fn write_buffer_size(&mut self, buffer_size: size_t) {
    unsafe { leveldb_options_set_write_buffer_size(self.options, buffer_size) }
  }

  pub fn max_open_files(&mut self, number: int) {
    unsafe { leveldb_options_set_max_open_files(self.options, number as c_int) }
  }

  pub fn block_size(&mut self, block_size: size_t) {
    unsafe { leveldb_options_set_block_size(self.options, block_size) }
  }

  pub fn block_restart_interval(&mut self, block_restart_interval: int) {
    unsafe { leveldb_options_set_block_restart_interval(self.options, block_restart_interval as c_int) }
  }

  pub fn compression(&mut self, compression: Compression) {
    unsafe { leveldb_options_set_compression(self.options, compression) }
  }
}

impl Drop for Options {
  fn drop(&mut self) {
    unsafe {
      leveldb_options_destroy(self.options);
    }
  }
}

pub struct WriteOptions {
  options: *leveldb_writeoptions_t,
}

impl WriteOptions {
  pub fn new() -> WriteOptions {
    unsafe {
      let options = leveldb_writeoptions_create();
      WriteOptions { options: options }
    }
  }

  pub fn options(&self) -> *leveldb_writeoptions_t {
    self.options
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
  options: *leveldb_readoptions_t,
}

impl ReadOptions {
  pub fn new() -> ReadOptions {
    unsafe {
      let options = leveldb_readoptions_create();
      ReadOptions { options: options }
    }
  }

  pub fn options(&self) -> *leveldb_readoptions_t {
    self.options
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
