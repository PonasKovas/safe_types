use std::ops::{Deref, DerefMut};

/// FFI-safe equivalent of `[T; N]`
#[repr(C)]
pub struct SArray<T, const N: usize> {
    inner: [T; N],
}

impl<T, const N: usize> Deref for SArray<T, N> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T, const N: usize> DerefMut for SArray<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: Copy, const N: usize> Copy for SArray<T, N> {}

impl<T: Clone, const N: usize> Clone for SArray<T, N> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
// TODO impl methods and trait
