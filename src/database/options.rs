use cbits::leveldb::*;

use libc::{size_t};

pub struct Options {
  pub create_if_missing: bool,
  pub error_if_exists: bool,
  pub paranoid_checks: bool,
  pub write_buffer_size: Option<size_t>,
  pub max_open_files: Option<i32>,
  pub block_size: Option<size_t>,
  pub block_restart_interval: Option<i32>,
  pub compression: Compression,
}

impl Options {
  pub fn new() -> Options {
    Options {
      create_if_missing: false,
      error_if_exists: false,
      paranoid_checks: false,
      write_buffer_size: None,
      max_open_files: None,
      block_size: None,
      block_restart_interval: None,
      compression: Compression::No
    }
  }
}

pub struct WriteOptions {
  pub sync: bool
}

impl WriteOptions {
  pub fn new() -> WriteOptions {
    WriteOptions { sync: true }
  }
}

pub struct ReadOptions {
  options: *mut   leveldb_readoptions_t,
}

impl ReadOptions {
  pub fn new() -> ReadOptions {
    unsafe {
      let options = leveldb_readoptions_create();
      ReadOptions { options: options }
    }
  }

  pub fn options(&self) -> *mut leveldb_readoptions_t {
    self.options
  }

  pub fn verify_checksums(&mut self, verify_checksums: bool) {
    unsafe { leveldb_readoptions_set_verify_checksums(self.options, verify_checksums as i8); }
  }

  pub fn fill_cache(&mut self, fill_cache: bool) {
     unsafe { leveldb_readoptions_set_fill_cache(self.options, fill_cache as i8); }
  }
}

impl Drop for ReadOptions {
  fn drop(&mut self) {
    unsafe {
      leveldb_readoptions_destroy(self.options);
    }
  }
}
