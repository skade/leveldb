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
    WriteOptions { sync: false }
  }
}

pub struct ReadOptions {
  pub verify_checksums: bool,
  pub fill_cache: bool,
  // TODO: Snapshotting
}

impl ReadOptions {
  pub fn new() -> ReadOptions {
    ReadOptions { verify_checksums: false,
                  fill_cache: true }
  }
}

pub unsafe fn c_options(options: Options, comparator: Option<*mut leveldb_comparator_t>) -> *mut leveldb_options_t {
  let c_options = leveldb_options_create();
  leveldb_options_set_create_if_missing(c_options, options.create_if_missing as i8);
  leveldb_options_set_error_if_exists(c_options, options.error_if_exists as i8);
  leveldb_options_set_paranoid_checks(c_options, options.paranoid_checks as i8);
  if options.write_buffer_size.is_some() {
    leveldb_options_set_write_buffer_size(c_options, options.write_buffer_size.unwrap());
  }
  if options.max_open_files.is_some() {
    leveldb_options_set_max_open_files(c_options, options.max_open_files.unwrap());
  }
  if options.block_size.is_some() {
    leveldb_options_set_block_size(c_options, options.block_size.unwrap());
  }
  if options.block_restart_interval.is_some() {
    leveldb_options_set_block_restart_interval(c_options, options.block_restart_interval.unwrap());
  }
  leveldb_options_set_compression(c_options, options.compression);
  if comparator.is_some() {
    leveldb_options_set_comparator(c_options, comparator.unwrap());
  }
  c_options
}

pub unsafe fn c_writeoptions(options: WriteOptions) -> *mut leveldb_writeoptions_t {
  let c_writeoptions = leveldb_writeoptions_create();
  leveldb_writeoptions_set_sync(c_writeoptions, options.sync as i8);
  c_writeoptions
}

pub unsafe fn c_readoptions(options: ReadOptions) -> *mut leveldb_readoptions_t {
  let c_readoptions = leveldb_readoptions_create();
  leveldb_readoptions_set_verify_checksums(c_readoptions, options.verify_checksums as i8);
  leveldb_readoptions_set_fill_cache(c_readoptions, options.fill_cache as i8);
  c_readoptions
}

