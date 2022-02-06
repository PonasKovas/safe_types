use super::SIpv4Addr;
use std::{
    fmt::Display,
    net::{SocketAddrV4, ToSocketAddrs},
    str::FromStr,
};

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
    pub fn ip(&self) -> &SIpv4Addr {
        &self.ip
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    pub fn set_ip(&mut self, new_ip: SIpv4Addr) {
        self.ip = new_ip;
    }
    pub fn set_port(&mut self, new_port: u16) {
        self.port = new_port;
    }
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
