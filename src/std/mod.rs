pub mod net;
pub mod result;
pub mod sync;

pub mod prelude {
    pub use crate::std::result::SResult::{self, Err as SErr, Ok as SOk};
}
