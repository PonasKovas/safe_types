use std::ops::Deref;

/// FFI-safe equivalent of `&[T]`
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SSlice<'a, T> {
    ptr: &'a T,
    length: usize,
}

impl<'a, T> SSlice<'a, T> {
    pub fn from_slice(slice: &[T]) -> Self {
        Self {
            ptr: unsafe { slice.as_ptr().as_ref().unwrap_unchecked() },
            length: slice.len(),
        }
    }
    pub fn as_slice(&self) -> &'a [T] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.length) }
    }
}

impl<'a, T> Deref for SSlice<'a, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}
