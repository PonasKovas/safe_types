#![deny(unsafe_op_in_unsafe_fn)]

mod phantom_type;
mod refs;
mod sarray;
mod sslice;
mod sstr;
pub mod std;
mod sunit;
mod tuples;

pub use refs::{Immutable, Mutable};
pub use sarray::SArray;
pub use sslice::{SMutSlice, SSlice};
pub use sstr::{SMutStr, SStr};
pub use sunit::SUnit;
pub use tuples::*;

pub(crate) use phantom_type::PhantomType;

/// This will increase when incompatible ABI changes are made.
pub const ABI_VERSION: u32 = 0;
