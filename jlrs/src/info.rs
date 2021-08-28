//! System and Julia version information.

use crate::{
    memory::global::Global,
    private::Private,
    wrappers::ptr::private::Wrapper,
    wrappers::ptr::{symbol::Symbol, value::Value},
};
use jl_sys::{
    jl_cpu_threads, jl_get_ARCH, jl_get_UNAME, jl_get_libllvm, jl_getallocationgranularity,
    jl_getpagesize, jl_git_branch, jl_git_commit, jl_is_debugbuild, jl_n_threads,
    jl_ver_is_release, jl_ver_major, jl_ver_minor, jl_ver_patch, jl_ver_string,
};
use std::{ffi::CStr, marker::PhantomData};

/// Global Julia information.
pub struct Info {
    _priv: PhantomData<*mut ()>,
}

impl Info {
    pub(crate) fn new() -> Self {
        Info { _priv: PhantomData }
    }

    /// Number of threads the CPU supports.
    pub fn n_cpu_threads(&self) -> usize {
        unsafe { jl_cpu_threads() as usize }
    }

    /// Number of threads Julia can use.
    pub fn n_threads(&self) -> usize {
        unsafe { jl_n_threads as usize }
    }

    /// The page size used by the garbage collector.
    pub fn page_size(&self) -> usize {
        unsafe { jl_getpagesize() as usize }
    }

    /// The allocation granularity.
    pub fn allocation_granularity(&self) -> usize {
        unsafe { jl_getallocationgranularity() as usize }
    }

    /// Returns `true` if a debug build of Julia is used.
    pub fn is_debugbuild(&self) -> bool {
        unsafe { jl_is_debugbuild() != 0 }
    }

    /// Name and information of the kernel.
    pub fn uname(&self) -> StrOrBytes<'static> {
        unsafe {
            let cstr = Symbol::wrap(jl_get_UNAME(), Private).as_cstr();

            if let Ok(rstr) = cstr.to_str() {
                Ok(rstr)
            } else {
                Err(cstr.to_bytes())
            }
        }
    }

    /// The CPU architecture.
    pub fn arch(&self) -> StrOrBytes<'static> {
        unsafe {
            let cstr = Symbol::wrap(jl_get_ARCH(), Private).as_cstr();

            if let Ok(rstr) = cstr.to_str() {
                Ok(rstr)
            } else {
                Err(cstr.to_bytes())
            }
        }
    }

    /// The path to the llvm library used by Julia.
    pub fn libllvm_path(&self) -> Option<StrOrBytes<'static>> {
        unsafe {
            let sym = jl_get_libllvm();
            if sym == Value::nothing(Global::new()).unwrap(Private) {
                return None;
            }

            let cstr = Symbol::wrap(sym.cast(), Private).as_cstr();

            let v = if let Ok(rstr) = cstr.to_str() {
                Ok(rstr)
            } else {
                Err(cstr.to_bytes())
            };

            Some(v)
        }
    }

    /// The major version of Julia.
    pub fn major_version(&self) -> isize {
        unsafe { jl_ver_major() as isize }
    }

    /// The minor version of Julia.
    pub fn minor_version(&self) -> isize {
        unsafe { jl_ver_minor() as isize }
    }

    /// The patch version of Julia.
    pub fn patch_version(&self) -> isize {
        unsafe { jl_ver_patch() as isize }
    }

    /// Returns true if a release version of Julia is used.
    pub fn is_release(&self) -> bool {
        unsafe { jl_ver_is_release() != 0 }
    }

    /// Returns the git branch that was used to compile the used version of Julia.
    pub fn git_branch(&self) -> StrOrBytes<'static> {
        unsafe {
            let cstr = CStr::from_ptr(jl_git_branch());

            if let Ok(rstr) = cstr.to_str() {
                Ok(rstr)
            } else {
                Err(cstr.to_bytes())
            }
        }
    }

    /// Returns the git commit that was used to compile the used version of Julia.
    pub fn git_commit(&self) -> &'static str {
        unsafe { CStr::from_ptr(jl_git_commit()).to_str().unwrap() }
    }

    /// Returns the version string of the used version of Julia.
    pub fn version_string(&self) -> &'static str {
        unsafe { CStr::from_ptr(jl_ver_string()).to_str().unwrap() }
    }
}

/// Alias for a result that contains either a valid UTF8-encoded string slice, or the raw byte
/// slice if the contents are not valid UTF8.
pub type StrOrBytes<'scope> = Result<&'scope str, &'scope [u8]>;