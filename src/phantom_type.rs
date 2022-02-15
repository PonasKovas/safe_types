use std::fmt::{Debug, Display};

// Used so the compiler would trigger the improper_ctypes_definitions lint
// if T is not FFI-safe
#[repr(C, packed)]
pub(crate) struct PhantomType<T>([T; 0]);

impl<T> PhantomType<T> {
    pub(crate) const fn new() -> Self {
        Self([])
    }
}

impl<T> Clone for PhantomType<T> {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl<T> Debug for PhantomType<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl<T> Display for PhantomType<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl<T> PartialEq for PhantomType<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T> Eq for PhantomType<T> {}

impl<T> PartialOrd for PhantomType<T> {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ordering::Equal)
    }
}

impl<T> Ord for PhantomType<T> {
    fn cmp(&self, _other: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}
