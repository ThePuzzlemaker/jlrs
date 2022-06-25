//! Wrapper for `WeakRef`.

use crate::{
    impl_debug, impl_julia_typecheck,
    memory::output::Output,
    private::Private,
    wrappers::ptr::{private::WrapperPriv, value::ValueRef, Ref},
};
use jl_sys::{jl_weakref_t, jl_weakref_type};
use std::{marker::PhantomData, ptr::NonNull};

/// A weak reference.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct WeakRef<'scope>(NonNull<jl_weakref_t>, PhantomData<&'scope ()>);

impl<'scope> WeakRef<'scope> {
    /*
    for (a, b) in zip(fieldnames(WeakRef), fieldtypes(WeakRef))
        println(a, ": ", b)
    end
    value: Any
    */

    /// The referenced `Value`.
    pub fn value(self) -> ValueRef<'scope, 'static> {
        // Safety: the pointer points to valid data
        unsafe { ValueRef::wrap(self.unwrap_non_null(Private).as_ref().value) }
    }

    /// Use the `Output` to extend the lifetime of this data.
    pub fn root<'target>(self, output: Output<'target>) -> WeakRef<'target> {
        // Safety: the pointer points to valid data
        unsafe {
            let ptr = self.unwrap_non_null(Private);
            output.set_root::<WeakRef>(ptr);
            WeakRef::wrap_non_null(ptr, Private)
        }
    }
}

impl_julia_typecheck!(WeakRef<'scope>, jl_weakref_type, 'scope);
impl_debug!(WeakRef<'_>);

impl<'scope> WrapperPriv<'scope, '_> for WeakRef<'scope> {
    type Wraps = jl_weakref_t;
    const NAME: &'static str = "WeakRef";

    // Safety: `inner` must not have been freed yet, the result must never be
    // used after the GC might have freed it.
    unsafe fn wrap_non_null(inner: NonNull<Self::Wraps>, _: Private) -> Self {
        Self(inner, PhantomData)
    }

    fn unwrap_non_null(self, _: Private) -> NonNull<Self::Wraps> {
        self.0
    }
}

impl_root!(WeakRef, 1);

/// A reference to a [`WeakRef`] that has not been explicitly rooted.
pub type WeakRefRef<'scope> = Ref<'scope, 'static, WeakRef<'scope>>;
impl_valid_layout!(WeakRefRef, WeakRef);
impl_ref_root!(WeakRef, WeakRefRef, 1);