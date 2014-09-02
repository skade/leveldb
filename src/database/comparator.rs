use cbits::leveldb::*;
use libc::{size_t,c_char,c_void};
use std::mem::transmute;
use std::raw::Slice;

struct ComparatorState {
  name: &'static str,
  cmp: |&[u8], &[u8]|: 'static -> Ordering,
}

pub struct Comparator {
  #[allow(dead_code)] //used in c-space
  state: ComparatorState,
  pub comparator: *mut leveldb_comparator_t,
}

#[unsafe_destructor]
impl Drop for Comparator {
  fn drop(&mut self) {
    unsafe { leveldb_comparator_destroy(self.comparator) };
  }
}

impl Comparator {
  pub fn new(name: &'static str,
             cmp: |&[u8], &[u8]|: 'static -> Ordering)
             -> Comparator {
    unsafe {
      let mut state = ComparatorState { name: name, cmp: cmp };
      let comparator = leveldb_comparator_create(
        &mut state as *mut _ as *mut c_void,
        destructor_callback,
        compare_callback,
        name_callback
      );

      Comparator { state: state, comparator: comparator }
    }
  }

  pub fn comparator(&self) -> *mut leveldb_comparator_t {
    self.comparator
  }
}

#[allow(dead_code)]
extern "C" fn destructor_callback(_state: *mut c_void) {
  // Do nothing
}

extern "C" fn compare_callback(state: *mut c_void,
           a: *const c_char, alen: size_t,
           b: *const c_char, blen: size_t) -> int {
  unsafe {
    let s: &mut ComparatorState = &mut *(state as *mut ComparatorState);

    let a_slice = transmute(Slice {
      data: a,
      len: alen as uint,
    });

    let b_slice = transmute(Slice {
      data: b,
      len: blen as uint,
    });

    match (s.cmp)(a_slice, b_slice) {
      Less    => -1,
      Equal   => 0,
      Greater => 1
    }
  }
}

extern "C" fn name_callback(state: *mut c_void) -> &'static str {
  unsafe {
    let s: &mut ComparatorState = &mut *(state as *mut ComparatorState);
    s.name
  }
}
