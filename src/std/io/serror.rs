use super::SErrorKind;
use crate::std::{error::SDynError, prelude::SOption};
use std::{
    fmt::{Debug, Display},
    io::Error,
};

/// The error type for I/O operations of the Read, Write, Seek, and associated traits.
///
/// See documentation of [`std::io::Error`]
#[repr(C)]
pub struct SError {
    repr: SRepr,
}

#[repr(C)]
enum SRepr {
    Os(i32),
    Custom {
        kind: SErrorKind,
        error: SOption<SDynError>,
    },
}

impl SError {
    pub fn from_error(e: Error) -> Self {
        if let Some(os_err) = e.raw_os_error() {
            Self {
                repr: SRepr::Os(os_err),
            }
        } else {
            Self {
                repr: SRepr::Custom {
                    kind: SErrorKind::from_errorkind(&e.kind()),
                    error: SOption::from_option(e.into_inner().map(|e| SDynError::from_error(&*e))),
                },
            }
        }
    }
    pub fn into_error(self) -> Error {
        match self.repr {
            SRepr::Os(os_err) => Error::from_raw_os_error(os_err),
            SRepr::Custom { kind, error } => {
                if let Some(error) = error.into_option() {
                    Error::new(kind.as_errorkind(), error)
                } else {
                    kind.as_errorkind().into()
                }
            }
        }
    }
    // TODO other methods?
}

impl From<Error> for SError {
    fn from(e: Error) -> Self {
        Self::from_error(e)
    }
}

impl From<SError> for Error {
    fn from(e: SError) -> Self {
        e.into_error()
    }
}

impl std::error::Error for SError {}

impl Display for SError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.repr {
            SRepr::Os(code) => {
                write!(f, "os error: {}", code)
            }
            SRepr::Custom { kind, error } => {
                if let &Some(ref error) = &*error.as_option() {
                    write!(f, "{:?}: {}", kind.as_errorkind(), error)
                } else {
                    write!(f, "{:?}", kind.as_errorkind())
                }
            }
        }
    }
}

impl Debug for SError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.repr {
            SRepr::Os(code) => {
                write!(f, "os error: {}", code)
            }
            SRepr::Custom { kind, error } => {
                if let &Some(ref error) = &*error.as_option() {
                    write!(f, "{:?}: {:?}", kind.as_errorkind(), error)
                } else {
                    write!(f, "{:?}", kind.as_errorkind())
                }
            }
        }
    }
}
