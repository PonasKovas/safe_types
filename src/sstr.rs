use crate::{SMutSlice, SSlice};
use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

/// FFI-safe equivalent of `&str`
#[derive(Clone)]
#[repr(C)]
pub struct SStr<'a> {
    inner: SSlice<'a, u8>,
}

/// FFI-safe equivalent of `&mut str`
#[repr(C)]
pub struct SMutStr<'a> {
    inner: SMutSlice<'a, u8>,
}

impl<'a> SStr<'a> {
    pub fn from_str(s: &'a str) -> Self {
        Self {
            inner: SSlice::from_slice(s.as_bytes()),
        }
    }
    pub fn as_str(&self) -> &'a str {
        unsafe { &std::str::from_utf8_unchecked(self.inner.as_slice()) }
    }
}

impl<'a> SMutStr<'a> {
    pub fn from_str(s: &'a mut str) -> Self {
        Self {
            inner: SMutSlice::from_slice(unsafe { s.as_bytes_mut() }),
        }
    }
    pub fn into_str(self) -> &'a mut str {
        unsafe { std::str::from_utf8_unchecked_mut(self.inner.into_slice()) }
    }
    pub fn as_str<'b>(&'b self) -> &'b str {
        unsafe { &std::str::from_utf8_unchecked(self.inner.as_slice()) }
    }
    pub fn as_str_mut<'b>(&'b mut self) -> &'b mut str {
        unsafe { std::str::from_utf8_unchecked_mut(self.inner.as_slice_mut()) }
    }
}

impl<'a> Deref for SStr<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<'a> Deref for SMutStr<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<'a> DerefMut for SMutStr<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_str_mut()
    }
}

impl<'a> Display for SStr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.as_str(), f)
    }
}

impl<'a> Display for SMutStr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.as_str(), f)
    }
}

impl<'a> Debug for SStr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.as_str(), f)
    }
}

impl<'a> Debug for SMutStr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.as_str(), f)
    }
}

// TODO: add more trait impls
