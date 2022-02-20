use super::{SIpAddr};
use std::{
    fmt::Display,
    net::{AddrParseError, IpAddr, Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

#[cfg(feature = "convenient_methods")]
use safe_types_derive::impl_methods;

/// An IPv4 address.
///
/// See documentation of [`std::net::Ipv4Addr`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct SIpv4Addr {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
}

impl SIpv4Addr {
    pub const BROADCAST: Self = Self::new(255, 255, 255, 255);
    pub const LOCALHOST: Self = Self::new(127, 0, 0, 1);
    pub const UNSPECIFIED: Self = Self::new(0, 0, 0, 0);

    pub const fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self { a, b, c, d }
    }

    pub const fn from_ipv4addr(x: &Ipv4Addr) -> Self {
        let octets = x.octets();

        Self {
            a: octets[0],
            b: octets[1],
            c: octets[2],
            d: octets[3],
        }
    }
    pub const fn as_ipv4addr(self) -> Ipv4Addr {
        Ipv4Addr::new(self.a, self.b, self.c, self.d)
    }
}

#[cfg(feature = "convenient_methods")]
impl SIpv4Addr {
    impl_methods!(as_ipv4addr, as_ipv4addr, as_ipv4addr, [
        const fn octets(&self) -> [u8; 4];
        const fn is_broadcast(&self) -> bool;
        const fn is_documentation(&self) -> bool;
        const fn is_link_local(&self) -> bool;
        const fn is_loopback(&self) -> bool;
        const fn is_multicast(&self) -> bool;
        const fn is_private(&self) -> bool;
        const fn is_unspecified(&self) -> bool;
        const fn to_ipv6_compatible(&self) -> Ipv6Addr;
        const fn to_ipv6_mapped(&self) -> Ipv6Addr;
    ]);
}

impl From<Ipv4Addr> for SIpv4Addr {
    fn from(x: Ipv4Addr) -> Self {
        Self::from(x.octets())
    }
}

impl From<SIpv4Addr> for Ipv4Addr {
    fn from(x: SIpv4Addr) -> Self {
        Self::new(x.a, x.b, x.c, x.d)
    }
}

impl Display for SIpv4Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}.{}", self.a, self.b, self.c, self.d)
    }
}

impl From<[u8; 4]> for SIpv4Addr {
    fn from(x: [u8; 4]) -> Self {
        Self {
            a: x[0],
            b: x[1],
            c: x[2],
            d: x[3],
        }
    }
}

impl From<u32> for SIpv4Addr {
    fn from(x: u32) -> Self {
        Self::from(x.to_be_bytes())
    }
}

impl FromStr for SIpv4Addr {
    type Err = AddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(Ipv4Addr::from_str(s)?))
    }
}

impl PartialEq<Ipv4Addr> for SIpv4Addr {
    fn eq(&self, other: &Ipv4Addr) -> bool {
        PartialEq::eq(&self.as_ipv4addr(), other)
    }
}

impl PartialOrd<Ipv4Addr> for SIpv4Addr {
    fn partial_cmp(&self, other: &Ipv4Addr) -> Option<std::cmp::Ordering> {
        PartialOrd::partial_cmp(&self.as_ipv4addr(), other)
    }
}

impl PartialEq<IpAddr> for SIpv4Addr {
    fn eq(&self, other: &IpAddr) -> bool {
        PartialEq::eq(&self.as_ipv4addr(), other)
    }
}

impl PartialOrd<IpAddr> for SIpv4Addr {
    fn partial_cmp(&self, other: &IpAddr) -> Option<std::cmp::Ordering> {
        PartialOrd::partial_cmp(&self.as_ipv4addr(), other)
    }
}

impl PartialEq<SIpAddr> for SIpv4Addr {
    fn eq(&self, other: &SIpAddr) -> bool {
        PartialEq::eq(&self.as_ipv4addr(), &other.as_ipaddr())
    }
}

impl PartialOrd<SIpAddr> for SIpv4Addr {
    fn partial_cmp(&self, other: &SIpAddr) -> Option<std::cmp::Ordering> {
        PartialOrd::partial_cmp(&self.as_ipv4addr(), &other.as_ipaddr())
    }
}
