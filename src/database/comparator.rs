use cbits::leveldb::*;
use libc::{size_t,c_char,c_void};

pub struct Comparator<S> {
  state: S,
  comparator: *mut leveldb_comparator_t,
  name: &'static str
}

impl<S> Comparator<S> {
  fn new(mut state: S,
         destructor: destructor_fn,
         compare: comparator_fn,
         name: name_fn) {
    unsafe {
      let comparator = leveldb_comparator_create(
        &mut state as *mut _ as *mut c_void,
        destructor,
        compare,
        name
      );

    }
  }
}
//pub trait Comparator<S> {
//  fn new(mut state: S,
//         destructor: destructor_fn,
//         compare: comparator_fn,
//         name: name_fn) {
//    unsafe {
//      leveldb_comparator_create(
//        &mut state as *mut _ as *mut c_void,
//        destructor,
//        compare,
//        name
//      )
//    }
//
//  }
//  fn create(pointer: *mut leveldb_comparator_t);
//  extern "C" fn compare(state: *mut S,
//             a: *mut c_char, alen: size_t,
//             b: *mut c_char, blen: size_t) -> int;
//  extern "C" fn name(state: *mut S) -> &'static str;
//  extern "C" fn destructor(state: *mut S);
//}
