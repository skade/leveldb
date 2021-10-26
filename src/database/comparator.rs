//! All keys in leveldb are compared by their binary value unless
//! defined otherwise.
//!
//! Comparators allow to override this comparison.
//! The ordering of keys introduced by the compartor influences iteration order.
//! Databases written with one Comparator cannot be opened with another.
use database::key::Key;
use leveldb_sys::*;
use libc::{c_char, c_void, size_t};
use std::cmp::Ordering;
use std::marker::PhantomData;
use std::slice;

/// A comparator has two important functions:
///
/// * the name function returns a fixed name to detect errors when
///   opening databases with a different name
/// * The comparison implementation
pub trait Comparator<'a> {
    /// The type that the comparator compares.
    type K: Key<'a>;

    /// Return the name of the Comparator
    fn name(&self) -> *const c_char;
    /// compare two keys. This must implement a total ordering.
    fn compare(&self, a: &Self::K, b: &Self::K) -> Ordering;
    /// whether the comparator is the `DefaultComparator`
    fn null() -> bool {
        false
    }
}

/// OrdComparator is a comparator comparing Key<'a>s that implement `Ord`
pub struct OrdComparator<'a, K: Key<'a> + Ord> {
    name: String,
    marker: PhantomData<&'a K>,
}

impl<'a, K: Key<'a> + Ord> OrdComparator<'a, K> {
    /// Create a new OrdComparator
    pub fn new(name: &str) -> OrdComparator<'a, K> {
        OrdComparator {
            marker: PhantomData,
            name: name.to_string(),
        }
    }
}

/// DefaultComparator is the a stand in for "no comparator set"
#[derive(Copy, Clone)]
pub struct DefaultComparator;

unsafe trait InternalComparator<'a>: Comparator<'a>
where
    Self: Sized,
{
    extern "C" fn name(state: *mut c_void) -> *const c_char {
        let x = unsafe { &*(state as *mut Self) };
        x.name()
    }

    extern "C" fn compare(
        state: *mut c_void,
        a: *const c_char,
        a_len: size_t,
        b: *const c_char,
        b_len: size_t,
    ) -> i32 {
        unsafe {
            let a_slice = slice::from_raw_parts::<u8>(a as *const u8, a_len as usize);
            let b_slice = slice::from_raw_parts::<u8>(b as *const u8, b_len as usize);
            let x = &*(state as *mut Self);
            let a_key = <<Self as Comparator>::K>::from(a_slice);
            let b_key = <<Self as Comparator>::K>::from(b_slice);
            match x.compare(&a_key, &b_key) {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => 1,
            }
        }
    }

    extern "C" fn destructor(state: *mut c_void) {
        let _x: Box<Self> = unsafe { Box::from_raw(state as *mut Self) };
        // let the Box fall out of scope and run the T's destructor
    }
}

unsafe impl<'a, C: Comparator<'a>> InternalComparator<'a> for C {}

#[allow(missing_docs)]
pub fn create_comparator<'a, T: Comparator<'a>>(x: Box<T>) -> *mut leveldb_comparator_t {
    unsafe {
        leveldb_comparator_create(
            Box::into_raw(x) as *mut c_void,
            <T as InternalComparator>::destructor,
            <T as InternalComparator>::compare,
            <T as InternalComparator>::name,
        )
    }
}

impl<'a, K: Key<'a> + Ord> Comparator<'a> for OrdComparator<'a, K> {
    type K = K;

    fn name(&self) -> *const c_char {
        let slice: &str = self.name.as_ref();
        slice.as_ptr() as *const c_char
    }

    fn compare(&self, a: &K, b: &K) -> Ordering {
        a.cmp(b)
    }
}

impl<'a> Comparator<'a> for DefaultComparator {
    type K = &'a [u8];

    fn name(&self) -> *const c_char {
        "default_comparator".as_ptr() as *const c_char
    }

    fn compare(&self, _a: &&'a [u8], _b: &&'a [u8]) -> Ordering {
        Ordering::Equal
    }

    fn null() -> bool {
        true
    }
}
