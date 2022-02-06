//! Some `#[repr(C)]` tuples.

/// `#[repr(C)]` version of `(A, B)`
#[repr(C)]
pub struct STuple2<A, B>(pub A, pub B);
