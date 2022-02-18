#![deny(improper_ctypes_definitions)]

//! This won't compile if any of the tested types are found to not be
//! FFI-safe

use safe_types;

#[test]
fn ffi_safe() {
    extern "C" fn _test(
        // Primitives
        _: safe_types::SUnit,
        _: safe_types::SArray<u8, 5>,
        _: safe_types::SStr<'static>,
        // Primitives: Tuples
        _: safe_types::STuple2<u8, u8>,
        // std
        _: safe_types::std::option::SOption<u8>,
        _: safe_types::std::result::SResult<u8, u8>,
        // std::sync
        _: safe_types::std::sync::SArcOpaque,
        // std::net
        _: safe_types::std::net::SIpAddr,
        _: safe_types::std::net::SIpv4Addr,
        _: safe_types::std::net::SIpv6Addr,
        _: safe_types::std::net::SSocketAddr,
        _: safe_types::std::net::SSocketAddrV4,
        _: safe_types::std::net::SSocketAddrV6,
        _: safe_types::std::net::STcpStream,
    ) {
    }
}
