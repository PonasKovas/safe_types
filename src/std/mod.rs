pub mod net;
pub mod option;
pub mod result;
pub mod sync;

pub mod prelude {
    pub use crate::std::option::SOption::{self, None as SNone, Some as SSome};
    pub use crate::std::result::SResult::{self, Err as SErr, Ok as SOk};
}
