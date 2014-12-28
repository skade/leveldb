//! All the option types needed for interfacing with leveldb.
//!
//! Those are:
//! * `Options`: used when opening a database
//! * `ReadOptions`: used when reading from leveldb
//! * `WriteOptions`: used when writng to leveldb
use cbits::leveldb::{leveldb_options_create,leveldb_writeoptions_create,leveldb_readoptions_create,
                     leveldb_options_t,leveldb_writeoptions_t,leveldb_readoptions_t,leveldb_comparator_t,
                     leveldb_options_set_compression,leveldb_options_set_create_if_missing,
                     leveldb_options_set_error_if_exists,leveldb_options_set_paranoid_checks,
                     leveldb_options_set_write_buffer_size,leveldb_options_set_block_size,
                     leveldb_options_set_max_open_files,leveldb_options_set_block_restart_interval,
                     leveldb_options_set_comparator,leveldb_writeoptions_set_sync,
                     leveldb_readoptions_set_verify_checksums,leveldb_readoptions_set_fill_cache, leveldb_readoptions_set_snapshot,
                     leveldb_snapshot_t,
                     Compression};

use libc::{size_t};

/// Options to consider when opening a new or pre-existing database.
///
/// Note that in contrast to the leveldb C API, the Comparator is not
/// passed using this structure.
///
/// For more detailed explanations, consider the
/// [leveldb documentation](https://github.com/google/leveldb/tree/master/doc)
#[deriving(Copy)]
pub struct Options {
  /// create the database if missing
  ///
  /// default: false
  pub create_if_missing: bool,
  /// report an error if the DB already exists instead of opening.
  ///
  /// default: false
  pub error_if_exists: bool,
  /// paranoid checks make the database report an error as soon as
  /// corruption is detected.
  ///
  /// default: false
  pub paranoid_checks: bool,
  /// Override the size of the write buffer to use.
  ///
  /// default: None
  pub write_buffer_size: Option<size_t>,
  /// Override the max number of open files.
  ///
  /// default: None
  pub max_open_files: Option<i32>,
  /// Override the size of the blocks leveldb uses for writing and caching.
  ///
  /// default: None
  pub block_size: Option<size_t>,
  /// Override the interval between restart points.
  ///
  /// default: None
  pub block_restart_interval: Option<i32>,
  /// Define whether leveldb should write compressed or not.
  ///
  /// default: Compression::No
  pub compression: Compression,
}

impl Options {
  /// Create a new `Options` struct with default settings.
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

/// The write options to use for a write operation.
#[deriving(Copy)]
pub struct WriteOptions {
  /// `fsync` before acknowledging a write operation.
  ///
  /// default: false
  pub sync: bool
}

impl WriteOptions {
  /// Return a new `WriteOptions` struct with default settings.
  pub fn new() -> WriteOptions {
    WriteOptions { sync: false }
  }
}

/// The read options to use for any read operation.
#[allow(missing_copy_implementations)]
pub struct ReadOptions {
  /// Whether to verify the saved checksums on read.
  ///
  /// default: false
  pub verify_checksums: bool,
  /// Whether to fill the internal cache with the
  /// results of the read.
  ///
  /// default: true
  pub fill_cache: bool,
  /// An optional snapshot to base this operation on.
  ///
  /// Consider using the `Snapshot` trait instead of setting
  /// this yourself.
  ///
  /// default: None
  pub snapshot: Option<*mut leveldb_snapshot_t>
}

impl ReadOptions {
  /// Return a `ReadOptions` struct with the default values.
  pub fn new() -> ReadOptions {
    ReadOptions { verify_checksums: false,
                  fill_cache: true,
                  snapshot: None }
  }
}

#[allow(missing_docs)]
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

#[allow(missing_docs)]
pub unsafe fn c_writeoptions(options: WriteOptions) -> *mut leveldb_writeoptions_t {
  let c_writeoptions = leveldb_writeoptions_create();
  leveldb_writeoptions_set_sync(c_writeoptions, options.sync as i8);
  c_writeoptions
}

#[allow(missing_docs)]
pub unsafe fn c_readoptions(options: &ReadOptions) -> *mut leveldb_readoptions_t {
  let c_readoptions = leveldb_readoptions_create();
  leveldb_readoptions_set_verify_checksums(c_readoptions, options.verify_checksums as i8);
  leveldb_readoptions_set_fill_cache(c_readoptions, options.fill_cache as i8);

  if let Some(snapshot) = options.snapshot {
    leveldb_readoptions_set_snapshot(c_readoptions, snapshot);
  }
  c_readoptions
}

