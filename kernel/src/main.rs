#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel_test::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod memory;
mod panic;
mod synch;

use kernel_boot;
use kernel_boot_interface;
use kernel_cpu;
use kernel_log::kprintln;
use memory::palloc;

unsafe fn put_white(x: u64, y: u64, binfo: &kernel_boot_interface::BootInfo) {
    let ptr = (binfo.frame_buffer.phys_address + binfo.hhdm.base) as *mut u8;
    let offset = y * binfo.frame_buffer.pitch + x * 4;
    *(ptr.offset(offset as isize) as *mut u32) = 0xffff_ffff;
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _kernel_start() -> ! {
    let boot_info = kernel_boot::arch_init();

    palloc::init(&boot_info.hhdm, &boot_info.memmap);

    let a = palloc::get_page();
    let b = palloc::get_page();

    kprintln!("a has address: {:?}\n and here is b's {:?}", a, b);
    for i in 0..100 {
        unsafe { put_white(i, i, &boot_info) };
    }

    #[cfg(test)]
    test_main();

    kernel_cpu::hcf();
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _kernel_start() -> ! {
    test_main();
    kernel_cpu::hcf();
}
