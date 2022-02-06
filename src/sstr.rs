use crate::SSlice;
use std::ops::Deref;

/// FFI-safe equivalent of a `&str`
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SStr<'a> {
    inner: SSlice<'a, u8>,
}

impl<'a> SStr<'a> {
    pub fn from_str(s: &str) -> Self {
        Self {
            inner: SSlice::from_slice(s.as_bytes()),
        }
    }
    pub fn as_str(&self) -> &'a str {
        unsafe { &std::str::from_utf8_unchecked(self.inner.as_slice()) }
    }
}

impl<'a> Deref for SStr<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

// TODO: add more trait impls
