#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use kernel_boot_interface::BootInfo;
// TODO: DONT USE KERNEL SERIAL DIRECTLY...
use kernel_log::{kprint, kprintln};

pub trait Testable {
    fn run(&self, boot_info: &BootInfo) -> ();
}

impl<T> Testable for T
where
    T: Fn(&BootInfo),
{
    fn run(&self, boot_info: &BootInfo) {
        kprint!("{}...\t", core::any::type_name::<T>());
        self(boot_info);
        kprintln!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    let boot_info = kernel_boot::arch_init();
    kprintln!("Running {} tests", tests.len());
    for test in tests {
        test.run(boot_info);
    }
    kernel_shutdown::shutdown(kernel_shutdown::ShutdownExitCode::Success);
}

pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    kernel_shutdown::shutdown(kernel_shutdown::ShutdownExitCode::Failed);
    kernel_cpu::hcf();
}
