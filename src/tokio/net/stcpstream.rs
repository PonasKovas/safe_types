use crate::std::io::SResult as SioResult;
use crate::std::net::SShutdown;
use crate::std::prelude::SOption;
use crate::std::task::{SContext, SPoll};
use crate::std::time::SDuration;
use crate::{
    std::{io::SError as SioError, net::SSocketAddr},
    SMutSlice,
};
use crate::{SSlice, SUnit};
use std::fmt::Debug;
use std::io::{Read, Write};
use std::marker::PhantomData;
use std::net::{Shutdown, SocketAddr};
use std::ops::{Deref, DerefMut};
use std::task::Context;
use std::task::Poll;
use std::time::Duration;
use tokio::net::TcpStream;

/// An immutable reference to a TCP stream between a local and a remote socket.
///
/// See documentation of [`tokio::net::TcpStream`]
#[repr(C)]
pub struct STcpStreamRef<'a> {
    ptr: *const (),
    _phantom: PhantomData<&'a ()>,
    vtable: &'static ImmutableVTable,
}

/// A mutable reference to a TCP stream between a local and a remote socket.
///
/// See documentation of [`tokio::net::TcpStream`]
#[repr(C)]
pub struct STcpStreamMut<'a> {
    inner: STcpStreamRef<'a>,
    vtable: &'static MutableVTable,
}

/// A TCP stream between a local and a remote socket.
///
/// See documentation of [`tokio::net::TcpStream`]
#[repr(C)]
pub struct STcpStream {
    inner: STcpStreamMut<'static>,
    vtable: &'static OwnedVTable,
}

// *const () is &TcpStream
#[repr(C)]
struct ImmutableVTable {
    linger: unsafe extern "C" fn(*const ()) -> SioResult<SOption<SDuration>>,
    local_addr: unsafe extern "C" fn(*const ()) -> SioResult<SSocketAddr>,
    nodelay: unsafe extern "C" fn(*const ()) -> SioResult<bool>,
    peer_addr: unsafe extern "C" fn(*const ()) -> SioResult<SSocketAddr>,
    // poll_peek: unsafe extern "C" fn(*const (), ...) -> ...,
    poll_read_ready: unsafe extern "C" fn(*const (), &mut SContext) -> SPoll<SioResult<SUnit>>,
    poll_write_ready: unsafe extern "C" fn(*const (), &mut SContext) -> SPoll<SioResult<SUnit>>,
    set_linger: unsafe extern "C" fn(*const (), SOption<SDuration>) -> SioResult<SUnit>,
    set_nodelay: unsafe extern "C" fn(*const (), bool) -> SioResult<SUnit>,
    set_ttl: unsafe extern "C" fn(*const (), u32) -> SioResult<SUnit>,
    // split: ...,
    try_read: unsafe extern "C" fn(*const (), SMutSlice<u8>) -> SioResult<usize>,
    try_read_vectored:
        unsafe extern "C" fn(*const (), SMutSlice<std::io::IoSliceMut>) -> SioResult<usize>,
    try_write: unsafe extern "C" fn(*const (), SSlice<u8>) -> SioResult<usize>,
    try_write_vectored:
        unsafe extern "C" fn(*const (), SSlice<std::io::IoSlice>) -> SioResult<usize>,
    ttl: unsafe extern "C" fn(*const ()) -> SioResult<u32>,
}

// *mut () is &mut TcpStream
#[repr(C)]
struct MutableVTable {}

// *mut () is Box<TcpStream>
#[repr(C)]
struct OwnedVTable {
    into_std: unsafe extern "C" fn(*mut ()) -> SioResult<crate::std::net::STcpStream>,
}

static IMMUTABLE_VTABLE: ImmutableVTable = ImmutableVTable {
    linger,
    local_addr,
    nodelay,
    peer_addr,
    // poll_peek,
    poll_read_ready,
    poll_write_ready,
    set_linger,
    set_nodelay,
    set_ttl,
    // split,
    try_read,
    try_read_vectored,
    try_write,
    try_write_vectored,
    ttl,
};
static MUTABLE_VTABLE: MutableVTable = MutableVTable {};
static OWNED_VTABLE: OwnedVTable = OwnedVTable { into_std };

impl<'a> STcpStreamRef<'a> {
    pub fn from_tcpstream(tcp_stream: &'a TcpStream) -> Self {
        let ptr = tcp_stream as *const _ as *const ();

        Self {
            ptr,
            vtable: &IMMUTABLE_VTABLE,
            _phantom: PhantomData,
        }
    }
    pub fn linger(&self) -> std::io::Result<Option<Duration>> {
        unsafe {
            (self.vtable.linger)(self.ptr)
                .into_result()
                .map(|s| s.into_option().map(|d| d.into()))
                .map_err(|e| e.into_error())
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
    pub fn peer_addr(&self) -> std::io::Result<SocketAddr> {
        unsafe {
            (self.vtable.peer_addr)(self.ptr)
                .into_result()
                .map(|s| s.as_socketaddr())
                .map_err(|e| e.into_error())
        }
    }
    // poll_peek
    pub fn poll_read_ready(&self, ctx: Context) -> Poll<std::io::Result<()>> {
        unsafe {
            SContext::from_context(&ctx, |mut ctx| {
                (self.vtable.poll_read_ready)(self.ptr, &mut ctx)
                    .into_poll()
                    .map(|r| r.into_result().map(|_| ()).map_err(|e| e.into_error()))
            })
        }
    }
    pub fn poll_write_ready(&self, ctx: Context) -> Poll<std::io::Result<()>> {
        unsafe {
            SContext::from_context(&ctx, |mut ctx| {
                (self.vtable.poll_write_ready)(self.ptr, &mut ctx)
                    .into_poll()
                    .map(|r| r.into_result().map(|_| ()).map_err(|e| e.into_error()))
            })
        }
    }
    pub fn set_linger(&self, linger: Option<Duration>) -> std::io::Result<()> {
        unsafe {
            (self.vtable.set_linger)(self.ptr, linger.map(|d| d.into()).into())
                .into_result()
                .map(|_| ())
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
    pub fn set_ttl(&self, ttl: u32) -> std::io::Result<()> {
        unsafe {
            (self.vtable.set_ttl)(self.ptr, ttl)
                .into_result()
                .map(|_| ())
                .map_err(|e| e.into_error())
        }
    }
    // split
    pub fn try_read(&self, buf: &mut [u8]) -> std::io::Result<usize> {
        unsafe {
            (self.vtable.try_read)(self.ptr, buf.into())
                .into_result()
                .map_err(|e| e.into_error())
        }
    }
    pub fn try_read_vectored(&self, bufs: &mut [std::io::IoSliceMut]) -> std::io::Result<usize> {
        unsafe {
            (self.vtable.try_read_vectored)(self.ptr, bufs.into())
                .into_result()
                .map_err(|e| e.into_error())
        }
    }
    pub fn try_write(&self, buf: &[u8]) -> std::io::Result<usize> {
        unsafe {
            (self.vtable.try_write)(self.ptr, buf.into())
                .into_result()
                .map_err(|e| e.into_error())
        }
    }
    pub fn try_write_vectored(&self, bufs: &[std::io::IoSlice]) -> std::io::Result<usize> {
        unsafe {
            (self.vtable.try_write_vectored)(self.ptr, bufs.into())
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
}

impl<'a> STcpStreamMut<'a> {
    pub fn from_tcpstream(tcp_stream: &'a mut TcpStream) -> Self {
        Self {
            inner: STcpStreamRef {
                ptr: tcp_stream as *const _ as *const (),
                vtable: &IMMUTABLE_VTABLE,
                _phantom: PhantomData,
            },
            vtable: &MUTABLE_VTABLE,
        }
    }
}

impl STcpStream {
    pub fn from_tcpstream(tcp_stream: Box<TcpStream>) -> Self {
        let tcp_stream: &mut TcpStream = Box::leak(tcp_stream);

        Self {
            inner: STcpStreamMut::from_tcpstream(tcp_stream),
            vtable: &OWNED_VTABLE,
        }
    }

    pub fn into_std(self) -> std::io::Result<std::net::TcpStream> {
        unsafe {
            (self.vtable.into_std)(self.inner.inner.ptr as *mut ())
                .into_result()
                .map(|s| s.into())
                .map_err(|e| e.into_error())
        }
    }
}

impl<'a> Deref for STcpStreamMut<'a> {
    type Target = STcpStreamRef<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Deref for STcpStream {
    type Target = STcpStreamMut<'static>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for STcpStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
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

// Converts *const () to &TcpStream
macro_rules! cast_ref {
    ($ptr:expr) => {
        unsafe { ($ptr as *const TcpStream).as_ref().unwrap_unchecked() }
    };
}

// Converts *mut () to &mut TcpStream
macro_rules! cast_mut {
    ($ptr:expr) => {
        unsafe { ($ptr as *mut TcpStream).as_mut().unwrap_unchecked() }
    };
}

////////////////////////////////////
//                                //
// EXTERN "C" FNS IMPLEMENTATIONS //
//                                //
////////////////////////////////////

unsafe extern "C" fn linger(ptr: *const ()) -> SioResult<SOption<SDuration>> {
    let tcp_stream: &TcpStream = cast_ref!(ptr);

    tcp_stream
        .linger()
        .map(|o| o.map(|d| d.into()).into())
        .map_err(|e| e.into())
        .into()
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

unsafe extern "C" fn peer_addr(ptr: *const ()) -> SioResult<SSocketAddr> {
    let tcp_stream: &TcpStream = cast_ref!(ptr);

    tcp_stream
        .peer_addr()
        .map(|s| s.into())
        .map_err(|e| e.into())
        .into()
}

// poll_peek

unsafe extern "C" fn poll_read_ready(
    ptr: *const (),
    ctx: &mut SContext,
) -> SPoll<SioResult<SUnit>> {
    let tcp_stream: &TcpStream = cast_ref!(ptr);

    ctx.with_context(|ctx| tcp_stream.poll_read_ready(ctx))
        .map(|r| r.map_err(|e| e.into()).map(|_| SUnit::new()).into())
        .into()
}

unsafe extern "C" fn poll_write_ready(
    ptr: *const (),
    ctx: &mut SContext,
) -> SPoll<SioResult<SUnit>> {
    let tcp_stream: &TcpStream = cast_ref!(ptr);

    ctx.with_context(|ctx| tcp_stream.poll_write_ready(ctx))
        .map(|r| r.map_err(|e| e.into()).map(|_| SUnit::new()).into())
        .into()
}

unsafe extern "C" fn set_linger(ptr: *const (), linger: SOption<SDuration>) -> SioResult<SUnit> {
    let tcp_stream: &TcpStream = cast_ref!(ptr);

    tcp_stream
        .set_linger(linger.into_option().map(|d| d.into()))
        .map_err(|e| e.into())
        .map(|_| SUnit::new())
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

unsafe extern "C" fn set_ttl(ptr: *const (), ttl: u32) -> SioResult<SUnit> {
    let tcp_stream: &TcpStream = cast_ref!(ptr);

    tcp_stream
        .set_ttl(ttl)
        .map(|_| SUnit::new())
        .map_err(|e| e.into())
        .into()
}

unsafe extern "C" fn try_read(ptr: *const (), buf: SMutSlice<u8>) -> SioResult<usize> {
    let tcp_stream: &TcpStream = cast_ref!(ptr);

    tcp_stream.try_read(buf.into()).map_err(|e| e.into()).into()
}

unsafe extern "C" fn try_read_vectored(
    ptr: *const (),
    bufs: SMutSlice<std::io::IoSliceMut>,
) -> SioResult<usize> {
    let tcp_stream: &TcpStream = cast_ref!(ptr);

    tcp_stream
        .try_read_vectored(bufs.into())
        .map_err(|e| e.into())
        .into()
}

unsafe extern "C" fn try_write(ptr: *const (), buf: SSlice<u8>) -> SioResult<usize> {
    let tcp_stream: &TcpStream = cast_ref!(ptr);

    tcp_stream
        .try_write(buf.into())
        .map_err(|e| e.into())
        .into()
}

unsafe extern "C" fn try_write_vectored(
    ptr: *const (),
    bufs: SSlice<std::io::IoSlice>,
) -> SioResult<usize> {
    let tcp_stream: &TcpStream = cast_ref!(ptr);

    tcp_stream
        .try_write_vectored(bufs.into())
        .map_err(|e| e.into())
        .into()
}

unsafe extern "C" fn ttl(ptr: *const ()) -> SioResult<u32> {
    let tcp_stream: &TcpStream = cast_ref!(ptr);

    tcp_stream.ttl().map_err(|e| e.into()).into()
}

unsafe extern "C" fn into_std(ptr: *mut ()) -> SioResult<crate::std::net::STcpStream> {
    let tcp_stream = unsafe { Box::from_raw(ptr as *mut TcpStream) };

    tcp_stream
        .into_std()
        .map(|s| s.into())
        .map_err(|e| e.into())
        .into()
}
