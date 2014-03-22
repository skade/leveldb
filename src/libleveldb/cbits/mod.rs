pub mod leveldb {
  //dead_code only allowed temporary, to avoid noise during
  //construction work
  #[allow(non_camel_case_types,dead_code)];
  use std::libc::{c_int,c_char,size_t};

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

  extern {
    pub fn leveldb_open(options: *leveldb_options_t,
                        name: *c_char,
                        errptr: **c_char) -> *leveldb_t;
    pub fn leveldb_close(database: *leveldb_t);
    pub fn leveldb_put(database: *leveldb_t,
                       options: *leveldb_writeoptions_t,
                       key: *c_char,
                       keylen: size_t,
                       val: *c_char,
                       vallen: size_t,
                       errptr: **c_char);

    pub fn leveldb_delete(database: *leveldb_t,
                          options: *leveldb_writeoptions_t,
                          key: *c_char,
                          keylen: size_t,
                          errptr: **c_char);

    pub fn leveldb_get(database: *leveldb_t,
                       options: *leveldb_readoptions_t,
                       key: *c_char,
                       keylen: size_t,
                       vallen: *size_t,
                       errptr: **c_char) -> *c_char;

    pub fn leveldb_options_create() -> *leveldb_options_t;
    pub fn leveldb_options_destroy(options: *leveldb_options_t);
    pub fn leveldb_options_set_comparator(cache: *leveldb_cache_t,
                                          options: *leveldb_comparator_t);
    pub fn leveldb_options_set_filter_policy(options: *leveldb_options_t,
                                             filter_policy: *leveldb_filterpolicy_t);
    pub fn leveldb_options_set_create_if_missing(options: *leveldb_options_t,
                                                 create: c_char);
    pub fn leveldb_options_set_error_if_exists(options: *leveldb_options_t,
                                               error: c_char);
    pub fn leveldb_options_set_paranoid_checks(options: *leveldb_options_t,
                                               checks: c_char);
    pub fn leveldb_options_set_env(options: *leveldb_options_t,
                                   env: *leveldb_env_t);
    pub fn leveldb_options_set_info_log(options: *leveldb_options_t,
                                        logger: *leveldb_logger_t);
    pub fn leveldb_options_set_write_buffer_size(options: *leveldb_options_t,
                                                 size: size_t);
    pub fn leveldb_options_set_max_open_files(options: *leveldb_options_t,
                                              number: c_int);
    pub fn leveldb_options_set_cache(options: *leveldb_options_t,
                                     cache: *leveldb_cache_t);
    pub fn leveldb_options_set_block_size(options: *leveldb_options_t,
                                          block_size: size_t);
    pub fn leveldb_options_set_block_restart_interval(options: *leveldb_options_t,
                                                      interval: c_int);
    pub fn leveldb_options_set_compression(options: *leveldb_options_t,         
                                           compression_level: Compression);

    pub fn leveldb_writeoptions_create() -> *leveldb_writeoptions_t;
    pub fn leveldb_writeoptions_destroy(options: *leveldb_writeoptions_t);
    pub fn leveldb_writeoptions_set_sync(options: *leveldb_writeoptions_t,
                                         sync: c_char);

    pub fn leveldb_readoptions_create() -> *leveldb_readoptions_t;
    pub fn leveldb_readoptions_destroy(options: *leveldb_readoptions_t);
    pub fn leveldb_readoptions_set_verify_checksums(options: *leveldb_readoptions_t,
                                                    verify_checksums: c_char);
    pub fn leveldb_readoptions_set_fill_cache(options: *leveldb_readoptions_t,
                                              fill_cache: c_char);
    pub fn leveldb_readoptions_set_snapshot(options: *leveldb_readoptions_t,
                                            snapshot: *leveldb_snapshot_t);
                                        
    pub fn leveldb_create_iterator(database: *leveldb_t,
                                   options: *leveldb_readoptions_t) -> *leveldb_iterator_t;
    pub fn leveldb_iter_destroy(iterator: *leveldb_iterator_t);
    pub fn leveldb_iter_valid(iterator: *leveldb_iterator_t) -> c_char;
    pub fn leveldb_iter_seek_to_first(iterator: *leveldb_iterator_t);
    pub fn leveldb_iter_seek_to_last(iterator: *leveldb_iterator_t);
    pub fn leveldb_iter_seek(iterator: *leveldb_iterator_t,
                             key: *c_char,
                             keylen: size_t);
    pub fn leveldb_iter_next(iterator: *leveldb_iterator_t);
    pub fn leveldb_iter_prev(iterator: *leveldb_iterator_t);
    pub fn leveldb_iter_key(iterator: *leveldb_iterator_t,
                            keylen: *size_t) -> *c_char;
    pub fn leveldb_iter_value(iterator: *leveldb_iterator_t,
                              vallen: *size_t) -> *c_char;
    pub fn leveldb_iter_get_error(iterator: *leveldb_iterator_t,
                                  errptr: **c_char);
    //pub fn leveldb_free(ptr: uint);
  }
}
