pub mod leveldb {
  //dead_code only allowed temporary, to avoid noise during
  //construction work
  #![allow(non_camel_case_types,dead_code)]
  use libc::{c_int,c_char,size_t,c_void};

  pub struct leveldb_t;
  pub struct leveldb_cache_t;
  pub struct leveldb_comparator_t;
  pub struct leveldb_env_t;
  pub struct leveldb_filelock_t;
  pub struct leveldb_filterpolicy_t;
  pub struct leveldb_iterator_t;
  pub struct leveldb_logger_t;
  pub struct leveldb_options_t;
  pub struct leveldb_randomfile_t;
  pub struct leveldb_readoptions_t;
  pub struct leveldb_seqfile_t;
  pub struct leveldb_snapshot_t;
  pub struct leveldb_writablefile_t;
  pub struct leveldb_writebatch_t;
  pub struct leveldb_writeoptions_t;

  pub enum Compression {
    No = 0,
    Snappy = 1
  }

  pub type destructor_fn = extern "C" fn(obj: *mut c_void);
  pub type comparator_fn = extern "C" fn(
    state: *mut c_void,
    a: *mut c_char, alen: size_t,
    b: *mut c_char, blen: size_t) -> int;
  pub type name_fn = extern "C" fn(
    state: *mut c_void
  ) -> &'static str;

  #[link(name = "leveldb")]
  #[link(name = "stdc++")]
  extern {
    pub fn leveldb_open(options: *mut leveldb_options_t,
                        name: *const c_char,
                        errptr: &mut *const c_char) -> *mut leveldb_t;
    pub fn leveldb_close(database: *mut leveldb_t);
    pub fn leveldb_put(database: *mut leveldb_t,
                       options: *mut leveldb_writeoptions_t,
                       key: *mut c_char,
                       keylen: size_t,
                       val: *mut c_char,
                       vallen: size_t,
                       errptr: &mut *const c_char);

    pub fn leveldb_delete(database: *mut leveldb_t,
                          options: *mut leveldb_writeoptions_t,
                          key: *mut c_char,
                          keylen: size_t,
                          errptr: &mut *const c_char);

    pub fn leveldb_get(database: *mut leveldb_t,
                       options: *mut leveldb_readoptions_t,
                       key: *mut c_char,
                       keylen: size_t,
                       vallen: *const size_t,
                       errptr: &mut *const c_char) -> *const u8;

    pub fn leveldb_options_create() -> *mut leveldb_options_t;
    pub fn leveldb_options_destroy(options: *mut leveldb_options_t);
    pub fn leveldb_options_set_comparator(cache: *mut leveldb_cache_t,
                                          options: *mut leveldb_comparator_t);
    pub fn leveldb_options_set_filter_policy(options: *mut leveldb_options_t,
                                             filter_policy: *mut leveldb_filterpolicy_t);
    pub fn leveldb_options_set_create_if_missing(options: *mut leveldb_options_t,
                                                 create: c_char);
    pub fn leveldb_options_set_error_if_exists(options: *mut leveldb_options_t,
                                               error: c_char);
    pub fn leveldb_options_set_paranoid_checks(options: *mut leveldb_options_t,
                                               checks: c_char);
    pub fn leveldb_options_set_env(options: *mut leveldb_options_t,
                                   env: *mut leveldb_env_t);
    pub fn leveldb_options_set_info_log(options: *mut leveldb_options_t,
                                        logger: *mut leveldb_logger_t);
    pub fn leveldb_options_set_write_buffer_size(options: *mut leveldb_options_t,
                                                 size: size_t);
    pub fn leveldb_options_set_max_open_files(options: *mut leveldb_options_t,
                                              number: c_int);
    pub fn leveldb_options_set_cache(options: *mut leveldb_options_t,
                                     cache: *mut leveldb_cache_t);
    pub fn leveldb_options_set_block_size(options: *mut leveldb_options_t,
                                          block_size: size_t);
    pub fn leveldb_options_set_block_restart_interval(options: *mut leveldb_options_t,
                                                      interval: c_int);
    pub fn leveldb_options_set_compression(options: *mut leveldb_options_t,
                                           compression_level: Compression);

    pub fn leveldb_writeoptions_create() -> *mut leveldb_writeoptions_t;
    pub fn leveldb_writeoptions_destroy(options: *mut leveldb_writeoptions_t);
    pub fn leveldb_writeoptions_set_sync(options: *mut leveldb_writeoptions_t,
                                         sync: c_char);

    pub fn leveldb_readoptions_create() -> *mut leveldb_readoptions_t;
    pub fn leveldb_readoptions_destroy(options: *mut leveldb_readoptions_t);
    pub fn leveldb_readoptions_set_verify_checksums(options: *mut leveldb_readoptions_t,
                                                    verify_checksums: c_char);
    pub fn leveldb_readoptions_set_fill_cache(options: *mut leveldb_readoptions_t,
                                              fill_cache: c_char);
    pub fn leveldb_readoptions_set_snapshot(options: *mut leveldb_readoptions_t,
                                            snapshot: *mut leveldb_snapshot_t);

    pub fn leveldb_create_iterator(database: *mut leveldb_t,
                                   options: *mut leveldb_readoptions_t) -> *mut leveldb_iterator_t;
    pub fn leveldb_iter_destroy(iterator: *mut leveldb_iterator_t);
    pub fn leveldb_iter_valid(iterator: *mut leveldb_iterator_t) -> c_char;
    pub fn leveldb_iter_seek_to_first(iterator: *mut leveldb_iterator_t);
    pub fn leveldb_iter_seek_to_last(iterator: *mut leveldb_iterator_t);
    pub fn leveldb_iter_seek(iterator: *mut leveldb_iterator_t,
                             key: *mut c_char,
                             keylen: size_t);
    pub fn leveldb_iter_next(iterator: *mut leveldb_iterator_t);
    pub fn leveldb_iter_prev(iterator: *mut leveldb_iterator_t);
    pub fn leveldb_iter_key(iterator: *mut leveldb_iterator_t,
                            keylen: *mut size_t) -> *mut c_char;
    pub fn leveldb_iter_value(iterator: *mut leveldb_iterator_t,
                              vallen: *const size_t) -> *const c_char;
    pub fn leveldb_iter_get_error(iterator: *mut leveldb_iterator_t,
                                  errptr: &mut *const c_char);

    pub fn leveldb_comparator_create(state: *mut c_void,
                                     destructor: destructor_fn,
                                     comparator: comparator_fn,
                                     name: name_fn);
    pub fn leveldb_comparator_destroy(comparator: *mut leveldb_comparator_t);
    pub fn leveldb_free(ptr: *mut c_void);
  }
}