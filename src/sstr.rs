use crate::{std::string::SString, SMutSlice, SSlice};
use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

/// FFI-safe equivalent of `&str`
#[derive(Clone)]
#[repr(C)]
pub struct SStr<'a> {
    pub(crate) inner: SSlice<'a, u8>,
}

/// FFI-safe equivalent of `&mut str`
#[repr(C)]
pub struct SMutStr<'a> {
    pub(crate) inner: SMutSlice<'a, u8>,
}

/// Equivalent of `str`. Unsized and references to it are **not FFI-safe**.
///
/// Needed to mark that `< < SStr<'a> as Deref >::Target as ToOwned >::Owned = SString`
/// instead of the std String
#[repr(transparent)]
pub struct SRawStr {
    pub inner: str,
}

impl SRawStr {
    pub fn from_str<'a>(s: &'a str) -> &'a SRawStr {
        unsafe { std::mem::transmute::<&'a str, &'a SRawStr>(s) }
    }
    pub fn from_mut_str<'a>(s: &'a mut str) -> &'a mut SRawStr {
        unsafe { std::mem::transmute::<&'a mut str, &'a mut SRawStr>(s) }
    }
    pub fn into_str<'a>(&'a self) -> &'a str {
        unsafe { std::mem::transmute::<&'a SRawStr, &'a str>(self) }
    }
    pub fn into_mut_str<'a>(&'a mut self) -> &'a mut str {
        unsafe { std::mem::transmute::<&'a mut SRawStr, &'a mut str>(self) }
    }
}
impl Deref for SRawStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for SRawStr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl ToOwned for SRawStr {
    type Owned = SString;

    fn to_owned(&self) -> Self::Owned {
        SString::from_string(self.inner.to_owned())
    }
}

impl<'a> SStr<'a> {
    pub fn from_str(s: &'a str) -> Self {
        Self {
            inner: SSlice::from_slice(s.as_bytes()),
        }
    }
    pub fn as_str<'b>(&'b self) -> &'b str
    where
        'a: 'b,
    {
        unsafe { &std::str::from_utf8_unchecked(self.inner.as_slice()) }
    }
    pub fn into_str(self) -> &'a str {
        unsafe { std::str::from_utf8_unchecked(self.inner.as_slice()) }
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
    pub fn as_str<'b>(&'b self) -> &'b str
    where
        'a: 'b,
    {
        unsafe { &std::str::from_utf8_unchecked(self.inner.as_slice()) }
    }
    pub fn as_str_mut<'b>(&'b mut self) -> &'b mut str
    where
        'a: 'b,
    {
        unsafe { std::str::from_utf8_unchecked_mut(self.inner.as_slice_mut()) }
    }
}

impl<'a> Deref for SStr<'a> {
    type Target = SRawStr;

    fn deref(&self) -> &Self::Target {
        SRawStr::from_str(self.as_str())
    }
}

impl<'a> Deref for SMutStr<'a> {
    type Target = SRawStr;

    fn deref(&self) -> &Self::Target {
        SRawStr::from_str(self.as_str())
    }
}

impl<'a> DerefMut for SMutStr<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        SRawStr::from_mut_str(self.as_str_mut())
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
