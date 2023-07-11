#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel_test::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod memory;
mod panic;
pub mod synch;

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _kernel_start() -> ! {
    test_main();
    kernel_cpu::hcf();
}
