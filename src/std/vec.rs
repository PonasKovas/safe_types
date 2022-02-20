use crate::{Immutable, Mutable};
use core::slice;
use std::{
    fmt::Debug,
    mem::forget,
    ops::{Index, IndexMut},
};

#[cfg(feature = "convenient_methods")]
use safe_types_derive::impl_methods;

/// FFI-safe equivalent of `Vec<T>`
///
/// See documentation of [`std::vec::Vec`]
///
/// *Note: due to rust's limitations, using this type will never trigger
/// the `improper_ctypes_definitions` lint, see https://github.com/rust-lang/rust/issues/94000 *
#[repr(C)]
pub struct SVec<T> {
    ptr: *mut T,
    length: usize,
    capacity: usize,
}

impl<T> SVec<T> {
    pub fn from_vec(mut v: Vec<T>) -> Self {
        let r = Self {
            ptr: v.as_mut_ptr(),
            length: v.len(),
            capacity: v.capacity(),
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
    pub fn as_vec_mut<'a>(&'a mut self) -> Mutable<'a, Self, Vec<T>> {
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

impl<T: Debug> Debug for SVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&*self.as_vec(), f)
    }
}

impl<T: Clone> Clone for SVec<T> {
    fn clone(&self) -> Self {
        (*self.as_vec()).clone().into()
    }
}

impl<T: PartialEq> PartialEq for SVec<T> {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&*self.as_vec(), &*other.as_vec())
    }
}

impl<T> Index<usize> for SVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl<T> IndexMut<usize> for SVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut_slice()[index]
    }
}

impl<'a, T> IntoIterator for &'a SVec<T> {
    type Item = &'a T;

    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().into_iter()
    }
}
