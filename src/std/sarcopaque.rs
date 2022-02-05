use std::fmt::Debug;
use std::sync::Arc;

/// Opaque SArc for keeping the reference count
///
/// Basically the normal `SArc`, except it doesn't expose the inner type
/// and has no way to access it.
/// Useful when your inner type is not FFI-safe and you just want to
/// keep track of references.
#[repr(C)]
pub struct SArcOpaque {
    raw: *const (),
    drop: unsafe fn(*const ()),
}

impl SArcOpaque {
    /// Constructs a SArcOpaque from an Arc
    pub fn new<T>(arc: Arc<T>) -> Self {
        let raw = Arc::into_raw(arc) as *const ();

        unsafe fn drop<T>(raw: *const ()) {
            unsafe {
                Arc::from_raw(raw as *const T);
            }
        }
        Self {
            raw,
            drop: drop::<T>,
        }
    }
}

impl Drop for SArcOpaque {
    fn drop(&mut self) {
        unsafe {
            (self.drop)(self.raw);
        }
    }
}

impl Debug for SArcOpaque {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{SArcOpaque}}")
    }
}
