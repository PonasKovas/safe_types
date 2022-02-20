use std::fmt::{Debug, Display};

use crate::{std::prelude::SVec, Immutable, Mutable};

#[cfg(feature = "convenient_methods")]
use safe_types_derive::impl_methods;

/// FFI-safe equivalent of `String`
///
/// See documentation of [`std::string::String`]
#[repr(C)]
pub struct SString {
    inner: SVec<u8>,
}

impl SString {
    pub fn from_string(s: String) -> Self {
        Self {
            inner: SVec::from_vec(s.into_bytes()),
        }
    }
    pub fn into_string(self) -> String {
        unsafe { String::from_utf8_unchecked(self.inner.into_vec()) }
    }
    pub fn as_string<'a>(&'a self) -> Immutable<'a, String> {
        Immutable::new_from(self)
    }
    pub fn as_string_mut<'a>(&'a mut self) -> Mutable<'a, Self, String> {
        Mutable::new_from(self)
    }

    pub fn from_utf8(vec: SVec<u8>) -> Result<Self, std::string::FromUtf8Error> {
        String::from_utf8(vec.into_vec()).map(|s| Self::from_string(s))
    }
    pub fn as_bytes<'a>(&'a self) -> &'a [u8] {
        self.inner.as_slice()
    }
    // TODO: as_mut_str, as_mut_vec, as_str
}

#[cfg(feature = "convenient_methods")]
impl SString {
    impl_methods!(into_string, as_string, as_string_mut, [
        fn capacity(&self) -> usize;
        fn clear(&mut self);
        fn insert(&mut self, idx: usize, ch: char);
        fn insert_str(&mut self, idx: usize, string: &str);
        fn into_boxed_str(self) -> Box<str>;
        fn into_bytes(self) -> Vec<u8>;
        fn is_empty(&self) -> bool;
        fn len(&self) -> usize;
        fn pop(&mut self) -> Option<char>;
        fn push(&mut self, ch: char);
        fn push_str(&mut self, string: &str);
        fn remove(&mut self, idx: usize) -> char;
        fn replace_range<R>(&mut self, range: R, replace_with: &str) where R: std::ops::RangeBounds<usize>;
        fn reserve(&mut self, additional: usize);
        fn reserve_exact(&mut self, additional: usize);
        fn retain<F>(&mut self, f: F) where F: FnMut(char) -> bool;
        fn shrink_to(&mut self, min_capacity: usize);
        fn shrink_to_fit(&mut self);
        fn split_off(&mut self, at: usize) -> String;
        fn truncate(&mut self, new_len: usize);
        fn try_reserve(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError>;
        fn try_reserve_exact( &mut self, additional: usize ) -> Result<(), std::collections::TryReserveError>;
    ]);
}

impl From<String> for SString {
    fn from(s: String) -> Self {
        Self::from_string(s)
    }
}

impl From<SString> for String {
    fn from(s: SString) -> Self {
        s.into_string()
    }
}

impl Display for SString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&*self.as_string(), f)
    }
}

impl Debug for SString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&*self.as_string(), f)
    }
}

impl PartialEq for SString {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&*self.as_string(), &*other.as_string())
    }
}
impl PartialEq<str> for SString {
    fn eq(&self, other: &str) -> bool {
        PartialEq::eq(&*self.as_string(), other)
    }
}

impl Clone for SString {
    fn clone(&self) -> Self {
        (&*self.as_string()).clone().into()
    }
}

// TODO traits
