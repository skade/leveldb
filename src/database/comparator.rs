//! All keys in leveldb are compared by their binary value unless
//! defined otherwise.
//!
//! Comparators allow to override this comparison.
//! The ordering of keys introduced by the compartor influences iteration order.
//! Databases written with one Comparator cannot be opened with another.
use cbits::leveldb::*;
use libc::{size_t,c_void};
use libc;
use std::mem;
use std::slice;
use std::cmp::Ordering;
use database::db_key::Key;
use database::db_key::from_u8;
use core::marker::PhantomData;

/// A comparator has two important functions:
///
/// * the name function returns a fixed name to detect errors when
///   opening databases with a different name
/// * The comparison implementation
pub trait Comparator {
     /// comparator key 
     type K: Key;

     /// Return the name of the Comparator
     fn name(&self) -> *const u8;
     /// compare two keys. This must implement a total ordering.
     fn compare(&self, a: &Self::K, b: &Self::K) -> Ordering;
     /// whether the comparator is the `DefaultComparator`
     fn null() -> bool {
         false
     }
}

/// OrdComparator is a comparator comparing Keys that implement `Ord`
#[derive(Copy)]
pub struct OrdComparator<K> {
    marker: PhantomData<K>,
}

impl<K> OrdComparator<K> {
    /// Create a new OrdComparator
    pub fn new() -> OrdComparator<K> {
        OrdComparator { marker: PhantomData }
    }
}
/// DefaultComparator is the a stand in for "no comparator set"
#[derive(Copy)]
pub struct DefaultComparator;

extern "C" fn name<K: Key, T: Comparator>(state: *mut libc::c_void) -> *const u8 {
     let x: &T = unsafe { &*(state as *mut T) };
     x.name()
}

extern "C" fn compare<K: Key, T: Comparator>(state: *mut libc::c_void,
                                     a: *const u8, a_len: size_t,
                                     b: *const u8, b_len: size_t) -> i32 {
     unsafe {
          let a_slice = slice::from_raw_parts::<u8>(a, a_len as usize);
          let b_slice = slice::from_raw_parts::<u8>(b, b_len as usize);
          let x: &T = &*(state as *mut T);
          let a_key = from_u8::<<T as Comparator>::K>(a_slice);
          let b_key = from_u8::<<T as Comparator>::K>(b_slice);
          match x.compare(&a_key, &b_key) {
              Ordering::Less => -1,
              Ordering::Equal => 0,
              Ordering::Greater => 1
          }
     }
}

extern "C" fn destructor<T>(state: *mut libc::c_void) {
     let _x: Box<T> = unsafe {mem::transmute(state)};
     // let the Box fall out of scope and run the T's destructor
}

#[allow(missing_docs)]
pub fn create_comparator<K: Key, T: Comparator<K = K>>(x: Box<T>) -> *mut leveldb_comparator_t {
     unsafe {
          leveldb_comparator_create(mem::transmute(x),
                                    destructor::<T>,
                                    compare::<K, T>,
                                    name::<K, T>)
     }
}

impl<K: Key + Ord> Comparator for OrdComparator<K> {
  type K = K;

  fn name(&self) -> *const u8 {
    use std::intrinsics::get_tydesc;

    let tydesc = unsafe { *get_tydesc::<K>() };
    tydesc.name.as_ptr()
  }
  
  fn compare(&self, a: &K, b: &K) -> Ordering {
    a.cmp(b)
  }
}

impl Comparator for DefaultComparator {
  type K = i32;

  fn name(&self) -> *const u8 {
    "default_comparator".as_ptr()
  }
  
  fn compare(&self, _a: &i32, _b: &i32) -> Ordering {
    Ordering::Equal
  }

  fn null() -> bool {
    true
  }
}
