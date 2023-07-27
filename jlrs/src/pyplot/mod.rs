//! Plot data with Plots.jl and PyPlot.jl
//!
//! In order to use this module PyPlot.jl must have been installed, as well as GTK3. GTK3 is
//! currently the only supported GUI. You must use a `matplotlibrc` file that sets the backend
//! to `Gtk3Agg`.
//!
//! When multiple figures are open, only the most recently opened one is updated automatically.

#[cfg(feature = "async")]
use jlrs_macros::julia_version;

use crate::{
    args::Values,
    call::{Call, ProvideKeywords},
    convert::into_jlrs_result::IntoJlrsResult,
    data::managed::{
        erase_scope_lifetime, function::Function, module::Module, value::Value, Managed,
    },
    error::JlrsResult,
    memory::target::{frame::GcFrame, Target},
    private::Private,
};
#[cfg(feature = "async")]
use crate::{call::CallAsync, memory::target::frame::AsyncGcFrame};

init_fn!(init_jlrs_py_plot, JLRS_PY_PLOT_JL, "JlrsPyPlot.jl");

/// A handle to a plotting window.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct PyPlot<'scope>(Value<'scope, 'static>);

impl<'scope> PyPlot<'scope> {
    /// This metod must be called before this module can be used.
    pub fn init<'frame>(frame: &mut GcFrame<'frame>) {
        if Module::main(&frame).submodule(&frame, "JlrsPyPlot").is_ok() {
            return;
        }
        unsafe { init_jlrs_py_plot(frame) };
    }

    /// Create a new plotting window by calling `plot_fn(args...)`. The window stays open until it
    /// has been closed, even if all handles have been dropped. `plot_fn` must be a plotting
    /// function from the Plots.jl package, such as `plot` or `hexbin`. The resources associated
    /// with the window are only cleaned up if one of the `PyPlot::wait` methods is called.
    pub unsafe fn new<'value, V, const N: usize>(
        frame: &mut GcFrame<'scope>,
        plot_fn: Function<'value, 'static>,
        args: V,
    ) -> JlrsResult<Self>
    where
        V: Values<'value, 'static, N>,
    {
        let values = args.into_extended_with_start([plot_fn.as_value()], Private);

        let plt = Module::main(&frame)
            .submodule(&frame, "JlrsPyPlot")
            .unwrap()
            .as_managed()
            .function(&frame, "jlrsplot")
            .unwrap()
            .as_managed()
            .call(frame, values.as_ref())
            .into_jlrs_result()?;

        Ok(PyPlot(plt))
    }

    /// Create a new plotting window by calling `plotfn(args...; keywords)`. The window stays open
    /// until it has been closed, even if all handles have been dropped. `plot_fn` must be a
    /// plotting function from the Plots.jl package, such as `plot` or `hexbin`. The resources
    /// associated  with the window are only cleaned up if one of the `PyPlot::wait` methods is
    /// called.
    pub unsafe fn new_with_keywords<'value, V, const N: usize>(
        frame: &mut GcFrame<'scope>,
        plot_fn: Function<'value, 'static>,
        args: V,
        keywords: Value<'value, 'static>,
    ) -> JlrsResult<Self>
    where
        V: Values<'value, 'static, N>,
    {
        let values = args.into_extended_with_start([plot_fn.as_value()], Private);

        let plt = Module::main(&frame)
            .submodule(&frame, "JlrsPyPlot")
            .unwrap()
            .as_managed()
            .function(&frame, "jlrsplot")
            .unwrap()
            .as_managed()
            .provide_keywords(keywords)?
            .call(frame, values.as_ref())
            .into_jlrs_result()?;

        Ok(PyPlot(plt))
    }

    /// Update an existing plotting window by calling
    /// `plot)fn(<plot associated with self>, args...)`. If the window has already been closed an
    /// error is returned. Note that if multiple plotting windows are currently open, only the
    /// most recently created one is redrawn automatically.
    pub unsafe fn update<'value, 'frame, V, const N: usize>(
        self,
        frame: &mut GcFrame<'scope>,
        plot_fn: Function<'value, 'static>,
        args: V,
    ) -> JlrsResult<isize>
    where
        V: Values<'value, 'static, N>,
    {
        let values = args.into_extended_with_start([plot_fn.as_value()], Private);

        Module::main(&frame)
            .submodule(&frame, "JlrsPyPlot")
            .unwrap()
            .as_managed()
            .function(&frame, "updateplot!")
            .unwrap()
            .as_managed()
            .call(frame, values.as_ref())
            .into_jlrs_result()?
            .unbox::<isize>()
    }

    /// Update an existing plotting window by calling
    /// `plot_fn(<plot associated with self>, args...; kwargs...)`. If the window has already been
    /// closed an error is returned. Note that if multiple plotting windows are currently open,
    /// only the most recently created one is redrawn automatically.
    pub unsafe fn update_with_keywords<'value, 'frame, V, const N: usize>(
        self,
        frame: &mut GcFrame<'scope>,
        plot_fn: Function<'value, 'static>,
        args: V,
        keywords: Value<'value, 'static>,
    ) -> JlrsResult<isize>
    where
        V: Values<'value, 'static, N>,
    {
        let values = args
            .into_extended_with_start([erase_scope_lifetime(self.0), plot_fn.as_value()], Private);

        Module::main(&frame)
            .submodule(&frame, "JlrsPyPlot")
            .unwrap()
            .as_managed()
            .function(&frame, "updateplot!")
            .unwrap()
            .as_managed()
            .provide_keywords(keywords)?
            .call(frame, values.as_ref())
            .into_jlrs_result()?
            .unbox::<isize>()
    }

    /// Wait until the window associated with `self` has been closed.
    pub fn wait<'frame>(self, frame: &mut GcFrame<'scope>) -> JlrsResult<()> {
        unsafe {
            Module::base(&frame)
                .function(&frame, "wait")?
                .as_managed()
                .call1(frame, self.0)
                .into_jlrs_result()?;

            Ok(())
        }
    }

    /// Whenever a plot is updated with a non-mutating plotting function a new version is
    /// created. Because all versions are protected from garbage collection until [`PyPlot::wait`]
    /// has returned, it's possible to change the pending version which will be used as the base
    /// plot when [`PyPlot::update`] is called.
    pub fn set_pending_version<'frame>(
        self,
        frame: &mut GcFrame<'frame>,
        version: isize,
    ) -> JlrsResult<()> {
        frame.scope(|mut frame| unsafe {
            let version = Value::new(&mut frame, version);

            Module::main(&frame)
                .submodule(&frame, "JlrsPyPlot")
                .unwrap()
                .as_managed()
                .function(&frame, "setversion")
                .unwrap()
                .as_managed()
                .call1(frame, version)
                .into_jlrs_result()?;

            Ok(())
        })
    }

    /// Wait until the window associated with `self` has been closed in a new task scheduled
    /// on the main thread.
    #[cfg(feature = "async")]
    pub async fn wait_async_main<'frame>(self, frame: &mut AsyncGcFrame<'frame>) -> JlrsResult<()> {
        unsafe {
            Module::base(&frame)
                .function(&frame, "wait")?
                .as_managed()
                .call_async_main(frame, [self.0])
                .await
                .into_jlrs_result()?;

            Ok(())
        }
    }

    #[cfg(feature = "async")]
    #[julia_version(since = "1.9")]
    /// Wait until the window associated with `self` has been closed in a new task scheduled
    /// on the `:interactive` thread pool.
    pub async fn wait_async_interactive<'frame>(
        self,
        frame: &mut AsyncGcFrame<'frame>,
    ) -> JlrsResult<()> {
        unsafe {
            Module::base(&frame)
                .function(&frame, "wait")?
                .as_managed()
                .call_async_interactive(frame, [self.0])
                .await
                .into_jlrs_result()?;

            Ok(())
        }
    }

    /// Wait until the window associated with `self` has been closed in a new task scheduled
    /// on another thread.
    #[cfg(feature = "async")]
    pub async fn wait_async_local<'frame>(
        self,
        frame: &mut AsyncGcFrame<'frame>,
    ) -> JlrsResult<()> {
        unsafe {
            Module::base(&frame)
                .function(&frame, "wait")?
                .as_managed()
                .call_async_local(frame, [self.0])
                .await
                .into_jlrs_result()?;

            Ok(())
        }
    }

    /// Wait until the window associated with `self` has been closed in a new task scheduled
    /// on another thread.
    #[cfg(feature = "async")]
    pub async fn wait_async<'frame>(self, frame: &mut AsyncGcFrame<'frame>) -> JlrsResult<()> {
        unsafe {
            Module::base(&frame)
                .function(&frame, "wait")?
                .as_managed()
                .call_async(frame, [self.0])
                .await
                .into_jlrs_result()?;

            Ok(())
        }
    }
}

/// This trait is, and can only be, implemented by [`Module`]. It adds the method `Module::plots`
/// that provides access to the contents of the `Plots` package.
pub trait AccessPlotsModule: private::AccessPlotsModulePriv {
    /// Returns the `Plots` module.
    fn plots<'global, T: Target<'global>>(target: &T) -> Module<'global> {
        unsafe {
            Module::main(target)
                .submodule(target, "JlrsPyPlot")
                .unwrap()
                .as_managed()
                .submodule(target, "Plots")
                .unwrap()
                .as_managed()
        }
    }
}

impl<'scope> AccessPlotsModule for Module<'scope> {}

mod private {
    use crate::data::managed::module::Module;

    pub trait AccessPlotsModulePriv {}

    impl<'scope> AccessPlotsModulePriv for Module<'scope> {}
}
