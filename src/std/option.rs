use super::result::SResult;
use crate::{tuples::STuple2, Immutable, Mutable};
use std::ops::{Deref, DerefMut};

#[cfg(feature = "convenient_methods")]
use safe_types_derive::impl_methods;

/// The `Option` type.
///
/// See documentation of [`std::option::Option`]
#[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
#[repr(C)]
pub enum SOption<T> {
    Some(T),
    None,
}

impl<T> SOption<T> {
    pub fn from_option(option: Option<T>) -> Self {
        match option {
            Some(v) => Self::Some(v),
            None => Self::None,
        }
    }
    pub fn into_option(self) -> Option<T> {
        match self {
            Self::Some(v) => Some(v),
            Self::None => None,
        }
    }
    pub fn as_option<'a>(&'a self) -> Immutable<'a, Option<T>> {
        Immutable::new_from(self)
    }
    pub fn as_option_mut<'a>(&'a mut self) -> Mutable<'a, Self, Option<T>> {
        Mutable::new_from(self)
    }
    pub fn as_mut(&mut self) -> Option<&mut T> {
        match self {
            Self::Some(v) => Option::Some(v),
            Self::None => Option::None,
        }
    }
    pub fn as_ref(&self) -> Option<&T> {
        match self {
            Self::Some(v) => Option::Some(v),
            Self::None => Option::None,
        }
    }
    pub fn get_or_insert(&mut self, value: T) -> &mut T {
        self.get_or_insert_with(|| value)
    }
    pub fn get_or_insert_with<F>(&mut self, f: F) -> &mut T
    where
        F: FnOnce() -> T,
    {
        if let Self::None = *self {
            *self = Self::Some(f());
        }

        match self {
            Self::Some(v) => v,
            // SAFETY: a `None` variant for `self` would have been replaced by a `Some`
            // variant in the code above.
            Self::None => unsafe { std::hint::unreachable_unchecked() },
        }
    }
    pub fn insert(&mut self, value: T) -> &mut T {
        *self = Self::Some(value);

        unsafe { self.as_mut().unwrap_unchecked() }
    }
}

#[cfg(feature = "convenient_methods")]
impl<T> SOption<T> {
    impl_methods!(into_option, as_option, as_option_mut, [
        fn and<U>(self, other: Option<U>) -> Option<U>;
        fn and_then<U, F>(self, f: F) -> Option<U> where F: FnOnce(T) -> Option<U>;
        fn expect(self, msg: &str) -> T;
        fn filter<P>(self, predicate: P) -> Option<T> where P: FnOnce(&T) -> bool;
        fn is_none(&self) -> bool;
        fn is_some(&self) -> bool;
        fn map<U, F>(self, op: F) -> Option<U> where F: FnOnce(T) -> U;
        fn map_or<U, F>(self, default: U, op: F) -> U where F: FnOnce(T) -> U;
        fn map_or_else<U, D, F>(self, default: D, op: F) -> U where D: FnOnce() -> U, F: FnOnce(T) -> U;
        fn ok_or<E>(self, err: E) -> Result<T, E>;
        fn ok_or_else<E, F>(self, err: F) -> Result<T, E> where F: FnOnce() -> E;
        fn or(self, other: Option<T>) -> Option<T>;
        fn or_else<F>(self, op: F) -> Option<T> where F: FnOnce() -> Option<T>;
        fn replace(&mut self, value: T) -> Option<T>;
        fn take(&mut self) -> Option<T>;
        fn unwrap(self) -> T;
        fn unwrap_or(self, default: T) -> T;
        fn unwrap_or_else<F>(self, op: F) -> T where F: FnOnce() -> T;
        unsafe fn unwrap_unchecked(self) -> T;
        fn xor(self, other: Option<T>) -> Option<T>;
        fn zip<U>(self, other: Option<U>) -> Option<(T, U)>;
    ]);
}

impl<T, E> SOption<SResult<T, E>> {
    pub fn transpose(self) -> SResult<SOption<T>, E> {
        match self {
            Self::Some(SResult::Ok(x)) => SResult::Ok(SOption::Some(x)),
            Self::Some(SResult::Err(e)) => SResult::Err(e),
            Self::None => SResult::Ok(SOption::None),
        }
    }
}

impl<T: Default> SOption<T> {
    pub fn get_or_insert_default(&mut self) -> &mut T {
        self.get_or_insert_with(|| Default::default())
    }
    pub fn unwrap_or_default(self) -> T {
        self.into_option().unwrap_or_default()
    }
}

impl<T> SOption<SOption<T>> {
    pub fn flatten(self) -> SOption<T> {
        match self {
            Self::Some(v) => v,
            Self::None => SOption::None,
        }
    }
}

impl<'a, T: Clone> SOption<&'a T> {
    pub fn cloned(self) -> SOption<T> {
        SOption::from_option(self.into_option().map(|t| t.clone()))
    }
}
impl<'a, T: Clone> SOption<&'a mut T> {
    pub fn cloned(self) -> SOption<T> {
        SOption::from_option(self.into_option().map(|t| t.clone()))
    }
}

impl<'a, T: Copy> SOption<&'a T> {
    pub fn copied(self) -> SOption<T> {
        match self {
            Self::Some(&v) => SOption::Some(v),
            Self::None => SOption::None,
        }
    }
}
impl<'a, T: Copy> SOption<&'a mut T> {
    pub fn copied(self) -> SOption<T> {
        match self {
            Self::Some(&mut v) => SOption::Some(v),
            Self::None => SOption::None,
        }
    }
}

impl<T: Deref> SOption<T> {
    pub fn as_deref(&self) -> SOption<&<T as Deref>::Target> {
        match self {
            Self::Some(v) => SOption::Some(v.deref()),
            Self::None => SOption::None,
        }
    }
}

impl<T: DerefMut> SOption<T> {
    pub fn as_deref_mut(&mut self) -> SOption<&mut <T as Deref>::Target> {
        match self {
            Self::Some(v) => SOption::Some(v.deref_mut()),
            Self::None => SOption::None,
        }
    }
}

impl<T: Clone> Clone for SOption<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Some(v) => Self::Some(v.clone()),
            Self::None => Self::None,
        }
    }
}

impl<T> Default for SOption<T> {
    fn default() -> Self {
        Self::None
    }
}

impl<T> From<Option<T>> for SOption<T> {
    fn from(r: Option<T>) -> Self {
        Self::from_option(r)
    }
}

impl<T> From<SOption<T>> for Option<T> {
    fn from(r: SOption<T>) -> Self {
        r.into_option()
    }
}

impl<'a, T> From<&'a SOption<T>> for SOption<&'a T> {
    fn from(r: &'a SOption<T>) -> Self {
        match r {
            SOption::Some(r) => Self::Some(r),
            SOption::None => Self::None,
        }
    }
}

impl<'a, T> From<&'a mut SOption<T>> for SOption<&'a mut T> {
    fn from(r: &'a mut SOption<T>) -> Self {
        match r {
            SOption::Some(r) => Self::Some(r),
            SOption::None => Self::None,
        }
    }
}

impl<T> From<T> for SOption<T> {
    fn from(r: T) -> Self {
        Self::Some(r)
    }
}
