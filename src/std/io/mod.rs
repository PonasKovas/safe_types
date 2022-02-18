mod serror;
mod serrorkind;

pub use serror::SError;
pub use serrorkind::SErrorKind;

/// A specialized Result type for I/O operations.
///
/// See documentation of [`std::io::Result`]
pub type SResult<T> = crate::std::result::SResult<T, SError>;
