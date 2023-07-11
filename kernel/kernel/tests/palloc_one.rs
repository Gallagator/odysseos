#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel_test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::num::NonZeroUsize;

use kernel_boot_interface::{memmap::BootMemType, BootInfo};
use odysseos::memory::{self, memmap::get_addr_entry, palloc::free_page};
use teensy_std::addr::Addr;

#[no_mangle]
pub extern "C" fn _kernel_start() -> ! {
    test_main();
    kernel_cpu::hcf();
}

#[test_case]
fn palloc_one_and_write(boot_info: &BootInfo) {
    memory::palloc::init(&boot_info.hhdm, &boot_info.memmap);
    let ppage = memory::palloc::get_page();
    assert!(get_addr_entry(&boot_info.memmap, &ppage).typ == BootMemType::Usable);

    let vpage = ppage
        .map_addr(|addr| addr.checked_add(boot_info.hhdm.base).unwrap())
        .as_ptr::<u8>()
        .unwrap();
    for i in 0..(kernel_paging::PAGE_SIZE_MIN as isize) {
        unsafe {
            *vpage.as_ptr().offset(i) = 0xCC;
        }
    }
    for i in 0..(kernel_paging::PAGE_SIZE_MIN as isize) {
        unsafe {
            assert_eq!(*vpage.as_ptr().offset(i), 0xCC);
        }
    }

    let ppage: Addr = Some(vpage).into();
    memory::palloc::free_page(
        ppage.map_addr(|addr| NonZeroUsize::new(usize::from(addr) - boot_info.hhdm.base).unwrap()),
    );
}

//#[test_case]
//fn palloc_hhdm(boot_info: &BootInfo) {
//    while let Some(page) = memory::palloc::get_page().as_ptr::<u8>() {
//        if page.addr().get() > boot_info.hhdm.max_len() {
//            break;
//        }
//
//        let vpage = page
//            .map_addr(|addr| NonZeroUsize::new(usize::from(addr) + boot_info.hhdm.base).unwrap());
//        for i in 0..(kernel_paging::PAGE_SIZE_MIN as isize) {
//            unsafe {
//                *vpage.as_ptr().offset(i) = 0xCC;
//            }
//        }
//    }
//}
