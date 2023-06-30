#![no_std]
#![no_main]

use kernel_boot;
use kernel_boot_interface;
use kernel_cpu;

unsafe fn put_white(x: u64, y: u64, binfo: &kernel_boot_interface::BootInfo) {
    let ptr = (binfo.frame_buffer.phys_address + binfo.hhdm.base) as *mut u8;
    let offset = y * binfo.frame_buffer.pitch + x * 4;
    *(ptr.offset(offset as isize) as *mut u32) = 0xffff_ffff;
}

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    let boot_info = kernel_boot::arch_init();
    // Ensure we got a framebuffer.
    for i in 0..100 {
        put_white(i, i, &boot_info);
    }
    kernel_cpu::hcf();
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    kernel_cpu::hcf();
}
