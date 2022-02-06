use super::SSocketAddr;
use std::io::Result;
use std::{
    marker::PhantomData,
    net::{TcpStream, ToSocketAddrs},
    ops::Deref,
    time::Duration,
};

/// A TCP stream between a local and a remote socket.
///
/// See documentation of [`std::net::TcpStream`]
///
/// This is the `Owned` variant, that means that it uses a [`Box`](std::boxed::Box)
/// internally and allocates memory. See `Ref` variant: [`STcpStreamRef`]
#[repr(C)]
pub struct STcpStreamOwned {
    ptr: *const (),
    vtable: VTable,
}

/// A TCP stream between a local and a remote socket.
///
/// See documentation of [`std::net::TcpStream`]
///
/// This is the `Ref` variant. See `Owned` variant: [`STcpStreamOwned`]
#[repr(C)]
pub struct STcpStreamRef<'a> {
    ptr: *const (),
    vtable: VTable,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Deref for STcpStreamRef<'a> {
    type Target = STcpStreamOwned;

    fn deref(&self) -> &Self::Target {
        // Cast the &STcpStreamRef to &STcpStreamOwned
        //
        // This is safe because STcpStreamRef and STcpStreamOwned
        // are identical in memory
        let ptr = self as *const STcpStreamRef as *const Self::Target;
        unsafe { ptr.as_ref() }.unwrap()
    }
}

impl Drop for STcpStreamOwned {
    fn drop(&mut self) {
        unsafe {
            (self.vtable.drop)(self.ptr);
        }
    }
}

impl STcpStreamOwned {
    /// Boxes the given TcpStream and constructs a `STcpStreamOwned`
    ///
    /// This operation is not reversible - you can never get the `TcpStream` back
    pub fn from_tcpstream(tcp_stream: TcpStream) -> Self {
        let ptr = Box::into_raw(Box::new(tcp_stream));

        Self {
            ptr: ptr as *const (),
            vtable: VTABLE,
        }
    }
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        TcpStream::connect(addr).map(|stream| Self::from_tcpstream(stream))
    }
    pub fn connect_timeout(addr: &SSocketAddr, timeout: Duration) {
        // Make repr(C) Duration
        todo!();
    }
    pub fn local_addr(&self) -> Result<SSocketAddr> {
        todo!(); // gotta make repr(C) Result and io::Error
                 // (self.vtable.local_addr)(self.ptr)
    }
}

impl<'a> STcpStreamRef<'a> {
    /// Constructs a new `STcpStreamRef` with a reference to the given TcpStream
    pub const fn from_tcpstream(tcp_stream: &'a TcpStream) -> Self {
        Self {
            ptr: tcp_stream as *const _ as *const (),
            vtable: VTABLE,
            _phantom: PhantomData,
        }
    }
}

#[repr(C)]
struct VTable {
    // Only used on Owned variant
    drop: unsafe extern "C" fn(*const ()),
}

unsafe impl<'a> Send for STcpStreamRef<'a> {}
unsafe impl Send for STcpStreamOwned {}
unsafe impl<'a> Sync for STcpStreamRef<'a> {}
unsafe impl Sync for STcpStreamOwned {}

const VTABLE: VTable = {
    unsafe extern "C" fn drop(ptr: *const ()) {
        unsafe {
            std::mem::drop(Box::from_raw(ptr as *mut TcpStream));
        }
    }
    VTable { drop }
};
