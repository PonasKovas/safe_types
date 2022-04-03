use crate::{Immutable, Mutable};
use std::{
    borrow::{Borrow, Cow},
    ops::Deref,
};

/// A clone-on-write smart pointer.
///
/// See documentation of [`std::borrow::Cow`]
///
/// The difference is that `Self::Borrowed` holds `B` instead of `&B`, since
/// most types that are used here are unsized and only have `&B` ffi-safe variants
/// (for example `SStr<'a>` is `&'a str` instead of `str`).
///
/// Generic `B` must be a reference (or act like one):
/// - It must `Deref` to something that implements `ToOwned`
/// - `&*B == B` must hold true (if you want to convert from/into the std `Cow`)
///
/// So, for example, the most common case would be `SCow<&'a str>` (instead of `Cow<'a, str>`).
/// Or to be more precise, `SCow<SStr<'a>>`, which is FFI-safe.
#[repr(C)]
pub enum SCow<B>
where
    B: Deref,
    <B as Deref>::Target: ToOwned,
{
    Borrowed(B),
    Owned(<<B as Deref>::Target as ToOwned>::Owned),
}

impl<'a, B> SCow<B>
where
    B: 'a + Deref,
    <B as Deref>::Target: ToOwned,
    B: Into<&'a <B as Deref>::Target>,
    &'a <B as Deref>::Target: Into<B>,
{
    pub fn from_cow(c: Cow<'a, <B as Deref>::Target>) -> Self {
        match c {
            Cow::Borrowed(r) => Self::Borrowed(r.into()),
            Cow::Owned(o) => Self::Owned(o),
        }
    }
    pub fn into_cow(self) -> Cow<'a, <B as Deref>::Target> {
        match self {
            Self::Borrowed(r) => Cow::Borrowed(r.into()),
            Self::Owned(o) => Cow::Owned(o),
        }
    }
    pub fn as_cow<'b>(&'b self) -> Immutable<'b, Cow<'a, <B as Deref>::Target>> {
        Immutable::new(unsafe { std::ptr::read(self).into_cow() })
    }
    pub fn as_cow_mut<'b>(&'b mut self) -> Mutable<'b, Self, Cow<'a, <B as Deref>::Target>> {
        Mutable::new(unsafe { std::ptr::read(self).into_cow() }, self)
    }
}

impl<'a, B: 'a + Deref> From<Cow<'a, <B as Deref>::Target>> for SCow<B>
where
    <B as Deref>::Target: ToOwned,
    B: Into<&'a <B as Deref>::Target>,
    &'a <B as Deref>::Target: Into<B>,
{
    fn from(b: Cow<'a, <B as Deref>::Target>) -> Self {
        Self::from_cow(b)
    }
}

impl<'a, B: 'a + Deref> From<SCow<B>> for Cow<'a, <B as Deref>::Target>
where
    <B as Deref>::Target: ToOwned,
    B: Into<&'a <B as Deref>::Target>,
    &'a <B as Deref>::Target: Into<B>,
{
    fn from(b: SCow<B>) -> Cow<'a, <B as Deref>::Target> {
        b.into_cow()
    }
}

impl<'a, B: 'a + Deref> Clone for SCow<B>
where
    <B as Deref>::Target: ToOwned,
    B: Into<&'a <B as Deref>::Target>,
    &'a <B as Deref>::Target: Into<B>,
{
    fn clone(&self) -> Self {
        self.as_cow().clone().into()
    }
}

impl<'a, B: 'a + Deref> Deref for SCow<B>
where
    <B as Deref>::Target: ToOwned,
    B: Into<&'a <B as Deref>::Target>,
    &'a <B as Deref>::Target: Into<B>,
{
    type Target = <B as Deref>::Target;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Borrowed(r) => r,
            Self::Owned(o) => o.borrow(),
        }
    }
}
