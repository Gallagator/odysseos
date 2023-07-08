#![no_std]
#![cfg_attr(not(test), no_main)]

mod memory;
mod synch;

use kernel_boot;
use kernel_boot_interface;
use kernel_cpu;
use memory::palloc;

unsafe fn put_white(x: u64, y: u64, binfo: &kernel_boot_interface::BootInfo) {
    let ptr = (binfo.frame_buffer.phys_address + binfo.hhdm.base) as *mut u8;
    let offset = y * binfo.frame_buffer.pitch + x * 4;
    *(ptr.offset(offset as isize) as *mut u32) = 0xffff_ffff;
}

#[no_mangle]
pub unsafe extern "C" fn _kernel_start() -> ! {
    let boot_info = kernel_boot::arch_init();

    palloc::init(&boot_info.hhdm, &boot_info.memmap);

    let a = palloc::get_page();
    let b = palloc::get_page();

    for i in 0..100 {
        put_white(i, i, &boot_info);
    }
    kernel_cpu::hcf();
}

#[cfg(not(test))]
#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    kernel_cpu::hcf();
}

#[cfg(test)]
fn main() {}
