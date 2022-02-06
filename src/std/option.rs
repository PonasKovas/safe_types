use std::ops::{Deref, DerefMut};

use crate::tuples::STuple2;

use super::result::SResult;

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

    pub fn and<U>(self, other: SOption<U>) -> SOption<U> {
        match self {
            Self::Some(_) => other,
            Self::None => SOption::None,
        }
    }
    pub fn and_then<U, F>(self, f: F) -> SOption<U>
    where
        F: FnOnce(T) -> SOption<U>,
    {
        match self {
            Self::Some(v) => f(v),
            Self::None => SOption::None,
        }
    }
    pub fn as_mut(&mut self) -> SOption<&mut T> {
        match self {
            Self::Some(v) => SOption::Some(v),
            Self::None => SOption::None,
        }
    }
    pub fn as_ref(&self) -> SOption<&T> {
        match self {
            Self::Some(v) => SOption::Some(v),
            Self::None => SOption::None,
        }
    }
    pub fn expect(self, msg: &str) -> T {
        self.into_option().expect(msg)
    }
    pub fn filter<P>(self, predicate: P) -> Self
    where
        P: FnOnce(&T) -> bool,
    {
        match self {
            Self::Some(v) => {
                if predicate(&v) {
                    Self::Some(v)
                } else {
                    Self::None
                }
            }
            Self::None => Self::None,
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
    pub fn is_none(&self) -> bool {
        !self.is_some()
    }
    pub fn is_some(&self) -> bool {
        match self {
            Self::Some(_) => true,
            Self::None => false,
        }
    }
    pub fn map<U, F>(self, op: F) -> SOption<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Self::Some(v) => SOption::Some(op(v)),
            Self::None => SOption::None,
        }
    }
    pub fn map_or<U, F>(self, default: U, op: F) -> U
    where
        F: FnOnce(T) -> U,
    {
        self.into_option().map_or(default, op)
    }
    pub fn map_or_else<U, D, F>(self, default: D, op: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(T) -> U,
    {
        self.into_option().map_or_else(default, op)
    }
    pub fn ok_or<E>(self, err: E) -> SResult<T, E> {
        match self {
            Self::Some(v) => SResult::Ok(v),
            Self::None => SResult::Err(err),
        }
    }
    pub fn ok_or_else<E, F>(self, err: F) -> SResult<T, E>
    where
        F: FnOnce() -> E,
    {
        match self {
            Self::Some(v) => SResult::Ok(v),
            Self::None => SResult::Err(err()),
        }
    }
    pub fn or(self, other: SOption<T>) -> Self {
        match self {
            Self::Some(v) => Self::Some(v),
            Self::None => other,
        }
    }
    pub fn or_else<F>(self, op: F) -> Self
    where
        F: FnOnce() -> Self,
    {
        match self {
            Self::Some(v) => Self::Some(v),
            Self::None => op(),
        }
    }
    pub fn replace(&mut self, value: T) -> Self {
        std::mem::replace(self, Self::Some(value))
    }
    pub fn take(&mut self) -> Self {
        std::mem::replace(self, Self::None)
    }
    pub fn unwrap(self) -> T {
        self.into_option().unwrap()
    }
    pub fn unwrap_or(self, default: T) -> T {
        self.into_option().unwrap_or(default)
    }
    pub fn unwrap_or_else<F>(self, op: F) -> T
    where
        F: FnOnce() -> T,
    {
        self.into_option().unwrap_or_else(op)
    }
    pub unsafe fn unwrap_unchecked(self) -> T {
        unsafe { self.into_option().unwrap_unchecked() }
    }
    pub fn xor(self, other: Self) -> Self {
        match (self, other) {
            (Self::Some(x), Self::None) => Self::Some(x),
            (Self::None, Self::Some(x)) => Self::Some(x),
            _ => Self::None,
        }
    }
    pub fn zip<U>(self, other: SOption<U>) -> SOption<STuple2<T, U>> {
        match (self, other) {
            (SOption::Some(a), SOption::Some(b)) => SOption::Some(STuple2(a, b)),
            _ => SOption::None,
        }
    }
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
        self.map(|t| t.clone())
    }
}
impl<'a, T: Clone> SOption<&'a mut T> {
    pub fn cloned(self) -> SOption<T> {
        self.map(|t| t.clone())
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
