use crate::std::{io::RefDynWrite, prelude::SOption};

/// Configuration for formatting.
///
/// See documentation of [`std::fmt::Formatter`]
#[repr(C)]
pub struct SFormatter<'a> {
    flags: u32,
    fill: char,
    alignment: Alignment,
    width: SOption<usize>,
    precision: SOption<usize>,

    buf: RefDynWrite<'a>,
}

#[repr(C)]
enum Alignment {
    Left,
    Right,
    Center,
}
