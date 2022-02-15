use std::{
    marker::PhantomData,
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
};

/// Wraps a value and only provides immutable access
///
/// Basically emulates `&T` but carrying `T` inside
pub struct Immutable<'a, T> {
    inner: ManuallyDrop<T>,
    _phantom: PhantomData<&'a T>,
}

/// Wraps a value, provides mutable access and then overrides original on Drop
///
/// Basically emulates `&mut T` but carrying `T` inside
pub struct Mutable<'a, O, T: Into<O>> {
    inner: ManuallyDrop<T>,
    original: &'a mut O,
}

impl<'a, T> Deref for Immutable<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a, T> Immutable<'a, T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: ManuallyDrop::new(inner),
            _phantom: PhantomData,
        }
    }
}

impl<'a, O, T: Into<O>> Deref for Mutable<'a, O, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a, O, T: Into<O>> DerefMut for Mutable<'a, O, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl<'a, O, T: Into<O>> Drop for Mutable<'a, O, T> {
    fn drop(&mut self) {
        *self.original = unsafe { ManuallyDrop::take(&mut self.inner) }.into();
    }
}
impl<'a, O, T: Into<O>> Mutable<'a, O, T> {
    pub fn new(inner: T, original: &'a mut O) -> Self {
        Self {
            inner: ManuallyDrop::new(inner),
            original,
        }
    }
}
impl<'a, O: Into<T>, T: Into<O>> Mutable<'a, O, T> {
    pub fn new_from(original: &'a mut O) -> Self {
        Self {
            inner: ManuallyDrop::new(unsafe { std::ptr::read(original).into() }),
            original,
        }
    }
}
