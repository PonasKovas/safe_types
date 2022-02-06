use super::{OwnedDynError, SErrorKind};
use crate::std::prelude::SOption;
use std::{fmt::Debug, io::Error};

/// The error type for I/O operations of the [`Read`], [`Write`], [`Seek`], and associated traits.
///
/// See documentation of [`std::io::Error`]
#[repr(C)]
pub struct SError {
    pub(crate) kind: SErrorKind,
    pub(crate) error: SOption<OwnedDynError>,
}

impl SError {
    pub fn from_error(error: Error) -> Self {
        Self {
            kind: SErrorKind::from_errorkind(&error.kind()),
            error: SOption::from_option(error.into_inner())
                .map(|e| OwnedDynError::from_dyn_error(e)),
        }
    }
    pub fn into_error(self) -> Error {
        if let SOption::Some(e) = self.error {
            Error::new(self.kind.as_errorkind(), e)
        } else {
            Error::from(self.kind.as_errorkind())
        }
    }
}

impl Debug for SError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
