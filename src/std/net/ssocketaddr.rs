use super::{SIpAddr, SSocketAddrV4, SSocketAddrV6};
use crate::Mutable;
use std::{
    net::{IpAddr, SocketAddr, SocketAddrV4, SocketAddrV6, ToSocketAddrs},
    str::FromStr,
};

#[cfg(feature = "convenient_methods")]
use safe_types_derive::impl_methods;

/// An internet socket address, either IPv4 or IPv6.
///
/// See documentation of [`std::net::SocketAddr`]
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum SSocketAddr {
    V4(SSocketAddrV4),
    V6(SSocketAddrV6),
}

impl SSocketAddr {
    pub fn from_socketaddr(x: &SocketAddr) -> Self {
        match x {
            SocketAddr::V4(v) => Self::V4(SSocketAddrV4::from_socketaddrv4(v)),
            SocketAddr::V6(v) => Self::V6(SSocketAddrV6::from_socketaddrv6(v)),
        }
    }
    pub fn as_socketaddr(&self) -> SocketAddr {
        match self {
            Self::V4(v) => SocketAddr::V4(v.as_socketaddrv4()),
            Self::V6(v) => SocketAddr::V6(v.as_socketaddrv6()),
        }
    }
    pub fn as_socketaddr_mut<'a>(&'a mut self) -> Mutable<'a, SSocketAddr, SocketAddr> {
        Mutable::new_from(self)
    }
    pub fn new(ip: SIpAddr, port: u16) -> Self {
        Self::from_socketaddr(&SocketAddr::new(ip.as_ipaddr(), port))
    }
}
#[cfg(feature = "convenient_methods")]
impl SSocketAddr {
    impl_methods!(as_socketaddr, as_socketaddr, as_socketaddr_mut, [
        fn ip(&self) -> IpAddr;
        fn is_ipv4(&self) -> bool;
        fn is_ipv6(&self) -> bool;
        fn port(&self) -> u16;
        fn set_ip(&mut self, new_ip: IpAddr);
        fn set_port(&mut self, new_port: u16);
    ]);
}

impl From<SocketAddr> for SSocketAddr {
    fn from(x: SocketAddr) -> Self {
        Self::from_socketaddr(&x)
    }
}
impl From<SSocketAddr> for SocketAddr {
    fn from(x: SSocketAddr) -> Self {
        x.as_socketaddr()
    }
}

impl<I: Into<IpAddr>> From<(I, u16)> for SSocketAddr {
    fn from(x: (I, u16)) -> Self {
        Self::from_socketaddr(&SocketAddr::from(x))
    }
}
impl From<SocketAddrV4> for SSocketAddr {
    fn from(x: SocketAddrV4) -> Self {
        Self::from_socketaddr(&SocketAddr::from(x))
    }
}
impl From<SocketAddrV6> for SSocketAddr {
    fn from(x: SocketAddrV6) -> Self {
        Self::from_socketaddr(&SocketAddr::from(x))
    }
}
impl From<SSocketAddrV4> for SSocketAddr {
    fn from(x: SSocketAddrV4) -> Self {
        Self::from_socketaddr(&SocketAddr::from(x.as_socketaddrv4()))
    }
}
impl From<SSocketAddrV6> for SSocketAddr {
    fn from(x: SSocketAddrV6) -> Self {
        Self::from_socketaddr(&SocketAddr::from(x.as_socketaddrv6()))
    }
}

impl FromStr for SSocketAddr {
    type Err = <SocketAddr as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(SocketAddr::from_str(s)?))
    }
}

impl ToSocketAddrs for SSocketAddr {
    type Iter = <SocketAddr as ToSocketAddrs>::Iter;

    fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
        ToSocketAddrs::to_socket_addrs(&self.as_socketaddr())
    }
}
