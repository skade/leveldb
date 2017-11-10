use ::std::slice;

/// Bytes allocated by leveldb
///
/// It's basically the same thing as `Box<[u8]>` except that it uses
/// leveldb_free() as a destructor.
pub struct Bytes {
    // We use static reference instead of pointer to inform the compiler that
    // it can't be null. (Because `NonZero` is unstable now.)
    bytes: &'static mut u8,
    size: usize,
    // Tells the compiler that we own u8
    _marker: ::std::marker::PhantomData<u8>,
}

impl Bytes {
    /// Creates instance of `Bytes` from leveldb-allocated data.
    ///
    /// Returns `None` if `ptr` is `null`.
    pub unsafe fn from_raw(ptr: *mut u8, size: usize) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Bytes {
                bytes: &mut *ptr,
                size: size,
                _marker: Default::default(),
            })
        }
    }

    /// Creates instance of `Bytes` from leveldb-allocated data without null checking.
    pub unsafe fn from_raw_unchecked(ptr: *mut u8, size: usize) -> Self {
        Bytes {
            bytes: &mut *ptr,
            size: size,
            _marker: Default::default(),
        }
    }
}

impl Drop for Bytes {
    fn drop(&mut self) {
        unsafe {
            use libc::c_void;

            ::leveldb_sys::leveldb_free(self.bytes as *mut u8 as *mut c_void);
        }
    }
}

impl ::std::ops::Deref for Bytes {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe {
            slice::from_raw_parts(self.bytes, self.size)
        }
    }
}

impl ::std::ops::DerefMut for Bytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            slice::from_raw_parts_mut(self.bytes as *mut u8, self.size)
        }
    }
}

impl ::std::borrow::Borrow<[u8]> for Bytes {
    fn borrow(&self) -> &[u8] {
        &*self
    }
}

impl ::std::borrow::BorrowMut<[u8]> for Bytes {
    fn borrow_mut(&mut self) -> &mut [u8] {
        &mut *self
    }
}

impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        &*self
    }
}

impl AsMut<[u8]> for Bytes {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut *self
    }
}

impl From<Bytes> for Vec<u8> {
    fn from(bytes: Bytes) -> Self {
        bytes.as_ref().to_owned()
    }
}

impl From<Bytes> for Box<[u8]> {
    fn from(bytes: Bytes) -> Self {
        bytes.as_ref().to_owned().into_boxed_slice()
    }
}
