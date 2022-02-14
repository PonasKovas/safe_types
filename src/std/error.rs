use super::prelude::SString;
use crate::{std::prelude::SOption, SArray};
use std::{
    error::Error,
    fmt::{Debug, Display},
    intrinsics::transmute,
    marker::PhantomData,
};

/// FFI-safe equivalent of `Box<dyn Error + Send + Sync>`
#[repr(C)]
pub struct DynError {
    // this implementation barely resembles the real dyn Error
    // and is lossy (information is lost when you convert a dyn Error to this type)
    // but I couldn't put myself through the horrors of making an accurate type.
    // Dynamic dispatch + FFI = not a good time
    // Feel free to take on this if you want to
    debug: SString,
    display: SString,
}
