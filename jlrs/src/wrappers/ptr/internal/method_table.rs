//! Wrapper for `MethodTable`.
//!
//! The documentation for this module has been slightly adapted from the comments for this struct
//! in [`julia.h`]
//!
//! [`julia.h`]: https://github.com/JuliaLang/julia/blob/96786e22ccabfdafd073122abb1fb69cea921e17/src/julia.h#L535

use crate::{
    impl_debug, impl_julia_typecheck,
    memory::output::Output,
    private::Private,
    wrappers::ptr::{
        array::ArrayRef, module::ModuleRef, private::WrapperPriv, symbol::SymbolRef,
        value::ValueRef, Ref,
    },
};
use cfg_if::cfg_if;
use jl_sys::{jl_methtable_t, jl_methtable_type};
use std::{marker::PhantomData, ptr::NonNull};

cfg_if! {
    if #[cfg(any(not(feature = "lts"), feature = "all-features-override"))] {
        use jl_sys::jl_value_t;
        use crate::wrappers::ptr::atomic_value;
        use std::sync::atomic::Ordering;
    }
}

/// contains the TypeMap for one Type
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct MethodTable<'scope>(NonNull<jl_methtable_t>, PhantomData<&'scope ()>);

impl<'scope> MethodTable<'scope> {
    /*
    for (a, b) in zip(fieldnames(Core.MethodTable), fieldtypes(Core.MethodTable))
        println(a, ": ", b)
    end
    name: Symbol
    defs: Any _Atomic
    leafcache: Any _Atomic
    cache: Any _Atomic
    max_args: Int64
    kwsorter: Any
    module: Module
    backedges: Vector{Any}
    : Int64
    : Int64
    offs: UInt8
    : UInt8
    */

    /// Sometimes a hack used by serialization to handle kwsorter
    pub fn name(self) -> SymbolRef<'scope> {
        // Safety: the pointer points to valid data
        unsafe { SymbolRef::wrap(self.unwrap_non_null(Private).as_ref().name) }
    }

    /// The `defs` field.
    pub fn defs(self) -> ValueRef<'scope, 'static> {
        cfg_if! {
            if #[cfg(all(feature = "lts", not(feature = "all-features-override")))] {
                // Safety: the pointer points to valid data
                unsafe { ValueRef::wrap(self.unwrap_non_null(Private).as_ref().defs) }
            } else {
                // Safety: the pointer points to valid data
                unsafe {
                    let defs = atomic_value::<jl_value_t>(&self.unwrap_non_null(Private).as_mut().defs as *const _);
                    let ptr = defs.load(Ordering::Relaxed);
                    ValueRef::wrap(ptr)
                }
            }
        }
    }

    /// The `leafcache` field.
    pub fn leafcache(self) -> ArrayRef<'scope, 'static> {
        cfg_if! {
            if #[cfg(all(feature = "lts", not(feature = "all-features-override")))] {
                // Safety: the pointer points to valid data
                unsafe { ArrayRef::wrap(self.unwrap_non_null(Private).as_ref().leafcache) }
            } else {
                // Safety: the pointer points to valid data
                unsafe {
                    let leafcache =
                        atomic_value::<jl_value_t>(&self.unwrap_non_null(Private).as_mut().leafcache as *const _);
                    let ptr = leafcache.load(Ordering::Relaxed);
                    ArrayRef::wrap(ptr.cast())
                }
            }
        }
    }

    /// The `cache` field.
    pub fn cache(self) -> ValueRef<'scope, 'static> {
        cfg_if! {
            if #[cfg(all(feature = "lts", not(feature = "all-features-override")))] {
                // Safety: the pointer points to valid data
                unsafe { ValueRef::wrap(self.unwrap_non_null(Private).as_ref().cache) }
            } else {
                // Safety: the pointer points to valid data
                unsafe {
                    let cache = atomic_value::<jl_value_t>(&self.unwrap_non_null(Private).as_mut().cache as *const _);
                    let ptr = cache.load(Ordering::Relaxed);
                    ValueRef::wrap(ptr)
                }
            }
        }
    }

    /// Max # of non-vararg arguments in a signature
    pub fn max_args(self) -> isize {
        // Safety: the pointer points to valid data
        unsafe { self.unwrap_non_null(Private).as_ref().max_args }
    }

    /// Keyword argument sorter function
    pub fn kw_sorter(self) -> ValueRef<'scope, 'static> {
        // Safety: the pointer points to valid data
        unsafe { ValueRef::wrap(self.unwrap_non_null(Private).as_ref().kwsorter) }
    }

    /// Used for incremental serialization to locate original binding
    pub fn module(self) -> ModuleRef<'scope> {
        // Safety: the pointer points to valid data
        unsafe { ModuleRef::wrap(self.unwrap_non_null(Private).as_ref().module) }
    }

    /// The `backedges` field.
    pub fn backedges(self) -> ArrayRef<'scope, 'static> {
        // Safety: the pointer points to valid data
        unsafe { ArrayRef::wrap(self.unwrap_non_null(Private).as_ref().backedges) }
    }

    /// 0, or 1 to skip splitting typemap on first (function) argument
    pub fn offs(self) -> u8 {
        // Safety: the pointer points to valid data
        unsafe { self.unwrap_non_null(Private).as_ref().offs }
    }

    /// Whether this accepts adding new methods
    pub fn frozen(self) -> u8 {
        // Safety: the pointer points to valid data
        unsafe { self.unwrap_non_null(Private).as_ref().frozen }
    }

    /// Use the `Output` to extend the lifetime of this data.
    pub fn root<'target>(self, output: Output<'target>) -> MethodTable<'target> {
        // Safety: the pointer points to valid data
        unsafe {
            let ptr = self.unwrap_non_null(Private);
            output.set_root::<MethodTable>(ptr);
            MethodTable::wrap_non_null(ptr, Private)
        }
    }
}

impl_julia_typecheck!(MethodTable<'scope>, jl_methtable_type, 'scope);
impl_debug!(MethodTable<'_>);

impl<'scope> WrapperPriv<'scope, '_> for MethodTable<'scope> {
    type Wraps = jl_methtable_t;
    const NAME: &'static str = "<MethodTable";

    // Safety: `inner` must not have been freed yet, the result must never be
    // used after the GC might have freed it.
    unsafe fn wrap_non_null(inner: NonNull<Self::Wraps>, _: Private) -> Self {
        Self(inner, PhantomData)
    }

    fn unwrap_non_null(self, _: Private) -> NonNull<Self::Wraps> {
        self.0
    }
}

impl_root!(MethodTable, 1);

/// A reference to a [`MethodTable`] that has not been explicitly rooted.
pub type MethodTableRef<'scope> = Ref<'scope, 'static, MethodTable<'scope>>;
impl_valid_layout!(MethodTableRef, MethodTable);
impl_ref_root!(MethodTable, MethodTableRef, 1);