use super::SIpAddr;
use std::{
    fmt::Display,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

#[cfg(feature = "convenient_methods")]
use safe_types_derive::impl_methods;

/// An IPv6 address.
///
/// See documentation of [`std::net::Ipv6Addr`]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct SIpv6Addr {
    a: u16,
    b: u16,
    c: u16,
    d: u16,
    e: u16,
    f: u16,
    g: u16,
    h: u16,
}

impl SIpv6Addr {
    pub const LOCALHOST: Self = Self::new(0, 0, 0, 0, 0, 0, 0, 1);
    pub const UNSPECIFIED: Self = Self::new(0, 0, 0, 0, 0, 0, 0, 0);

    pub const fn new(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> Self {
        Self {
            a,
            b,
            c,
            d,
            e,
            f,
            g,
            h,
        }
    }
    pub const fn from_ipv6addr(x: &Ipv6Addr) -> Self {
        let segments = x.segments();

        Self {
            a: segments[0],
            b: segments[1],
            c: segments[2],
            d: segments[3],
            e: segments[4],
            f: segments[5],
            g: segments[6],
            h: segments[7],
        }
    }
    pub const fn as_ipv6addr(self) -> Ipv6Addr {
        Ipv6Addr::new(
            self.a, self.b, self.c, self.d, self.e, self.f, self.g, self.h,
        )
    }
}

#[cfg(feature = "convenient_methods")]
impl SIpv6Addr {
    impl_methods!(as_ipv6addr, as_ipv6addr, as_ipv6addr, [
        const fn segments(&self) -> [u16; 8];
        const fn octets(&self) -> [u8; 16];
        const fn is_loopback(&self) -> bool;
        const fn is_multicast(&self) -> bool;
        const fn is_unspecified(&self) -> bool;
        const fn to_ipv4(&self) -> Option<Ipv4Addr>;
    ]);
}

impl Display for SIpv6Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.as_ipv6addr(), f)
    }
}

impl From<[u16; 8]> for SIpv6Addr {
    fn from(x: [u16; 8]) -> Self {
        Self::from_ipv6addr(&Ipv6Addr::from(x))
    }
}

impl From<[u8; 16]> for SIpv6Addr {
    fn from(x: [u8; 16]) -> Self {
        Self::from_ipv6addr(&Ipv6Addr::from(x))
    }
}

impl From<u128> for SIpv6Addr {
    fn from(x: u128) -> Self {
        Self::from_ipv6addr(&Ipv6Addr::from(x))
    }
}

impl From<Ipv6Addr> for SIpv6Addr {
    fn from(x: Ipv6Addr) -> Self {
        Self::from_ipv6addr(&x)
    }
}
impl From<SIpv6Addr> for Ipv6Addr {
    fn from(x: SIpv6Addr) -> Self {
        x.as_ipv6addr()
    }
}

impl FromStr for SIpv6Addr {
    type Err = <Ipv6Addr as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_ipv6addr(&Ipv6Addr::from_str(s)?))
    }
}

impl PartialEq<Ipv6Addr> for SIpv6Addr {
    fn eq(&self, other: &Ipv6Addr) -> bool {
        PartialEq::eq(&self.as_ipv6addr(), other)
    }
}

impl PartialOrd<Ipv6Addr> for SIpv6Addr {
    fn partial_cmp(&self, other: &Ipv6Addr) -> Option<std::cmp::Ordering> {
        PartialOrd::partial_cmp(&self.as_ipv6addr(), other)
    }
}

impl PartialEq<IpAddr> for SIpv6Addr {
    fn eq(&self, other: &IpAddr) -> bool {
        PartialEq::eq(&self.as_ipv6addr(), other)
    }
}

impl PartialOrd<IpAddr> for SIpv6Addr {
    fn partial_cmp(&self, other: &IpAddr) -> Option<std::cmp::Ordering> {
        PartialOrd::partial_cmp(&self.as_ipv6addr(), other)
    }
}

impl PartialEq<SIpAddr> for SIpv6Addr {
    fn eq(&self, other: &SIpAddr) -> bool {
        PartialEq::eq(&self.as_ipv6addr(), &other.as_ipaddr())
    }
}

impl PartialOrd<SIpAddr> for SIpv6Addr {
    fn partial_cmp(&self, other: &SIpAddr) -> Option<std::cmp::Ordering> {
        PartialOrd::partial_cmp(&self.as_ipv6addr(), &other.as_ipaddr())
    }
}
