#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

// TODO: DONT USE KERNEL SERIAL DIRECTLY...
use kernel_log::{kprint, kprintln};

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        kprint!("{}...\t", core::any::type_name::<T>());
        self();
        kprintln!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    kprintln!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    kernel_shutdown::shutdown(kernel_shutdown::ShutdownExitCode::Success);
}

pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    kernel_shutdown::shutdown(kernel_shutdown::ShutdownExitCode::Failed);
    kernel_cpu::hcf();
}
