use std::io::ErrorKind;

/// A list specifying general categories of I/O error.
///
/// See documentation of [`std::io::ErrorKind`]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
#[repr(C)]
pub enum SErrorKind {
    NotFound,
    PermissionDenied,
    ConnectionRefused,
    ConnectionReset,
    ConnectionAborted,
    NotConnected,
    AddrInUse,
    AddrNotAvailable,
    BrokenPipe,
    AlreadyExists,
    WouldBlock,
    InvalidInput,
    InvalidData,
    TimedOut,
    WriteZero,
    Interrupted,
    Unsupported,
    UnexpectedEof,
    OutOfMemory,
    Other,
}

impl SErrorKind {
    pub fn from_errorkind(e: &ErrorKind) -> Self {
        match e {
            ErrorKind::NotFound => Self::NotFound,
            ErrorKind::PermissionDenied => Self::PermissionDenied,
            ErrorKind::ConnectionRefused => Self::ConnectionRefused,
            ErrorKind::ConnectionReset => Self::ConnectionReset,
            ErrorKind::ConnectionAborted => Self::ConnectionAborted,
            ErrorKind::NotConnected => Self::NotConnected,
            ErrorKind::AddrInUse => Self::AddrInUse,
            ErrorKind::AddrNotAvailable => Self::AddrNotAvailable,
            ErrorKind::BrokenPipe => Self::BrokenPipe,
            ErrorKind::AlreadyExists => Self::AlreadyExists,
            ErrorKind::WouldBlock => Self::WouldBlock,
            ErrorKind::InvalidInput => Self::InvalidInput,
            ErrorKind::InvalidData => Self::InvalidData,
            ErrorKind::TimedOut => Self::TimedOut,
            ErrorKind::WriteZero => Self::WriteZero,
            ErrorKind::Interrupted => Self::Interrupted,
            ErrorKind::Unsupported => Self::Unsupported,
            ErrorKind::UnexpectedEof => Self::UnexpectedEof,
            ErrorKind::OutOfMemory => Self::OutOfMemory,
            _ => Self::Other,
        }
    }
    pub fn as_errorkind(&self) -> ErrorKind {
        match self {
            Self::NotFound => ErrorKind::NotFound,
            Self::PermissionDenied => ErrorKind::PermissionDenied,
            Self::ConnectionRefused => ErrorKind::ConnectionRefused,
            Self::ConnectionReset => ErrorKind::ConnectionReset,
            Self::ConnectionAborted => ErrorKind::ConnectionAborted,
            Self::NotConnected => ErrorKind::NotConnected,
            Self::AddrInUse => ErrorKind::AddrInUse,
            Self::AddrNotAvailable => ErrorKind::AddrNotAvailable,
            Self::BrokenPipe => ErrorKind::BrokenPipe,
            Self::AlreadyExists => ErrorKind::AlreadyExists,
            Self::WouldBlock => ErrorKind::WouldBlock,
            Self::InvalidInput => ErrorKind::InvalidInput,
            Self::InvalidData => ErrorKind::InvalidData,
            Self::TimedOut => ErrorKind::TimedOut,
            Self::WriteZero => ErrorKind::WriteZero,
            Self::Interrupted => ErrorKind::Interrupted,
            Self::Unsupported => ErrorKind::Unsupported,
            Self::UnexpectedEof => ErrorKind::UnexpectedEof,
            Self::OutOfMemory => ErrorKind::OutOfMemory,
            _ => ErrorKind::Other,
        }
    }
}

impl From<ErrorKind> for SErrorKind {
    fn from(e: ErrorKind) -> Self {
        Self::from_errorkind(&e)
    }
}

impl From<SErrorKind> for ErrorKind {
    fn from(e: SErrorKind) -> Self {
        e.as_errorkind()
    }
}
