use super::option::SOption;
use crate::{Immutable, Mutable};
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

#[cfg(feature = "convenient_methods")]
use safe_types_derive::impl_methods;

/// `SResult` is a type that represents either success ([`Ok`](SResult::Ok)) or failure ([`Err`](SResult::Err)).
///
/// See documentation of [`std::result::Result`]
#[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
#[repr(C)]
pub enum SResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> SResult<T, E> {
    pub fn from_result(r: Result<T, E>) -> Self {
        match r {
            Ok(v) => Self::Ok(v),
            Err(v) => Self::Err(v),
        }
    }
    pub fn into_result(self) -> Result<T, E> {
        match self {
            Self::Ok(v) => Ok(v),
            Self::Err(v) => Err(v),
        }
    }
    pub fn as_result<'a>(&'a self) -> Immutable<'a, Result<T, E>> {
        Immutable::new(unsafe { std::ptr::read(self) }.into_result())
    }
    pub fn as_result_mut<'a>(&'a mut self) -> Mutable<'a, Self, Result<T, E>> {
        Mutable::new_from(self)
    }
    pub fn as_mut(&mut self) -> Result<&mut T, &mut E> {
        match self {
            Self::Ok(v) => Result::Ok(v),
            Self::Err(e) => Result::Err(e),
        }
    }
    pub fn as_ref(&self) -> Result<&T, &E> {
        match self {
            Self::Ok(v) => Result::Ok(v),
            Self::Err(e) => Result::Err(e),
        }
    }
}

#[cfg(feature = "convenient_methods")]
impl<T, E> SResult<T, E> {
    impl_methods!(into_result, as_result, as_result_mut, [
        fn and<U>(self, other: Result<U, E>) -> Result<U, E>;
        fn and_then<U, F>(self, op: F) -> Result<U, E> where F: FnOnce(T) -> Result<U, E>;
        fn err(self) -> Option<E>;
        fn ok(self) -> Option<T>;
        fn is_err(&self) -> bool;
        fn is_ok(&self) -> bool;
        fn map<U, F>(self, op: F) -> Result<U, E> where F: FnOnce(T) -> U;
        fn map_err<U, F>(self, op: F) -> Result<T, U> where F: FnOnce(E) -> U;
        fn map_or<U, F>(self, default: U, op: F) -> U where F: FnOnce(T) -> U;
        fn map_or_else<U, F, G>(self, err_op: G, ok_op: F) -> U where F: FnOnce(T) -> U, G: FnOnce(E) -> U;
        fn or<F>(self, other: Result<T, F>) -> Result<T, F>;
        fn or_else<F, O>(self, op: O) -> Result<T, F> where O: FnOnce(E) -> Result<T, F>;
        unsafe fn unwrap_unchecked(self) -> T;
        unsafe fn unwrap_err_unchecked(self) -> E;
        fn unwrap_or(self, default: T) -> T;
        fn unwrap_or_else<F>(self, op: F) -> T where F: FnOnce(E) -> T;
    ]);
}

#[cfg(feature = "convenient_methods")]
impl<T: Default, E> SResult<T, E> {
    impl_methods!(into_result, as_result, as_result_mut, [
        fn unwrap_or_default(self) -> T;
    ]);
}

impl<T, E: Debug> SResult<T, E> {
    pub fn expect(self, msg: &str) -> T {
        self.into_result().expect(msg)
    }
    pub fn unwrap(self) -> T {
        self.into_result().unwrap()
    }
}
impl<T: Debug, E> SResult<T, E> {
    pub fn expect_err(self, msg: &str) -> E {
        self.into_result().expect_err(msg)
    }
    pub fn unwrap_err(self) -> E {
        self.into_result().unwrap_err()
    }
}

impl<T, E> SResult<SOption<T>, E> {
    pub fn transpose(self) -> SOption<SResult<T, E>> {
        match self {
            Self::Ok(SOption::Some(x)) => SOption::Some(SResult::Ok(x)),
            Self::Ok(SOption::None) => SOption::None,
            Self::Err(e) => SOption::Some(SResult::Err(e)),
        }
    }
}

impl<T, E> SResult<T, E>
where
    T: Deref,
{
    pub fn as_deref(&self) -> SResult<&<T as Deref>::Target, &E> {
        match self {
            Self::Ok(v) => SResult::Ok(v.deref()),
            Self::Err(e) => SResult::Err(e),
        }
    }
}

impl<T, E> SResult<T, E>
where
    T: DerefMut,
{
    pub fn as_deref_mut(&mut self) -> SResult<&<T as Deref>::Target, &E> {
        match self {
            Self::Ok(v) => SResult::Ok(v.deref_mut()),
            Self::Err(e) => SResult::Err(e),
        }
    }
}

impl<T: Clone, E: Clone> Clone for SResult<T, E> {
    fn clone(&self) -> Self {
        match self {
            Self::Ok(x) => Self::Ok(x.clone()),
            Self::Err(x) => Self::Err(x.clone()),
        }
    }
}

impl<T, E> From<Result<T, E>> for SResult<T, E> {
    fn from(r: Result<T, E>) -> Self {
        Self::from_result(r)
    }
}

impl<T, E> From<SResult<T, E>> for Result<T, E> {
    fn from(r: SResult<T, E>) -> Self {
        r.into_result()
    }
}
