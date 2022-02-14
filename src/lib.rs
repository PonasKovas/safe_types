#![deny(unsafe_op_in_unsafe_fn)]

mod array;
mod sslice;
mod sstr;
pub mod std;
mod tuples;

pub use array::SArray;
pub use sslice::{SMutSlice, SSlice};
pub use sstr::SStr;
pub use tuples::*;

use ::std::{
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
impl<'a, T> Deref for Immutable<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// Wraps a value and only provides mutable access
///
/// Basically emulates `&mut T` but carrying `T` inside
pub struct Mutable<'a, T> {
    inner: ManuallyDrop<T>,
    _phantom: PhantomData<&'a T>,
}
impl<'a, T> Deref for Mutable<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a, T> DerefMut for Mutable<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
