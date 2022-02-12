#![deny(unsafe_op_in_unsafe_fn)]

mod array;
mod sslice;
mod sstr;
pub mod std;
mod tuples;

pub use array::SArray;
pub use sslice::SSlice;
pub use sstr::SStr;
pub use tuples::*;
