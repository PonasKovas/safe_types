use std::net::Shutdown;

/// Possible values which can be passed to the `TcpStream::shutdown` method.
///
/// See documentation of [`std::net::Shutdown`]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum SShutdown {
    Read,
    Write,
    Both,
}

impl SShutdown {
    pub fn from_shutdown(s: Shutdown) -> Self {
        match s {
            Shutdown::Read => Self::Read,
            Shutdown::Write => Self::Write,
            Shutdown::Both => Self::Both,
        }
    }
    pub fn as_shutdown(self) -> Shutdown {
        match self {
            Self::Read => Shutdown::Read,
            Self::Write => Shutdown::Write,
            Self::Both => Shutdown::Both,
        }
    }
}

impl From<Shutdown> for SShutdown {
    fn from(s: Shutdown) -> Self {
        Self::from_shutdown(s)
    }
}
impl From<SShutdown> for Shutdown {
    fn from(s: SShutdown) -> Self {
        s.as_shutdown()
    }
}
