//! Modes handle memory management differences between the sync and async runtime.
//!
//! Whenever a new frame is created it's pushed to the GC frame stack and popped from this stack
//! when it's dropped. There are some minor differences between the sync and async runtime how
//! this pushing and popping works, which is handled by the `Mode` trait provided by this module.

/// Handle memory management differences between the sync and async runtime.
pub trait Mode: Copy + private::ModePriv {}

/// Mode used by the sync runtime.
#[derive(Clone, Copy)]
pub struct Sync;

impl Mode for Sync {}

cfg_if::cfg_if! {
    if #[cfg(feature = "async")] {
        use std::{cell::Cell, ffi::c_void};

        /// Mode used by the async runtime.
        #[derive(Clone, Copy)]
        pub struct Async<'frame>{
            top: &'frame Cell<*mut c_void>
        }

        impl<'frame> Async<'frame> {
            // Safety: top must be a reference to the slot that stores the top pointer of the
            // backing AsyncStackPage. The AsyncStackPage must have been linked into the gc stack
            pub(crate) unsafe fn new(top: &'frame Cell<*mut c_void>) -> Self {
                Async { top }
            }

            // Safety: top must only be changed when pushing or popping frames from the stack, the
            // linked AsyncStackPages and the frames they store must form a single, nested
            // hierarchy. The top pointer must point to the frame at the top of the stack, to
            // the top pointer of another AsyncStackPage, or be a null pointer if no more pages
            // exist.
            pub(crate) unsafe fn top(&self) -> &Cell<*mut c_void> {
                self.top
            }
        }

        impl<'frame> Mode for Async<'frame> {
        }
    }
}

pub(crate) mod private {
    use crate::{memory::mode::Sync, private::Private};
    use std::ptr::{null_mut, NonNull};
    use std::{cell::Cell, ffi::c_void};

    pub trait ModePriv {
        // Safety: must only be called when a new scope is entered, pop_frame
        // must be called when leaving the scope with the same raw_frame
        unsafe fn push_frame(&self, raw_frame: &[Cell<*mut c_void>], _: Private);
        // Safety: see push_frame
        unsafe fn pop_frame(&self, raw_frame: &[Cell<*mut c_void>], _: Private);
    }

    cfg_if::cfg_if! {
        if #[cfg(all(feature = "lts", not(feature = "all-features-override")))] {
            impl ModePriv for Sync {
                unsafe fn push_frame(&self, raw_frame: &[Cell<*mut c_void>], _: Private) {
                    let rtls = NonNull::new_unchecked(jl_sys::jl_get_ptls_states()).as_mut();
                    raw_frame[0].set(null_mut());
                    raw_frame[1].set(rtls.pgcstack.cast());
                    rtls.pgcstack = raw_frame[..].as_ptr() as *const _ as *mut _;
                }

                unsafe fn pop_frame(&self, _raw_frame: &[Cell<*mut c_void>], _: Private) {
                    let rtls = NonNull::new_unchecked(jl_sys::jl_get_ptls_states()).as_mut();
                    rtls.pgcstack = NonNull::new_unchecked(rtls.pgcstack).as_ref().prev;
                }
            }
        } else {
            use jl_sys::{jl_get_current_task, jl_task_t};
            impl ModePriv for Sync {
                unsafe fn push_frame(&self, raw_frame: &[Cell<*mut c_void>], _: Private) {
                    let task = NonNull::new_unchecked(jl_get_current_task().cast::<jl_task_t>()).as_mut();
                    raw_frame[0].set(null_mut());
                    raw_frame[1].set(task.gcstack.cast());
                    task.gcstack = raw_frame[..].as_ptr() as *const _ as *mut _;
                }

                unsafe fn pop_frame(&self, _raw_frame: &[Cell<*mut c_void>], _: Private) {
                    let task = NonNull::new_unchecked(jl_get_current_task().cast::<jl_task_t>()).as_mut();
                    task.gcstack = NonNull::new_unchecked(task.gcstack).as_ref().prev;
                }
            }
        }
    }

    cfg_if::cfg_if! {
        if #[cfg(feature = "async")] {
            use super::Async;
            impl<'frame> ModePriv for Async<'frame> {
                unsafe fn push_frame(&self, raw_frame: &[Cell<*mut c_void>], _: Private) {
                    raw_frame[0].set(null_mut());
                    raw_frame[1].set(self.top().get());
                    self.top().set(raw_frame.as_ptr() as *const _ as *mut _);
                }

                unsafe fn pop_frame(&self, raw_frame: &[Cell<*mut c_void>], _: Private) {
                    self.top().set(raw_frame[1].get());
                }
            }
        }
    }
}
