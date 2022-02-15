use super::{SIpv4Addr, SIpv6Addr};
use std::{
    fmt::Display,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

#[cfg(feature = "convenient_methods")]
use safe_types_derive::impl_methods;

/// An IP address, either IPv4 or IPv6.
///
/// See documentation of [`std::net::IpAddr`]
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum SIpAddr {
    V4(SIpv4Addr),
    V6(SIpv6Addr),
}

impl SIpAddr {
    pub const fn from_ipaddr(x: &IpAddr) -> Self {
        match x {
            IpAddr::V4(v) => Self::V4(SIpv4Addr::from_ipv4addr(v)),
            IpAddr::V6(v) => Self::V6(SIpv6Addr::from_ipv6addr(v)),
        }
    }
    pub const fn as_ipaddr(&self) -> IpAddr {
        match self {
            Self::V4(v) => IpAddr::V4(v.as_ipv4addr()),
            Self::V6(v) => IpAddr::V6(v.as_ipv6addr()),
        }
    }
}
#[cfg(feature = "convenient_methods")]
impl SIpAddr {
    impl_methods!(as_ipaddr, as_ipaddr, as_ipaddr, [
        const fn is_ipv4(&self) -> bool;
        const fn is_ipv6(&self) -> bool;
        const fn is_loopback(&self) -> bool;
        const fn is_multicast(&self) -> bool;
        const fn is_unspecified(&self) -> bool;
    ]);
}

impl Display for SIpAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.as_ipaddr(), f)
    }
}

impl From<IpAddr> for SIpAddr {
    fn from(x: IpAddr) -> Self {
        Self::from_ipaddr(&x)
    }
}
impl From<SIpAddr> for IpAddr {
    fn from(x: SIpAddr) -> Self {
        x.as_ipaddr()
    }
}

impl From<[u16; 8]> for SIpAddr {
    fn from(x: [u16; 8]) -> Self {
        Self::from(IpAddr::from(x))
    }
}

impl From<[u8; 16]> for SIpAddr {
    fn from(x: [u8; 16]) -> Self {
        Self::from(IpAddr::from(x))
    }
}

impl From<[u8; 4]> for SIpAddr {
    fn from(x: [u8; 4]) -> Self {
        Self::from(IpAddr::from(x))
    }
}

impl From<Ipv4Addr> for SIpAddr {
    fn from(x: Ipv4Addr) -> Self {
        Self::from(IpAddr::from(x))
    }
}

impl From<Ipv6Addr> for SIpAddr {
    fn from(x: Ipv6Addr) -> Self {
        Self::from(IpAddr::from(x))
    }
}

impl From<SIpv4Addr> for SIpAddr {
    fn from(x: SIpv4Addr) -> Self {
        Self::V4(x)
    }
}

impl From<SIpv6Addr> for SIpAddr {
    fn from(x: SIpv6Addr) -> Self {
        Self::V6(x)
    }
}

impl FromStr for SIpAddr {
    type Err = <IpAddr as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(IpAddr::from_str(s)?))
    }
}

impl PartialEq<IpAddr> for SIpAddr {
    fn eq(&self, other: &IpAddr) -> bool {
        PartialEq::eq(&self.as_ipaddr(), other)
    }
}
impl PartialEq<Ipv4Addr> for SIpAddr {
    fn eq(&self, other: &Ipv4Addr) -> bool {
        PartialEq::eq(&self.as_ipaddr(), other)
    }
}
impl PartialEq<Ipv6Addr> for SIpAddr {
    fn eq(&self, other: &Ipv6Addr) -> bool {
        PartialEq::eq(&self.as_ipaddr(), other)
    }
}
impl PartialEq<SIpv4Addr> for SIpAddr {
    fn eq(&self, other: &SIpv4Addr) -> bool {
        PartialEq::eq(&self.as_ipaddr(), &other.as_ipv4addr())
    }
}
impl PartialEq<SIpv6Addr> for SIpAddr {
    fn eq(&self, other: &SIpv6Addr) -> bool {
        PartialEq::eq(&self.as_ipaddr(), &other.as_ipv6addr())
    }
}

impl PartialOrd<IpAddr> for SIpAddr {
    fn partial_cmp(&self, other: &IpAddr) -> Option<std::cmp::Ordering> {
        PartialOrd::partial_cmp(&self.as_ipaddr(), other)
    }
}
impl PartialOrd<Ipv4Addr> for SIpAddr {
    fn partial_cmp(&self, other: &Ipv4Addr) -> Option<std::cmp::Ordering> {
        PartialOrd::partial_cmp(&self.as_ipaddr(), other)
    }
}
impl PartialOrd<Ipv6Addr> for SIpAddr {
    fn partial_cmp(&self, other: &Ipv6Addr) -> Option<std::cmp::Ordering> {
        PartialOrd::partial_cmp(&self.as_ipaddr(), other)
    }
}
impl PartialOrd<SIpv4Addr> for SIpAddr {
    fn partial_cmp(&self, other: &SIpv4Addr) -> Option<std::cmp::Ordering> {
        PartialOrd::partial_cmp(&self.as_ipaddr(), &other.as_ipv4addr())
    }
}
impl PartialOrd<SIpv6Addr> for SIpAddr {
    fn partial_cmp(&self, other: &SIpv6Addr) -> Option<std::cmp::Ordering> {
        PartialOrd::partial_cmp(&self.as_ipaddr(), &other.as_ipv6addr())
    }
}
