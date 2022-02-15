use crate::PhantomType;
use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut, Index, IndexMut},
};

/// FFI-safe equivalent of `&[T]`
///
/// See documentation of [`slice`]
#[derive(Clone)]
#[repr(C)]
pub struct SSlice<'a, T> {
    ptr: *const T,
    length: usize,
    // So the compiler would trigger the improper_ctypes_definitions lint
    // if T is not FFI-safe
    _phantom_t: PhantomType<T>,
    _phantom_d: PhantomData<&'a T>,
}

/// FFI-safe equivalent of `&mut [T]`
///
/// See documentation of [`slice`]
#[repr(C)]
pub struct SMutSlice<'a, T> {
    ptr: *mut T,
    length: usize,
    // So the compiler would trigger the improper_ctypes_definitions lint
    // if T is not FFI-safe
    _phantom_t: PhantomType<T>,
    _phantom_d: PhantomData<&'a T>,
}

impl<'a, T> SSlice<'a, T> {
    pub fn from_slice(slice: &[T]) -> Self {
        Self {
            ptr: unsafe { slice.as_ptr().as_ref().unwrap_unchecked() },
            length: slice.len(),
            _phantom_t: PhantomType::new(),
            _phantom_d: PhantomData,
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
            ptr: slice.as_mut_ptr(),
            length: slice.len(),
            _phantom_t: PhantomType::new(),
            _phantom_d: PhantomData,
        }
    }
    pub fn into_slice(self) -> &'a mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.length) }
    }
    pub fn as_slice<'b>(&'b self) -> &'b [T] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.length) }
    }
    pub fn as_slice_mut<'b>(&'b mut self) -> &'b mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.length) }
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
        self.as_slice_mut()
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
        s.into_slice()
    }
}
impl<'a, T> From<SMutSlice<'a, T>> for &'a mut [T] {
    fn from(s: SMutSlice<'a, T>) -> Self {
        s.into_slice()
    }
}

impl<'a, T, I> Index<I> for SSlice<'a, T>
where
    I: std::slice::SliceIndex<[T]>,
{
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &I::Output {
        self.as_slice().index(index)
    }
}

impl<'a, T, I> Index<I> for SMutSlice<'a, T>
where
    I: std::slice::SliceIndex<[T]>,
{
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &I::Output {
        self.as_slice().index(index)
    }
}

impl<'a, T, I> IndexMut<I> for SMutSlice<'a, T>
where
    I: std::slice::SliceIndex<[T]>,
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut I::Output {
        self.as_slice_mut().index_mut(index)
    }
}

// TODO more trait impls
