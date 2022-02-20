use crate::{Immutable, Mutable};

/// A pointer type for heap allocation.
///
/// See documentation of [`std::boxed::Box`]
///
/// *Note: due to rust's limitations, using this type will never trigger
/// the `improper_ctypes_definitions` lint, see https://github.com/rust-lang/rust/issues/94000 *
#[repr(C)]
pub struct SBox<T> {
    ptr: *mut T,
}

impl<T> SBox<T> {
    pub fn from_box(b: Box<T>) -> Self {
        Self {
            ptr: Box::into_raw(b),
        }
    }
    pub fn into_box(self) -> Box<T> {
        unsafe { Box::from_raw(self.ptr) }
    }
    pub fn as_box<'a>(&'a self) -> Immutable<'a, Box<T>> {
        Immutable::new(unsafe { Box::from_raw(self.ptr) })
    }
    pub fn as_box_mut<'a>(&'a mut self) -> Mutable<'a, Self, Box<T>> {
        Mutable::new(unsafe { std::ptr::read(self).into_box() }, self)
    }
}

impl<T> From<Box<T>> for SBox<T> {
    fn from(b: Box<T>) -> Self {
        Self::from_box(b)
    }
}

impl<T: Clone> Clone for SBox<T> {
    fn clone(&self) -> Self {
        SBox::from_box(Box::new(unsafe { (*self.ptr).clone() }))
    }
}

impl<T> Drop for SBox<T> {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.ptr);
        }
    }
}
