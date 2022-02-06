use crate::{std::io, std::prelude::SResult, SArray, SSlice};
use std::{intrinsics::transmute, io::Write, marker::PhantomData};

type FatPtr = SArray<*mut (), 2>;

/// `#[repr(C)]` equivalent of `&'a dyn Write`
#[repr(C)]
pub struct RefDynWrite<'a> {
    ptr: FatPtr,
    write: unsafe extern "C" fn(FatPtr, SSlice<u8>) -> SResult<usize, io::SError>,
    flush: unsafe extern "C" fn(FatPtr) -> SResult<(), io::SError>,
    // There are more methods on this trait, but they're not required
    // so i'm gonna go the easy way for now and not include them
    _phantom: PhantomData<&'a ()>,
}

/// `#[repr(C)]` equivalent of `Box<dyn Write>`
#[repr(C)]
pub struct OwnedDynWrite {
    inner: RefDynWrite<'static>,
}

unsafe extern "C" fn write(ptr: FatPtr, buf: SSlice<u8>) -> SResult<usize, io::SError> {
    let dyn_object = unsafe { transmute::<FatPtr, *mut dyn Write>(ptr) };
    let res = unsafe { (*dyn_object).write(buf.as_slice()) };
    SResult::from_result(res).map_err(|error| io::SError::from_error(error))
}
unsafe extern "C" fn flush(ptr: FatPtr) -> SResult<(), io::SError> {
    let dyn_object = unsafe { transmute::<FatPtr, *mut dyn Write>(ptr) };
    let res = unsafe { (*dyn_object).flush() };
    SResult::from_result(res).map_err(|error| io::SError::from_error(error))
}

impl<'a> RefDynWrite<'a> {
    pub fn from_dyn_write(dyn_write: &(dyn Write)) -> Self {
        Self {
            ptr: unsafe { transmute::<*const dyn Write, FatPtr>(dyn_write as *const _) },
            write,
            flush,
            _phantom: PhantomData,
        }
    }
}

impl OwnedDynWrite {
    pub fn from_dyn_write(dyn_write: Box<dyn Write>) -> Self {
        Self {
            inner: RefDynWrite {
                ptr: unsafe { transmute::<*const dyn Write, FatPtr>(Box::into_raw(dyn_write)) },
                write,
                flush,
                _phantom: PhantomData,
            },
        }
    }
}

impl Drop for OwnedDynWrite {
    fn drop(&mut self) {
        unsafe { Box::from_raw(transmute::<FatPtr, *mut dyn Write>(self.inner.ptr)) };
    }
}

impl<'a> Write for RefDynWrite<'a> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        (self.write)(self.ptr, SSlice::from_slice(buf))
            .into_result()
            .map_err(|e| e.into_error())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}
