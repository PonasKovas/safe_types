// Used so the compiler would trigger the improper_ctypes_definitions lint
// if T is not FFI-safe
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(C, packed)]
pub(crate) struct PhantomType<T>([T; 0]);

impl<T> PhantomType<T> {
    pub(crate) const fn new() -> Self {
        Self([])
    }
}
