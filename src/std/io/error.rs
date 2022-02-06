use crate::{std::prelude::SOption, SArray};
use std::{
    error::Error,
    fmt::{Debug, Display},
    intrinsics::transmute,
    marker::PhantomData,
};

type FatPtr = SArray<*const (), 2>;

/// FFI-safe equivalent of `Box<dyn Error + Send + Sync>`
#[repr(C)]
pub struct OwnedDynError {
    inner: RefDynError<'static>,
}

/// FFI-safe equivalent of `&'a dyn Error + Send + Sync`
#[repr(C)]
pub struct RefDynError<'a> {
    ptr: FatPtr, // &(dyn Error) - a fat pointer
    source: unsafe extern "C" fn(FatPtr) -> SOption<RefDynError<'a>>,
    // backtrace is not stabilized yet, so not supported
    // description and cause are deprecated, so not supported either
    _phantom: PhantomData<&'a ()>,
}

unsafe extern "C" fn source<'a>(ptr: FatPtr) -> SOption<RefDynError<'a>> {
    let src = unsafe { (*transmute::<FatPtr, *const dyn Error>(ptr)).source() };
    SOption::from_option(src).map(|error| RefDynError::from_dyn_error(error))
}

impl<'a> RefDynError<'a> {
    pub fn from_dyn_error(error: &(dyn Error)) -> Self {
        Self {
            ptr: unsafe { transmute::<*const dyn Error, FatPtr>(error as *const _) },
            source,
            _phantom: PhantomData,
        }
    }
    pub fn into_dyn_error(&self) -> &(dyn Error) {
        self
    }
}

impl OwnedDynError {
    pub fn from_dyn_error(error: Box<dyn Error>) -> Self {
        Self {
            inner: RefDynError {
                ptr: unsafe { transmute::<*const dyn Error, FatPtr>(Box::into_raw(error)) },
                source,
                _phantom: PhantomData,
            },
        }
    }
}

impl Drop for OwnedDynError {
    fn drop(&mut self) {
        unsafe { Box::from_raw(transmute::<FatPtr, *mut dyn Error>(self.inner.ptr)) };
    }
}

impl<'a> Debug for RefDynError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl<'a> Display for RefDynError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<'a> Error for RefDynError<'a> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        unsafe {
            // wtf 'static
            (self.source)(self.ptr)
                .into_option()
                .map(|e| e.into_dyn_error())
        }
    }
}

impl Debug for OwnedDynError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl Display for OwnedDynError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for OwnedDynError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        unsafe {
            (self.inner.source)(self.inner.ptr)
                .into_option()
                .map(|e| e.into_dyn_error())
        }
    }
}

unsafe impl Send for OwnedDynError {}
unsafe impl Sync for OwnedDynError {}
unsafe impl<'a> Send for RefDynError<'a> {}
unsafe impl<'a> Sync for RefDynError<'a> {}
