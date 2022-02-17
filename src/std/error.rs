use super::prelude::SString;
use std::{
    error::Error,
    fmt::{Debug, Display},
};

/// FFI-safe equivalent of `Box<dyn Error + Send + Sync>`
#[repr(C)]
pub struct SDynError {
    // this implementation barely resembles the real dyn Error
    // and is lossy (information is lost when you convert a dyn Error to this type)
    // but I couldn't put myself through the horrors of making an accurate type.
    // Dynamic dispatch + FFI = not a good time
    // Feel free to take on this if you want to
    debug: SString,
    display: SString,
}

impl SDynError {
    pub fn from_error(e: &dyn Error) -> Self {
        Self {
            debug: format!("{:?}", e).into(),
            display: format!("{}", e).into(),
        }
    }
    pub fn into_dyn_error(self) -> Box<dyn Error + Send + Sync> {
        Box::new(self)
    }
}

impl Display for SDynError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display)
    }
}

impl Debug for SDynError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.debug)
    }
}

impl Error for SDynError {}
