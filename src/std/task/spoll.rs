use crate::{Immutable, Mutable};
use std::task::Poll;

#[cfg(feature = "convenient_methods")]
use safe_types_derive::impl_methods;

/// Indicates whether a value is available or if the current task has been scheduled to receive a wakeup instead.
///
/// See documentation of [`std::task::Poll`]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum SPoll<T> {
    Ready(T),
    Pending,
}

impl<T> SPoll<T> {
    pub fn from_poll(p: Poll<T>) -> Self {
        match p {
            Poll::Ready(v) => Self::Ready(v),
            Poll::Pending => Self::Pending,
        }
    }
    pub fn into_poll(self) -> Poll<T> {
        match self {
            Self::Ready(v) => Poll::Ready(v),
            Self::Pending => Poll::Pending,
        }
    }
    pub fn as_poll<'a>(&'a self) -> Immutable<'a, Poll<T>> {
        Immutable::new_from(self)
    }
    pub fn as_poll_mut<'a>(&'a mut self) -> Mutable<'a, Self, Poll<T>> {
        Mutable::new_from(self)
    }
}

#[cfg(feature = "convenient_methods")]
impl<T> SPoll<T> {
    impl_methods!(into_poll, as_poll, as_poll_mut, [
        fn is_pending(&self) -> bool;
        fn is_ready(&self) -> bool;
        fn map<U, F>(self, f: F) -> Poll<U> where F: FnOnce(T) -> U;
    ]);
}

#[cfg(feature = "convenient_methods")]
impl<T, E> SPoll<Result<T, E>> {
    impl_methods!(into_poll, as_poll, as_poll_mut, [
        fn map_err<U, F>(self, f: F) -> Poll<Result<T, U>> where F: FnOnce(E) -> U;
        fn map_ok<U, F>(self, f: F) -> Poll<Result<U, E>> where F: FnOnce(T) -> U;
    ]);
}

#[cfg(feature = "convenient_methods")]
impl<T, E> SPoll<Option<Result<T, E>>> {
    impl_methods!(into_poll, as_poll, as_poll_mut, [
        fn map_err<U, F>(self, f: F) -> Poll<Option<Result<T, U>>> where F: FnOnce(E) -> U;
        fn map_ok<U, F>(self, f: F) -> Poll<Option<Result<U, E>>> where F: FnOnce(T) -> U;
    ]);
}

impl<T> From<SPoll<T>> for Poll<T> {
    fn from(s: SPoll<T>) -> Self {
        s.into_poll()
    }
}

impl<T> From<Poll<T>> for SPoll<T> {
    fn from(p: Poll<T>) -> Self {
        Self::from_poll(p)
    }
}

impl<T> From<T> for SPoll<T> {
    fn from(v: T) -> Self {
        Self::Ready(v)
    }
}
