use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

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

    pub fn and<U>(self, other: SResult<U, E>) -> SResult<U, E> {
        match self {
            Self::Ok(_) => other,
            Self::Err(e) => SResult::Err(e),
        }
    }
    pub fn and_then<U, F>(self, op: F) -> SResult<U, E>
    where
        F: FnOnce(T) -> SResult<U, E>,
    {
        match self {
            Self::Ok(v) => op(v),
            Self::Err(e) => SResult::Err(e),
        }
    }
    pub fn as_mut(&mut self) -> SResult<&mut T, &mut E> {
        match self {
            Self::Ok(v) => SResult::Ok(v),
            Self::Err(e) => SResult::Err(e),
        }
    }
    pub fn as_ref(&self) -> SResult<&T, &E> {
        match self {
            Self::Ok(v) => SResult::Ok(v),
            Self::Err(e) => SResult::Err(e),
        }
    }
    // TODO: SOption
    // pub fn err(self) -> SOption<E> {
    //     match self {
    //         Self::Ok(_) => SOption::None,
    //         Self::Err(e) => SOption::Some(e),
    //     }
    // }
    // pub fn ok(self) -> SOption<T> {
    //     match self {
    //         Self::Ok(v) => SOption::Some(v),
    //         Self::Err(_) => SOption::None,
    //     }
    // }
    pub fn is_err(&self) -> bool {
        match self {
            Self::Ok(_) => false,
            Self::Err(_) => true,
        }
    }
    pub fn is_ok(&self) -> bool {
        match self {
            Self::Ok(_) => true,
            Self::Err(_) => false,
        }
    }
    pub fn map<U, F>(self, op: F) -> SResult<U, E>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Self::Ok(v) => SResult::Ok(op(v)),
            Self::Err(e) => SResult::Err(e),
        }
    }
    pub fn map_err<U, F>(self, op: F) -> SResult<T, U>
    where
        F: FnOnce(E) -> U,
    {
        match self {
            Self::Ok(v) => SResult::Ok(v),
            Self::Err(e) => SResult::Err(op(e)),
        }
    }
    pub fn map_or<U, F>(self, default: U, op: F) -> U
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Self::Ok(v) => op(v),
            Self::Err(_) => default,
        }
    }
    pub fn map_or_else<U, F, G>(self, err_op: G, ok_op: F) -> U
    where
        F: FnOnce(T) -> U,
        G: FnOnce(E) -> U,
    {
        match self {
            Self::Ok(v) => ok_op(v),
            Self::Err(e) => err_op(e),
        }
    }
    pub fn or<F>(self, other: SResult<T, F>) -> SResult<T, F> {
        match self {
            Self::Ok(v) => SResult::Ok(v),
            Self::Err(_) => other,
        }
    }
    pub fn or_else<F, O>(self, op: O) -> SResult<T, F>
    where
        O: FnOnce(E) -> SResult<T, F>,
    {
        match self {
            Self::Ok(v) => SResult::Ok(v),
            Self::Err(e) => op(e),
        }
    }
    pub unsafe fn unwrap_unchecked(self) -> T {
        unsafe { self.into_result().unwrap_unchecked() }
    }
    pub unsafe fn unwrap_err_unchecked(self) -> E {
        unsafe { self.into_result().unwrap_err_unchecked() }
    }
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Self::Ok(v) => v,
            Self::Err(_) => default,
        }
    }
    pub fn unwrap_or_else<F>(self, op: F) -> T
    where
        F: FnOnce(E) -> T,
    {
        match self {
            Self::Ok(v) => v,
            Self::Err(e) => op(e),
        }
    }
}

impl<T: Default, E> SResult<T, E> {
    pub fn unwrap_or_default(self) -> T {
        match self {
            Self::Ok(v) => v,
            Self::Err(_) => Default::default(),
        }
    }
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

// TODO: SOption
// impl<T, E> SResult<SOption<T>, E> {
//     pub fn transpose(self) -> SOption<SResult<T, E>> {
//         match self {
//             Self::Ok(SOption::Some(x)) => SOption::Some(SResult::Ok(x)),
//             Self::Ok(SOption::None) => SOption::None,
//             Self::Err(e) => SOption::Some(SResult::Err(e)),
//         }
//     }
// }

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
