use super::SShutdown;
use crate::std::io::SResult as SioResult;
use crate::std::prelude::SOption;
use crate::std::time::SDuration;
use crate::SUnit;
use crate::{
    std::{io::SError as SioError, net::SSocketAddr},
    SMutSlice,
};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::ops::{Deref, DerefMut};
use std::time::Duration;

/// An immutable reference to a TCP stream between a local and a remote socket.
///
/// See documentation of [`std::net::TcpStream`]
#[repr(C)]
pub struct STcpStreamRef<'a> {
    inner: ManuallyDrop<STcpStream>,
    _phantom: PhantomData<&'a ()>,
}

/// A mutable reference to a TCP stream between a local and a remote socket.
///
/// See documentation of [`std::net::TcpStream`]
#[repr(C)]
pub struct STcpStreamMut<'a> {
    inner: ManuallyDrop<STcpStream>,
    _phantom: PhantomData<&'a ()>,
}

/// A TCP stream between a local and a remote socket.
///
/// See documentation of [`std::net::TcpStream`]
#[repr(C)]
pub struct STcpStream {
    ptr: *const (),
    vtable: VTable,
}

#[repr(C)]
struct VTable {
    local_addr: unsafe extern "C" fn(*const ()) -> SioResult<SSocketAddr>,
    nodelay: unsafe extern "C" fn(*const ()) -> SioResult<bool>,
    peek: unsafe extern "C" fn(*const (), SMutSlice<u8>) -> SioResult<usize>,
    peer_addr: unsafe extern "C" fn(*const ()) -> SioResult<SSocketAddr>,
    read_timeout: unsafe extern "C" fn(*const ()) -> SioResult<SOption<SDuration>>,
    set_nodelay: unsafe extern "C" fn(*const (), bool) -> SioResult<SUnit>,
    set_nonblocking: unsafe extern "C" fn(*const (), bool) -> SioResult<SUnit>,
    set_read_timeout: unsafe extern "C" fn(*const (), SOption<SDuration>) -> SioResult<SUnit>,
    set_ttl: unsafe extern "C" fn(*const (), u32) -> SioResult<SUnit>,
    set_write_timeout: unsafe extern "C" fn(*const (), SOption<SDuration>) -> SioResult<SUnit>,
    shutdown: unsafe extern "C" fn(*const (), SShutdown) -> SioResult<SUnit>,
    take_error: unsafe extern "C" fn(*const ()) -> SioResult<SOption<SioError>>,
    try_clone: unsafe extern "C" fn(*const ()) -> SioResult<STcpStream>,
    ttl: unsafe extern "C" fn(*const ()) -> SioResult<u32>,
    write_timeout: unsafe extern "C" fn(*const ()) -> SioResult<SOption<SDuration>>,
}

const VTABLE: VTable = VTable {
    local_addr,
    nodelay,
    peek,
    peer_addr,
    read_timeout,
    set_nodelay,
    set_nonblocking,
    set_read_timeout,
    set_ttl,
    set_write_timeout,
    shutdown,
    take_error,
    try_clone,
    ttl,
    write_timeout,
};

impl STcpStream {
    pub fn from_tcpstream(tcp_stream: Box<TcpStream>) -> Self {
        let ptr = Box::into_raw(tcp_stream) as *const ();

        Self {
            ptr,
            vtable: VTABLE,
        }
    }
    pub fn local_addr(&self) -> std::io::Result<SocketAddr> {
        unsafe {
            (self.vtable.local_addr)(self.ptr)
                .into_result()
                .map(|s| s.as_socketaddr())
                .map_err(|e| e.into_error())
        }
    }
    pub fn nodelay(&self) -> std::io::Result<bool> {
        unsafe {
            (self.vtable.nodelay)(self.ptr)
                .into_result()
                .map_err(|e| e.into_error())
        }
    }
    pub fn peek(&self, buf: &mut [u8]) -> std::io::Result<usize> {
        unsafe {
            (self.vtable.peek)(self.ptr, SMutSlice::from_slice(buf))
                .into_result()
                .map_err(|e| e.into_error())
        }
    }
    pub fn peer_addr(&self) -> std::io::Result<SocketAddr> {
        unsafe {
            (self.vtable.peer_addr)(self.ptr)
                .into_result()
                .map(|s| s.as_socketaddr())
                .map_err(|e| e.into_error())
        }
    }
    pub fn read_timeout(&self) -> std::io::Result<Option<Duration>> {
        unsafe {
            (self.vtable.read_timeout)(self.ptr)
                .into_result()
                .map(|o| o.into_option().map(|d| d.into()))
                .map_err(|e| e.into_error())
        }
    }
    pub fn set_nodelay(&self, nodelay: bool) -> std::io::Result<()> {
        unsafe {
            (self.vtable.set_nodelay)(self.ptr, nodelay)
                .into_result()
                .map(|_| ())
                .map_err(|e| e.into_error())
        }
    }
    pub fn set_nonblocking(&self, nonblocking: bool) -> std::io::Result<()> {
        unsafe {
            (self.vtable.set_nonblocking)(self.ptr, nonblocking)
                .into_result()
                .map(|_| ())
                .map_err(|e| e.into_error())
        }
    }
    pub fn set_read_timeout(&self, dur: Option<Duration>) -> std::io::Result<()> {
        unsafe {
            (self.vtable.set_read_timeout)(self.ptr, dur.map(|d| d.into()).into())
                .into_result()
                .map(|_| ())
                .map_err(|e| e.into_error())
        }
    }
    pub fn set_ttl(&self, ttl: u32) -> std::io::Result<()> {
        unsafe {
            (self.vtable.set_ttl)(self.ptr, ttl)
                .into_result()
                .map(|_| ())
                .map_err(|e| e.into_error())
        }
    }
    pub fn set_write_timeout(&self, dur: Option<Duration>) -> std::io::Result<()> {
        unsafe {
            (self.vtable.set_write_timeout)(self.ptr, dur.map(|d| d.into()).into())
                .into_result()
                .map(|_| ())
                .map_err(|e| e.into_error())
        }
    }
    pub fn shutdown(&self, how: Shutdown) -> std::io::Result<()> {
        unsafe {
            (self.vtable.shutdown)(self.ptr, how.into())
                .into_result()
                .map(|_| ())
                .map_err(|e| e.into_error())
        }
    }
    pub fn take_error(&self) -> std::io::Result<Option<std::io::Error>> {
        unsafe {
            (self.vtable.take_error)(self.ptr)
                .into_result()
                .map(|o| o.into_option().map(|e| e.into()))
                .map_err(|e| e.into_error())
        }
    }
    /// This allocates: the new `STcpStream` is put in a `Box`
    pub fn try_clone(&self) -> std::io::Result<STcpStream> {
        unsafe {
            (self.vtable.try_clone)(self.ptr)
                .into_result()
                .map_err(|e| e.into_error())
        }
    }
    pub fn ttl(&self) -> std::io::Result<u32> {
        unsafe {
            (self.vtable.ttl)(self.ptr)
                .into_result()
                .map_err(|e| e.into_error())
        }
    }
    pub fn write_timeout(&self) -> std::io::Result<Option<Duration>> {
        unsafe {
            (self.vtable.write_timeout)(self.ptr)
                .into_result()
                .map(|o| o.into_option().map(|d| d.into()))
                .map_err(|e| e.into_error())
        }
    }
}

impl<'a> STcpStreamRef<'a> {
    pub fn from_tcpstream(tcp_stream: &'a TcpStream) -> Self {
        let ptr = tcp_stream as *const _ as *const ();

        Self {
            inner: ManuallyDrop::new(STcpStream {
                ptr,
                vtable: VTABLE,
            }),
            _phantom: PhantomData,
        }
    }
}

impl<'a> STcpStreamMut<'a> {
    pub fn from_tcpstream(tcp_stream: &'a mut TcpStream) -> Self {
        let ptr = tcp_stream as *const _ as *const ();

        Self {
            inner: ManuallyDrop::new(STcpStream {
                ptr,
                vtable: VTABLE,
            }),
            _phantom: PhantomData,
        }
    }
}

impl<'a> Deref for STcpStreamRef<'a> {
    type Target = STcpStream;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl<'a> Deref for STcpStreamMut<'a> {
    type Target = STcpStream;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}
impl<'a> DerefMut for STcpStreamMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.inner
    }
}

impl<'a> Debug for STcpStreamRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "STcpStreamRef")
    }
}
impl<'a> Debug for STcpStreamMut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "STcpStreamMut")
    }
}
impl Debug for STcpStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "STcpStream")
    }
}

impl From<TcpStream> for STcpStream {
    fn from(t: TcpStream) -> Self {
        Self::from_tcpstream(Box::new(t))
    }
}
impl<'a> From<&'a TcpStream> for STcpStreamRef<'a> {
    fn from(t: &'a TcpStream) -> Self {
        Self::from_tcpstream(t)
    }
}
impl<'a> From<&'a mut TcpStream> for STcpStreamMut<'a> {
    fn from(t: &'a mut TcpStream) -> Self {
        Self::from_tcpstream(t)
    }
}

unsafe impl Send for STcpStream {}
unsafe impl Sync for STcpStream {}

// Converts &() to &TcpStream
macro_rules! cast_ref {
    ($ptr:expr) => {
        unsafe {
            ($ptr as *const () as *const TcpStream)
                .as_ref()
                .unwrap_unchecked()
        }
    };
}

unsafe extern "C" fn local_addr(ptr: *const ()) -> SioResult<SSocketAddr> {
    let tcp_stream: &TcpStream = cast_ref!(ptr);

    tcp_stream
        .local_addr()
        .map(|s| s.into())
        .map_err(|e| e.into())
        .into()
}

unsafe extern "C" fn nodelay(ptr: *const ()) -> SioResult<bool> {
    let tcp_stream: &TcpStream = cast_ref!(ptr);

    tcp_stream.nodelay().map_err(|e| e.into()).into()
}

unsafe extern "C" fn peek(ptr: *const (), buf: SMutSlice<u8>) -> SioResult<usize> {
    let tcp_stream: &TcpStream = cast_ref!(ptr);
    let buf = buf.into_slice();

    tcp_stream.peek(buf).map_err(|e| e.into()).into()
}

unsafe extern "C" fn peer_addr(ptr: *const ()) -> SioResult<SSocketAddr> {
    let tcp_stream: &TcpStream = cast_ref!(ptr);

    tcp_stream
        .peer_addr()
        .map(|s| s.into())
        .map_err(|e| e.into())
        .into()
}

unsafe extern "C" fn read_timeout(ptr: *const ()) -> SioResult<SOption<SDuration>> {
    let tcp_stream: &TcpStream = cast_ref!(ptr);

    tcp_stream
        .read_timeout()
        .map_err(|e| e.into())
        .map(|o| o.map(|d| d.into()).into())
        .into()
}

unsafe extern "C" fn set_nodelay(ptr: *const (), nodelay: bool) -> SioResult<SUnit> {
    let tcp_stream: &TcpStream = cast_ref!(ptr);

    tcp_stream
        .set_nodelay(nodelay)
        .map(|_| SUnit::new())
        .map_err(|e| e.into())
        .into()
}

unsafe extern "C" fn set_nonblocking(ptr: *const (), nonblocking: bool) -> SioResult<SUnit> {
    let tcp_stream: &TcpStream = cast_ref!(ptr);

    tcp_stream
        .set_nonblocking(nonblocking)
        .map(|_| SUnit::new())
        .map_err(|e| e.into())
        .into()
}

unsafe extern "C" fn set_read_timeout(ptr: *const (), dur: SOption<SDuration>) -> SioResult<SUnit> {
    let tcp_stream: &TcpStream = cast_ref!(ptr);
    let dur = dur.into_option().map(|d| d.into());

    tcp_stream
        .set_read_timeout(dur)
        .map(|_| SUnit::new())
        .map_err(|e| e.into())
        .into()
}

unsafe extern "C" fn set_ttl(ptr: *const (), ttl: u32) -> SioResult<SUnit> {
    let tcp_stream: &TcpStream = cast_ref!(ptr);

    tcp_stream
        .set_ttl(ttl)
        .map(|_| SUnit::new())
        .map_err(|e| e.into())
        .into()
}

unsafe extern "C" fn set_write_timeout(
    ptr: *const (),
    dur: SOption<SDuration>,
) -> SioResult<SUnit> {
    let tcp_stream: &TcpStream = cast_ref!(ptr);
    let dur = dur.into_option().map(|d| d.into());

    tcp_stream
        .set_write_timeout(dur)
        .map(|_| SUnit::new())
        .map_err(|e| e.into())
        .into()
}

unsafe extern "C" fn shutdown(ptr: *const (), how: SShutdown) -> SioResult<SUnit> {
    let tcp_stream: &TcpStream = cast_ref!(ptr);

    tcp_stream
        .shutdown(how.into())
        .map(|_| SUnit::new())
        .map_err(|e| e.into())
        .into()
}

unsafe extern "C" fn take_error(ptr: *const ()) -> SioResult<SOption<SioError>> {
    let tcp_stream: &TcpStream = cast_ref!(ptr);

    tcp_stream
        .take_error()
        .map(|o| o.map(|e| e.into()).into())
        .map_err(|e| e.into())
        .into()
}

unsafe extern "C" fn try_clone(ptr: *const ()) -> SioResult<STcpStream> {
    let tcp_stream: &TcpStream = cast_ref!(ptr);

    tcp_stream
        .try_clone()
        .map_err(|e| e.into())
        .map(|stream| STcpStream::from_tcpstream(Box::new(stream)))
        .into()
}

unsafe extern "C" fn ttl(ptr: *const ()) -> SioResult<u32> {
    let tcp_stream: &TcpStream = cast_ref!(ptr);

    tcp_stream.ttl().map_err(|e| e.into()).into()
}

unsafe extern "C" fn write_timeout(ptr: *const ()) -> SioResult<SOption<SDuration>> {
    let tcp_stream: &TcpStream = cast_ref!(ptr);

    tcp_stream
        .write_timeout()
        .map_err(|e| e.into())
        .map(|o| o.map(|d| d.into()).into())
        .into()
}
