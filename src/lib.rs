#![deny(unsafe_op_in_unsafe_fn)]

mod array;
mod sslice;
mod sstr;
pub mod std;
pub mod tuples;

pub use array::SArray;
pub use sslice::SSlice;
pub use sstr::SStr;
