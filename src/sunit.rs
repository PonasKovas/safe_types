use std::fmt::Debug;

/// The `()` type, also called “unit”.
///
/// See documentation of [`unit`]
#[derive(Hash, PartialOrd, Ord, PartialEq, Eq)]
#[repr(C)]
pub struct SUnit {
    _a: [u8; 0], // seems to be the only way currently
}

impl SUnit {
    pub const fn new() -> Self {
        Self { _a: [] }
    }
}

impl Debug for SUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "()")
    }
}

impl Default for SUnit {
    fn default() -> Self {
        Self::new()
    }
}

impl From<()> for SUnit {
    fn from(_: ()) -> Self {
        Self::new()
    }
}

impl From<SUnit> for () {
    fn from(_: SUnit) -> Self {
        ()
    }
}
