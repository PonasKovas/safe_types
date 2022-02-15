use crate::{Immutable, Mutable, PhantomType};

#[repr(C)]
pub struct SBox<T> {
    ptr: *mut T,
    // Used so the compiler would trigger the improper_ctypes_definitions lint
    // if T is not FFI-safe
    _phantom: PhantomType<T>,
}

impl<T> SBox<T> {
    pub fn from_box(b: Box<T>) -> Self {
        Self {
            ptr: Box::into_raw(b),
            _phantom: PhantomType::new(),
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
