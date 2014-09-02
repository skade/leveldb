use cbits::leveldb::*;
use libc::{size_t,c_void};
use libc;
use std::mem;
use std::slice;

pub trait Comparator {
     fn name(&self) -> *const u8;
     fn compare(&self, a: &[u8], b: &[u8]) -> i32;
}

extern "C" fn name<T: Comparator>(state: *mut libc::c_void) -> *const u8 {
     let x: &T = unsafe { &*(state as *mut T) };
     x.name()
}

extern "C" fn compare<T: Comparator>(state: *mut libc::c_void,
                                     a: *const u8, a_len: size_t,
                                     b: *const u8, b_len: size_t) -> i32 {
     unsafe {
         slice::raw::buf_as_slice(a, a_len as uint, |a_slice| {
              slice::raw::buf_as_slice(b, b_len as uint, |b_slice| {
                   let x: &T = &*(state as *mut T);
                   x.compare(a_slice, b_slice)
              })
         })
     }
}

extern "C" fn destructor<T>(state: *mut libc::c_void) {
     let _x: Box<T> = unsafe {mem::transmute(state)};
     // let the Box fall out of scope and run the T's destructor
}

pub fn create_comparator<T: Comparator>(x: Box<T>) -> *mut leveldb_comparator_t {
     unsafe {
          leveldb_comparator_create(mem::transmute(x),
                                    destructor::<T>,
                                    compare::<T>,
                                    name::<T>)
     }
}

