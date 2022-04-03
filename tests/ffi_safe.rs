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
        _: safe_types::SMutStr<'static>,
        _: safe_types::SSlice<'static, u8>,
        _: safe_types::SMutSlice<'static, u8>,
        // Primitives: Tuples
        _: safe_types::STuple2<u8, u8>,
        // std
        _: safe_types::std::option::SOption<u8>,
        _: safe_types::std::result::SResult<u8, u8>,
        _: safe_types::std::boxed::SBox<u8>,
        _: safe_types::std::error::SDynError,
        _: safe_types::std::vec::SVec<u8>,
        // std::io
        _: safe_types::std::io::SError,
        _: safe_types::std::io::SErrorKind,
        _: safe_types::std::io::SResult<u8>,
        // std::sync
        _: safe_types::std::sync::SArcOpaque,
        // std::string
        _: safe_types::std::string::SString,
        // std::time
        _: safe_types::std::time::SDuration,
        // std::net
        _: safe_types::std::net::SIpAddr,
        _: safe_types::std::net::SIpv4Addr,
        _: safe_types::std::net::SIpv6Addr,
        _: safe_types::std::net::SSocketAddr,
        _: safe_types::std::net::SSocketAddrV4,
        _: safe_types::std::net::SSocketAddrV6,
        _: safe_types::std::net::STcpStream,
        _: safe_types::std::net::SShutdown,
        // std::task
        _: safe_types::std::task::SContext,
        _: safe_types::std::task::SPoll<u8>,
        // std::borrow
        _: safe_types::std::borrow::SCow<safe_types::SStr<'static>>,
    ) {
    }
}
