mod serror;
mod serrorkind;

pub use serror::SError;
pub use serrorkind::SErrorKind;

pub type SResult<T> = crate::std::result::SResult<T, SError>;
