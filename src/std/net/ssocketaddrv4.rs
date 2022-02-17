use super::SIpv4Addr;
use crate::{Immutable, Mutable};
use std::{
    fmt::Display,
    net::{Ipv4Addr, SocketAddrV4, ToSocketAddrs},
    str::FromStr,
};

#[cfg(feature = "convenient_methods")]
use safe_types_derive::impl_methods;

/// An IPv4 socket address.
///
/// See documentation of [`std::net::SocketAddrV4`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct SSocketAddrV4 {
    ip: SIpv4Addr,
    port: u16,
}

impl SSocketAddrV4 {
    pub fn from_socketaddrv4(x: &SocketAddrV4) -> Self {
        Self {
            ip: SIpv4Addr::from_ipv4addr(x.ip()),
            port: x.port(),
        }
    }
    pub fn as_socketaddrv4(&self) -> SocketAddrV4 {
        SocketAddrV4::new(self.ip.as_ipv4addr(), self.port)
    }
    pub fn as_socketaddrv4_mut<'a>(&'a mut self) -> Mutable<'a, Self, SocketAddrV4> {
        Mutable::new_from(self)
    }
    pub fn ip(&self) -> Ipv4Addr {
        *self.as_socketaddrv4().ip()
    }
}
#[cfg(feature = "convenient_methods")]
impl SSocketAddrV4 {
    impl_methods!(as_socketaddrv4, as_socketaddrv4, as_socketaddrv4_mut, [
        fn port(&self) -> u16;
        fn set_ip(&mut self, new_ip: Ipv4Addr);
        fn set_port(&mut self, new_port: u16);
    ]);
}

impl Display for SSocketAddrV4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.as_socketaddrv4(), f)
    }
}

impl From<SocketAddrV4> for SSocketAddrV4 {
    fn from(x: SocketAddrV4) -> Self {
        Self::from_socketaddrv4(&x)
    }
}

impl From<SSocketAddrV4> for SocketAddrV4 {
    fn from(x: SSocketAddrV4) -> Self {
        x.as_socketaddrv4()
    }
}

impl FromStr for SSocketAddrV4 {
    type Err = <SocketAddrV4 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(SocketAddrV4::from_str(s)?))
    }
}

impl ToSocketAddrs for SSocketAddrV4 {
    type Iter = <SocketAddrV4 as ToSocketAddrs>::Iter;

    fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
        ToSocketAddrs::to_socket_addrs(&self.as_socketaddrv4())
    }
}
