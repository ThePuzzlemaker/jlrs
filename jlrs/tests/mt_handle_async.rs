#[cfg(all(feature = "multi-rt", feature = "async-rt"))]
#[cfg(not(any(feature = "julia-1-6", feature = "julia-1-7", feature = "julia-1-8")))]
mod mt_handle {
    use std::thread;

    use jlrs::{
        data::managed::value::Value,
        memory::scope::LocalScope,
        runtime::{builder::Builder, executor::tokio_exec::Tokio},
    };

    #[test]
    fn call_from_spawned_and_main_thread() {
        let tokio = Tokio::<1>::new(false);
        let (mut julia, async_handle, th) = Builder::new().async_runtime(tokio).spawn_mt().unwrap();

        let t1 = thread::spawn(move || {
            julia.with(|handle| {
                handle.local_scope::<_, 1>(|mut frame| unsafe {
                    Value::eval_string(&mut frame, "1 + 2")
                        .unwrap()
                        .unbox::<isize>()
                })
            })
        });

        let blocking_task_res = async_handle
            .blocking_task(|mut frame| unsafe {
                Value::eval_string(&mut frame, "3 + 4")
                    .unwrap()
                    .unbox::<isize>()
            })
            .try_dispatch()
            .ok()
            .unwrap()
            .blocking_recv()
            .unwrap()
            .unwrap();

        assert_eq!(t1.join().unwrap().unwrap(), 3);
        assert_eq!(blocking_task_res, 7);

        std::mem::drop(async_handle);
        th.join().unwrap();
    }
}
