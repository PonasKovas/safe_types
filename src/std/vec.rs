use crate::{Immutable, Mutable, PhantomType};
use core::slice;
use std::mem::forget;

#[cfg(feature = "convenient_methods")]
use safe_types_derive::impl_methods;

/// FFI-safe equivalent of `Vec<T>`
///
/// See documentation of [`std::vec::Vec`]
#[repr(C)]
pub struct SVec<T> {
    ptr: *mut T,
    length: usize,
    capacity: usize,
    // So the compiler would trigger the improper_ctypes_definitions lint
    // if T is not FFI-safe
    _phantom: PhantomType<T>,
}

impl<T> SVec<T> {
    pub fn from_vec(mut v: Vec<T>) -> Self {
        let r = Self {
            ptr: v.as_mut_ptr(),
            length: v.len(),
            capacity: v.capacity(),
            _phantom: PhantomType::new(),
        };

        forget(v);

        r
    }
    pub fn into_vec(self) -> Vec<T> {
        let r = unsafe { Vec::from_raw_parts(self.ptr, self.length, self.capacity) };

        forget(self);

        r
    }
    pub fn as_vec<'a>(&'a self) -> Immutable<'a, Vec<T>> {
        Immutable::new_from(self)
    }
    pub fn as_vec_mut<'a>(&'a mut self) -> Mutable<'a, SVec<T>, Vec<T>> {
        Mutable::new_from(self)
    }
}

impl<T> SVec<T> {
    pub fn new() -> Self {
        Self::from_vec(Vec::new())
    }
    pub fn as_ptr(&self) -> *const T {
        self.ptr
    }
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    pub fn len(&self) -> usize {
        self.length
    }
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.ptr, self.length) }
    }
    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.ptr, self.length) }
    }
}

#[cfg(feature = "convenient_methods")]
impl<T> SVec<T> {
    impl_methods!(into_vec, as_vec, as_vec_mut, [
		fn append(&mut self, other: &mut Vec<T>);
		fn as_mut_ptr(&mut self) -> *mut T;
	    fn clear(&mut self);
	    fn dedup_by<F>(&mut self, same_bucket: F) where F: FnMut(&mut T, &mut T) -> bool;
	    fn dedup_by_key<F, K>(&mut self, key: F) where F: FnMut(&mut T) -> K, K: PartialEq<K>;
	    fn insert(&mut self, index: usize, element: T);
	    fn into_boxed_slice(self) -> Box<[T]>;
	    fn is_empty(&self) -> bool;
	    fn leak<'a>(self) -> &'a mut [T];
	    fn pop(&mut self) -> Option<T>;
	    fn push(&mut self, value: T);
	    fn remove(&mut self, index: usize) -> T;
	    fn reserve(&mut self, additional: usize);
	    fn reserve_exact(&mut self, additional: usize);
	    fn resize_with<F>(&mut self, new_len: usize, f: F) where F: FnMut() -> T;
	    fn retain<F>(&mut self, f: F) where F: FnMut(&T) -> bool;
	    unsafe fn set_len(&mut self, new_len: usize);
	    fn shrink_to(&mut self, min_capacity: usize);
	    fn shrink_to_fit(&mut self);
	    fn split_off(&mut self, at: usize) -> Vec<T>;
	    fn swap_remove(&mut self, index: usize) -> T;
	    fn truncate(&mut self, len: usize);
	    fn try_reserve(&mut self, additional: usize) -> Result<(), ::std::collections::TryReserveError>;
		fn try_reserve_exact(&mut self, additional: usize) -> Result<(), ::std::collections::TryReserveError>;
	]);
}
#[cfg(feature = "convenient_methods")]
impl<T: PartialEq<T>> SVec<T> {
    impl_methods!(into_vec, as_vec, as_vec_mut, [
        fn dedup(&mut self);
    ]);
}
#[cfg(feature = "convenient_methods")]
impl<T: Clone> SVec<T> {
    impl_methods!(into_vec, as_vec, as_vec_mut, [
        fn extend_from_slice(&mut self, other: &[T]);
        fn extend_from_within<R>(&mut self, src: R) where R: ::std::ops::RangeBounds<usize>;
        fn resize(&mut self, new_len: usize, value: T);
    ]);
}

impl<T> From<Vec<T>> for SVec<T> {
    fn from(v: Vec<T>) -> Self {
        Self::from_vec(v)
    }
}
impl<T> From<SVec<T>> for Vec<T> {
    fn from(v: SVec<T>) -> Self {
        v.into_vec()
    }
}

unsafe impl<T> Send for SVec<T> {}
unsafe impl<T> Sync for SVec<T> {}
