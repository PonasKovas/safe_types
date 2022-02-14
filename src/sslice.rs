use std::ops::{Deref, DerefMut};

/// FFI-safe equivalent of `&[T]`
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SSlice<'a, T> {
    ptr: &'a T,
    length: usize,
}

/// FFI-safe equivalent of `&mut [T]`
#[repr(C)]
pub struct SMutSlice<'a, T> {
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

impl<'a, T> SMutSlice<'a, T> {
    pub fn from_slice(slice: &'a mut [T]) -> Self {
        Self {
            ptr: unsafe { slice.as_ptr().as_ref().unwrap_unchecked() },
            length: slice.len(),
        }
    }
    pub fn as_slice(&self) -> &'a mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr as *const _ as *mut _, self.length) }
    }
}

impl<'a, T> Deref for SMutSlice<'a, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<'a, T> DerefMut for SMutSlice<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_slice()
    }
}

impl<'a, T> From<&'a [T]> for SSlice<'a, T> {
    fn from(s: &'a [T]) -> Self {
        Self::from_slice(s)
    }
}
impl<'a, T> From<&'a mut [T]> for SSlice<'a, T> {
    fn from(s: &'a mut [T]) -> Self {
        Self::from_slice(s)
    }
}

impl<'a, T> From<SSlice<'a, T>> for &'a [T] {
    fn from(s: SSlice<'a, T>) -> Self {
        s.as_slice()
    }
}

impl<'a, T> From<&'a mut [T]> for SMutSlice<'a, T> {
    fn from(s: &'a mut [T]) -> Self {
        Self::from_slice(s)
    }
}

impl<'a, T> From<SMutSlice<'a, T>> for &'a [T] {
    fn from(s: SMutSlice<'a, T>) -> Self {
        s.as_slice()
    }
}
impl<'a, T> From<SMutSlice<'a, T>> for &'a mut [T] {
    fn from(s: SMutSlice<'a, T>) -> Self {
        s.as_slice()
    }
}
