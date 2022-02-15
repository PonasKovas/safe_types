#![deny(unsafe_op_in_unsafe_fn)]

mod array;
mod phantom_type;
mod refs;
mod sslice;
mod sstr;
pub mod std;
mod tuples;

pub use array::SArray;
pub use refs::{Immutable, Mutable};
pub use sslice::{SMutSlice, SSlice};
pub use sstr::{SMutStr, SStr};
pub use tuples::*;

pub(crate) use phantom_type::PhantomType;

/// This will increase when incompatible ABI changes are made.
pub const ABI_VERSION: u32 = 0;
