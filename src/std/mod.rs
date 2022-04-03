pub mod borrow;
pub mod boxed;
pub mod error;
pub mod io;
pub mod net;
pub mod option;
pub mod result;
pub mod string;
pub mod sync;
pub mod task;
pub mod time;
pub mod vec;

pub mod prelude {
    pub use crate::std::option::SOption::{self, None as SNone, Some as SSome};
    pub use crate::std::result::SResult::{self, Err as SErr, Ok as SOk};
    pub use crate::std::string::SString;
    pub use crate::std::vec::SVec;
}
