#![deny(improper_ctypes_definitions)]

//! This won't compile if any of the tested types are found to not be
//! FFI-safe

use repr_c_types;

#[test]
fn ffi_safe() {
    extern "C" fn _test(
        // Tuples
        _: repr_c_types::tuples::STuple2<u8, u8>,
        // std
        _: repr_c_types::std::option::SOption<u8>,
        _: repr_c_types::std::result::SResult<u8, u8>,
        // std::sync
        _: repr_c_types::std::sync::SArcOpaque,
        // std::net
        _: repr_c_types::std::net::SIpAddr,
        _: repr_c_types::std::net::SIpv4Addr,
        _: repr_c_types::std::net::SIpv6Addr,
        _: repr_c_types::std::net::SSocketAddr,
        _: repr_c_types::std::net::SSocketAddrV4,
        _: repr_c_types::std::net::SSocketAddrV6,
        _: repr_c_types::std::net::STcpStreamRef,
        _: repr_c_types::std::net::STcpStreamOwned,
    ) {
    }
}
