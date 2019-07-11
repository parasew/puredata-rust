use std::slice;
use std::mem::size_of;
use puredata_sys::{getbytes, freebytes};

///A slice allocated and freed using puredata_sys
pub struct Slice<T: 'static + Sized + Copy> (&'static mut [T]);

impl<T> Slice<T>
where T:'static + Copy {
}

impl<T> Default for Slice<T>
where T:'static + Sized + Copy {
    fn default() -> Self {
        unsafe {
            Self (slice::from_raw_parts_mut(std::ptr::null_mut(), 0))
        }
    }
}

impl<T> Slice<T>
where T:'static + Sized + Copy + Default {
    /// Create a new slice.
    ///
    /// # Arguments
    ///
    /// * `len` - the new slice length.
    ///
    /// # Remarks
    ///
    /// This will set all the current content to `Default::default()` for `T`.
    pub fn new(len: usize) -> Self {
        let mut s = Self::default();
        s.resize(len);
        s
    }

    /// Resize the slice.
    ///
    /// # Arguments
    ///
    /// * `len` - the new slice length.
    ///
    /// # Remarks
    ///
    /// This will set all the current content to `Default::default()` for `T`.
    pub fn resize(&mut self, len: usize) {
        //TODO use resizebytes?
        self.cleanup();
        self.alloc(len);
    }

    fn alloc(&mut self, len: usize) {
        unsafe {
            let bytes = puredata_sys::getbytes(len * size_of::<T>());
            let bytes = std::mem::transmute::<_, *mut T>(bytes);
            self.0 = slice::from_raw_parts_mut(bytes, len);
        }
        for i in self.0.iter_mut() {
            *i = Default::default();
        }
    }
}

impl<T> Slice<T>
where T:'static + Sized + Copy  {
    fn cleanup(&mut self) {
        if self.0.len() > 0 {
            unsafe {
                freebytes(self.0.as_mut_ptr() as *mut ::std::os::raw::c_void,
                self.0.len() * size_of::<T>());
                self.0 = slice::from_raw_parts_mut(std::ptr::null_mut(), 0)
            }
        }
    }
}

impl<T> Drop for Slice<T>
where T:'static + Sized + Copy {
    fn drop(&mut self) {
        self.cleanup();
    }
}
