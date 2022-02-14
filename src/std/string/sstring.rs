use crate::std::prelude::SVec;

/// FFI-safe equivalent of `String`
///
/// See documentation of [`std::string::String`]
#[repr(C)]
pub struct SString {
    inner: SVec<u8>,
}
