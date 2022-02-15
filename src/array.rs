use std::fmt::Debug;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[cfg(feature = "convenient_methods")]
use safe_types_derive::impl_methods;

/// FFI-safe equivalent of `[T; N]`
///
/// See documentation of [`array`]
#[repr(C)]
pub struct SArray<T, const N: usize> {
    inner: [T; N],
}

impl<T, const N: usize> SArray<T, N> {
    pub fn from_array(array: [T; N]) -> Self {
        Self { inner: array }
    }
    pub fn into_array(self) -> [T; N] {
        self.inner
    }
    pub fn as_array(&self) -> &[T; N] {
        &self.inner
    }
    pub fn as_array_mut(&mut self) -> &mut [T; N] {
        &mut self.inner
    }
    pub const fn as_slice(&self) -> &[T] {
        &self.inner
    }
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.inner
    }
}

#[cfg(feature = "convenient_methods")]
impl<T, const N: usize> SArray<T, N> {
    impl_methods!(into_array, as_array, as_array_mut, [
        fn map<F, U>(self, f: F) -> [U; N] where F: FnMut(T) -> U;
    ]);
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

impl<T, const N: usize> AsMut<[T]> for SArray<T, N> {
    fn as_mut(&mut self) -> &mut [T] {
        self.inner.as_mut_slice()
    }
}

impl<T, const N: usize> AsRef<[T]> for SArray<T, N> {
    fn as_ref(&self) -> &[T] {
        self.inner.as_slice()
    }
}

impl<T, const N: usize> std::borrow::Borrow<[T]> for SArray<T, N> {
    fn borrow(&self) -> &[T] {
        self.inner.as_slice()
    }
}

impl<T, const N: usize> std::borrow::BorrowMut<[T]> for SArray<T, N> {
    fn borrow_mut(&mut self) -> &mut [T] {
        self.inner.as_mut_slice()
    }
}

impl<T: Debug, const N: usize> Debug for SArray<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T: Default, const N: usize> Default for SArray<T, N> {
    fn default() -> Self {
        SArray::from_array([(); N].map(|_| Default::default()))
    }
}

impl<T: PartialEq<A>, A, const N: usize> PartialEq<SArray<A, N>> for SArray<T, N> {
    fn eq(&self, other: &SArray<A, N>) -> bool {
        self.inner == other.inner
    }
}

impl<T: PartialEq<A>, A, const N: usize> PartialEq<[A; N]> for SArray<T, N> {
    fn eq(&self, other: &[A; N]) -> bool {
        self.inner == *other
    }
}

impl<T, I, const N: usize> Index<I> for SArray<T, N>
where
    [T]: Index<I>,
{
    type Output = <[T] as Index<I>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.inner[index]
    }
}

impl<T, I, const N: usize> IndexMut<I> for SArray<T, N>
where
    [T]: IndexMut<I>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

impl<T, const N: usize> IntoIterator for SArray<T, N> {
    type Item = T;
    type IntoIter = <[T; N] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
