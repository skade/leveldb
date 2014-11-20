use cbits::leveldb::*;
use libc::{size_t,c_void};
use libc;
use std::mem;
use std::slice;
use database::db_key::Key;
use database::db_key::from_u8;

pub trait Comparator<K: Key + Ord> {
     fn name(&self) -> *const u8;
     fn compare(&self, a: &K, b: &K) -> Ordering {
         a.cmp(b)
     }
}

pub struct DefaultComparator;

extern "C" fn name<K: Key + Ord, T: Comparator<K>>(state: *mut libc::c_void) -> *const u8 {
     let x: &T = unsafe { &*(state as *mut T) };
     x.name()
}

extern "C" fn compare<K: Key + Ord, T: Comparator<K>>(state: *mut libc::c_void,
                                     a: *const u8, a_len: size_t,
                                     b: *const u8, b_len: size_t) -> i32 {
     unsafe {
         slice::raw::buf_as_slice(a, a_len as uint, |a_slice| {
              slice::raw::buf_as_slice(b, b_len as uint, |b_slice| {
                   let x: &T = &*(state as *mut T);
                   let a_key = from_u8::<K>(a_slice);
                   let b_key = from_u8::<K>(b_slice);
                   match x.compare(&a_key, &b_key) {
                       Less => -1,
                       Equal => 0,
                       Greater => 1
                   }
              })
         })
     }
}

extern "C" fn destructor<T>(state: *mut libc::c_void) {
     let _x: Box<T> = unsafe {mem::transmute(state)};
     // let the Box fall out of scope and run the T's destructor
}

pub fn create_comparator<K: Key + Ord, T: Comparator<K>>(x: Box<T>) -> *mut leveldb_comparator_t {
     unsafe {
          leveldb_comparator_create(mem::transmute(x),
                                    destructor::<T>,
                                    compare::<K, T>,
                                    name::<K, T>)
     }
}

impl<K: Key + Ord> Comparator<K> for DefaultComparator {
  fn name(&self) -> *const u8 {
    "default_comparator".as_ptr()
  }
}
