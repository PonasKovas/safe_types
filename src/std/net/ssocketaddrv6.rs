use crate::{Immutable, Mutable};

use super::SIpv6Addr;
use std::{
    fmt::Display,
    net::{Ipv6Addr, SocketAddrV6, ToSocketAddrs},
    str::FromStr,
};

#[cfg(feature = "convenient_methods")]
use safe_types_derive::impl_methods;

/// An IPv6 socket address.
///
/// See documentation of [`std::net::SocketAddrV6`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct SSocketAddrV6 {
    ip: SIpv6Addr,
    flowinfo: u32,
    scope_id: u32,
    port: u16,
}

impl SSocketAddrV6 {
    pub fn from_socketaddrv6(x: &SocketAddrV6) -> Self {
        Self {
            ip: SIpv6Addr::from_ipv6addr(x.ip()),
            flowinfo: x.flowinfo(),
            scope_id: x.scope_id(),
            port: x.port(),
        }
    }
    pub fn as_socketaddrv6(&self) -> SocketAddrV6 {
        SocketAddrV6::new(
            self.ip.as_ipv6addr(),
            self.port,
            self.flowinfo,
            self.scope_id,
        )
    }
    pub fn as_socketaddrv6_mut<'a>(&'a mut self) -> Mutable<'a, Self, SocketAddrV6> {
        Mutable::new_from(self)
    }
    pub fn ip(&self) -> Ipv6Addr {
        *self.as_socketaddrv6().ip()
    }
    pub fn new(ip: SIpv6Addr, port: u16, flowinfo: u32, scope_id: u32) -> Self {
        Self {
            ip,
            port,
            flowinfo,
            scope_id,
        }
    }
}

#[cfg(feature = "convenient_methods")]
impl SSocketAddrV6 {
    impl_methods!(as_socketaddrv6, as_socketaddrv6, as_socketaddrv6_mut, [
        fn flowinfo(&self) -> u32;
        fn port(&self) -> u16;
        fn scope_id(&self) -> u32;
        fn set_flowinfo(&mut self, new_flowinfo: u32);
        fn set_ip(&mut self, new_ip: Ipv6Addr);
        fn set_port(&mut self, new_port: u16);
        fn set_scope_id(&mut self, new_scope_id: u32);
    ]);
}

impl Display for SSocketAddrV6 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.as_socketaddrv6(), f)
    }
}

impl From<SocketAddrV6> for SSocketAddrV6 {
    fn from(x: SocketAddrV6) -> Self {
        Self::from_socketaddrv6(&x)
    }
}

impl From<SSocketAddrV6> for SocketAddrV6 {
    fn from(x: SSocketAddrV6) -> Self {
        x.as_socketaddrv6()
    }
}

impl FromStr for SSocketAddrV6 {
    type Err = <SocketAddrV6 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(SocketAddrV6::from_str(s)?))
    }
}

impl ToSocketAddrs for SSocketAddrV6 {
    type Iter = <SocketAddrV6 as ToSocketAddrs>::Iter;

    fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
        ToSocketAddrs::to_socket_addrs(&self.as_socketaddrv6())
    }
}
