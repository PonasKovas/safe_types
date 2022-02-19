use crate::{Immutable, Mutable};
use std::fmt::Debug;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::time::Duration;

#[cfg(feature = "convenient_methods")]
use safe_types_derive::impl_methods;

#[cfg(unix)]
type Handle = std::os::unix::io::RawFd;
#[cfg(windows)]
type Handle = std::os::windows::io::RawSocket;

/// A TCP stream between a local and a remote socket.
///
/// See documentation of [`std::net::TcpStream`]
#[repr(C)]
pub struct STcpStream {
    inner: Handle,
}

impl STcpStream {
    pub fn from_tcpstream(tcp_stream: TcpStream) -> Self {
        #[cfg(unix)]
        let inner = std::os::unix::prelude::IntoRawFd::into_raw_fd(tcp_stream);
        #[cfg(windows)]
        let inner = std::os::windows::io::IntoRawSocket::into_raw_socket(tcp_stream);

        Self { inner }
    }
    pub fn into_tcpstream(self) -> TcpStream {
        unsafe {
            #[cfg(unix)]
            let res = std::os::unix::io::FromRawFd::from_raw_fd(self.inner);
            #[cfg(windows)]
            let res = std::os::windows::io::FromRawSocket::from_raw_socket(self.inner);

            res
        }
    }
    pub fn as_tcpstream<'a>(&'a self) -> Immutable<'a, TcpStream> {
        Immutable::new_from(self)
    }
    pub fn as_tcpstream_mut<'a>(&'a mut self) -> Mutable<'a, Self, TcpStream> {
        Mutable::new_from(self)
    }

    pub fn connect<A: ToSocketAddrs>(addr: A) -> std::io::Result<Self> {
        TcpStream::connect(addr).map(|s| s.into())
    }
    pub fn connect_timeout(addr: &SocketAddr, timeout: Duration) -> std::io::Result<Self> {
        TcpStream::connect_timeout(addr, timeout).map(|s| s.into())
    }
}

#[cfg(feature = "convenient_methods")]
impl STcpStream {
    impl_methods!(into_tcpstream, as_tcpstream, as_tcpstream_mut, [
        fn local_addr(&self) -> std::io::Result<std::net::SocketAddr>;
        fn nodelay(&self) -> std::io::Result<bool>;
        fn peek(&self, buf: &mut [u8]) -> std::io::Result<usize>;
        fn peer_addr(&self) -> std::io::Result<std::net::SocketAddr>;
        fn read_timeout(&self) -> std::io::Result<Option<std::time::Duration>>;
        fn set_nodelay(&self, nodelay: bool) -> std::io::Result<()>;
        fn set_nonblocking(&self, nonblocking: bool) -> std::io::Result<()>;
        fn set_read_timeout(&self, dur: Option<std::time::Duration>) -> std::io::Result<()>;
        fn set_ttl(&self, ttl: u32) -> std::io::Result<()>;
        fn set_write_timeout(&self, dur: Option<std::time::Duration>) -> std::io::Result<()>;
        fn shutdown(&self, how: std::net::Shutdown) -> std::io::Result<()>;
        fn take_error(&self) -> std::io::Result<Option<std::io::Error>>;
        fn try_clone(&self) -> std::io::Result<TcpStream>;
        fn ttl(&self) -> std::io::Result<u32>;
        fn write_timeout(&self) -> std::io::Result<Option<std::time::Duration>>;
    ]);
}

impl Read for STcpStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.as_tcpstream_mut().read(buf)
    }
    fn read_vectored(&mut self, bufs: &mut [std::io::IoSliceMut<'_>]) -> std::io::Result<usize> {
        self.as_tcpstream_mut().read_vectored(bufs)
    }
}

impl Write for STcpStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.as_tcpstream_mut().write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.as_tcpstream_mut().flush()
    }
    fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize> {
        self.as_tcpstream_mut().write_vectored(bufs)
    }
}

impl Debug for STcpStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&*self.as_tcpstream(), f)
    }
}

impl From<TcpStream> for STcpStream {
    fn from(t: TcpStream) -> Self {
        Self::from_tcpstream(t)
    }
}

impl From<STcpStream> for TcpStream {
    fn from(s: STcpStream) -> Self {
        s.into_tcpstream()
    }
}
