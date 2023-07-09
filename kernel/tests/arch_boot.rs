#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel_test::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[no_mangle]
pub extern "C" fn _kernel_start() -> ! {
    test_main();
    kernel_cpu::hcf();
}

#[test_case]
fn arch_boot() {
    let _boot_info = kernel_boot::arch_init();
}

#[panic_handler]
pub fn test_panic(info: &core::panic::PanicInfo) -> ! {
    kernel_test::panic(info);
}
