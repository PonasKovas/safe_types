use std::{
    hint::unreachable_unchecked,
    marker::PhantomData,
    mem::ManuallyDrop,
    task::{Context, RawWaker, RawWakerVTable, Waker},
};

/// The `Context` of an asynchronous task.
///
/// See documentation of [`std::task::Context`]
#[repr(C)]
pub struct SContext<'a> {
    waker: *const SWaker,
    _phantom: PhantomData<&'a Waker>,
}

#[repr(C)]
struct SWaker {
    vtable: *const VTable,
    waker: WakerUnion,
}

union WakerUnion {
    reference: *const Waker,
    owned: ManuallyDrop<Waker>,
}

#[repr(C)]
struct VTable {
    wake_by_ref: unsafe extern "C" fn(*const SWaker),
    clone: unsafe extern "C" fn(*const SWaker) -> *const SWaker,
    wake: unsafe extern "C" fn(*const SWaker),
    drop: unsafe extern "C" fn(*const SWaker),
}

static REF_VTABLE: VTable = {
    unsafe extern "C" fn wake_by_ref(waker: *const SWaker) {
        // Since this is the Ref variant of the vtable
        // we can assume that the union is reference (not owned)
        unsafe { (*(*waker).waker.reference).wake_by_ref() }
    }
    unsafe extern "C" fn clone(waker: *const SWaker) -> *const SWaker {
        // Since this is the Ref variant of the vtable
        // we can assume that the union is reference (not owned)
        let waker = unsafe { (*(*waker).waker.reference).clone() };

        Box::into_raw(Box::new(SWaker {
            vtable: &OWNED_VTABLE,
            waker: WakerUnion {
                owned: ManuallyDrop::new(waker),
            },
        }))
        .cast()
    }
    unsafe extern "C" fn wake(_: *const SWaker) {
        // Since this is the Ref variant of the vtable
        // this method will never be called, since it takes Waker by value
        unsafe { unreachable_unchecked() }
    }
    unsafe extern "C" fn drop(_: *const SWaker) {
        // Since this is the Ref variant of the vtable
        // this method will never be called, since it takes Waker by value
        unsafe { unreachable_unchecked() }
    }

    VTable {
        wake_by_ref,
        clone,
        wake,
        drop,
    }
};

static OWNED_VTABLE: VTable = {
    unsafe extern "C" fn wake_by_ref(waker: *const SWaker) {
        // Since this is the Owned variant of the vtable
        // we can assume that the union is owned (not reference)
        unsafe { (*(*waker).waker.owned).wake_by_ref() }
    }
    unsafe extern "C" fn clone(waker: *const SWaker) -> *const SWaker {
        // Since this is the Owned variant of the vtable
        // we can assume that the union is owned (not reference)
        let waker = unsafe { (*(*waker).waker.owned).clone() };

        Box::into_raw(Box::new(SWaker {
            vtable: &OWNED_VTABLE,
            waker: WakerUnion {
                owned: ManuallyDrop::new(waker),
            },
        }))
        .cast()
    }
    unsafe extern "C" fn wake(waker: *const SWaker) {
        unsafe {
            let waker = Box::from_raw(waker as *mut SWaker);
            // Since this is the Owned variant of the vtable
            // we can assume that the union is owned (not reference)
            ManuallyDrop::into_inner(waker.waker.owned).wake();
        }
    }
    unsafe extern "C" fn drop(waker: *const SWaker) {
        unsafe {
            let waker = Box::from_raw(waker as *mut SWaker);
            // Since this is the Owned variant of the vtable
            // we can assume that the union is owned (not reference)
            ManuallyDrop::into_inner(waker.waker.owned);
        }
    }

    VTable {
        wake_by_ref,
        clone,
        wake,
        drop,
    }
};

static RUST_RAWWAKER_VTABLE: RawWakerVTable = {
    unsafe fn clone(ptr: *const ()) -> RawWaker {
        let waker = ptr as *const SWaker;

        let cloned = unsafe { ((*(*waker).vtable).clone)(waker) };

        RawWaker::new(cloned.cast(), &RUST_RAWWAKER_VTABLE)
    }
    unsafe fn wake(ptr: *const ()) {
        let waker = ptr as *const SWaker;

        unsafe { ((*(*waker).vtable).wake)(waker) }
    }
    unsafe fn wake_by_ref(ptr: *const ()) {
        let waker = ptr as *const SWaker;

        unsafe { ((*(*waker).vtable).wake_by_ref)(waker) }
    }
    unsafe fn drop(ptr: *const ()) {
        let waker = ptr as *const SWaker;

        unsafe { ((*(*waker).vtable).drop)(waker) }
    }

    RawWakerVTable::new(clone, wake, wake_by_ref, drop)
};

impl<'a> SContext<'a> {
    pub fn from_context<T, F: FnOnce(SContext<'a>) -> T>(ctx: &Context<'a>, closure: F) -> T {
        let waker = SWaker {
            vtable: &REF_VTABLE,
            waker: WakerUnion {
                reference: ctx.waker() as *const _,
            },
        };

        let ctx = Self {
            waker: &waker as *const _,
            _phantom: PhantomData,
        };

        closure(ctx)
    }

    pub fn with_context<T, F: FnOnce(&mut Context) -> T>(&mut self, closure: F) -> T {
        let waker = unsafe {
            ManuallyDrop::new(Waker::from_raw(RawWaker::new(
                &self.waker as *const _ as *const (),
                &RUST_RAWWAKER_VTABLE,
            )))
        };

        let mut context = Context::from_waker(&waker);

        closure(&mut context)
    }
}
