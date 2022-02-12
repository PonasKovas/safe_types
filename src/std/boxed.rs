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
