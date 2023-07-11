use core::{num::NonZeroUsize, ptr::NonNull};
use spin;

use kernel_boot_interface::{
    hhdm::BootHhdm,
    memmap::{BootMemType, Memmap, MemmapEntry},
};
use metamorphoses::bitmap::{self, Bitmap, BitmapRange};
use teensy_std::addr::Addr;

use kernel_paging;

use crate::memory::memmap;
use crate::synch::Mutex;

// NOTE: Not a fan of using Once to make this safe
static PAGE_POOL: spin::Once<Mutex<PagePool>> = spin::Once::new();

struct PagePool {
    bmap: Bitmap<'static>,
}

pub fn init(hhdm: &BootHhdm, memmap: &Memmap) {
    init_memory_pool(hhdm, memmap);
}

pub fn get_page() -> Addr {
    PAGE_POOL.wait().lock().get_one()
}

pub fn free_page(addr: Addr) {
    PAGE_POOL.wait().lock().free_one(addr);
}

pub fn get_pages(num_pages: usize) -> Addr {
    PAGE_POOL.wait().lock().get_multiple(num_pages)
}

pub fn free_pages(addr: Addr, num_pages: usize) {
    PAGE_POOL.wait().lock().free_multiple(addr, num_pages);
}

/// Search for the largest contiguous memory region to store the MEMORY_POOL bitmap in
fn init_memory_pool(hhdm: &BootHhdm, memmap: &Memmap) {
    let largest_entry = memmap::get_largest_memmap_entry(hhdm, memmap);
    let memory_pages = memmap::get_num_memory_pages(memmap);

    let bitmap_buf_size = memory_pages / bitmap::WORD_SIZE_BITS
        + if kernel_paging::page_min_no(memory_pages) % bitmap::WORD_SIZE_BITS > 0 {
            1
        } else {
            0
        };

    debug_assert!(bitmap_buf_size * bitmap::WORD_SIZE < largest_entry.len); // check bitmap fits

    let bitmap_buf: &'static mut [u64] = unsafe {
        core::slice::from_raw_parts_mut(
            (largest_entry.base + hhdm.base) as *mut u64,
            bitmap_buf_size,
        )
    };

    let page_pool = PagePool::new(Bitmap::new(bitmap_buf, memory_pages), &memmap);
    PAGE_POOL.call_once(|| Mutex::new(page_pool));
}

impl PagePool {
    fn new(mut bmap: Bitmap<'static>, memmap: &Memmap) -> Self {
        bmap.fill(true);
        let mut page_pool = Self { bmap };
        memmap
            .iter()
            .filter(|entry| entry.typ == BootMemType::Usable)
            .for_each(|usable| page_pool.mark_region(usable, false));
        page_pool
    }

    fn mark_region(&mut self, entry: &MemmapEntry, _is_set: bool) {
        let page_start_no = kernel_paging::page_min_no(entry.base);
        let page_end_no = kernel_paging::page_min_no(entry.end());
        self.bmap
            .flip_range(&BitmapRange::new(page_start_no, page_end_no));
    }

    /// Allocates a single page from the memory pool
    fn get_one(&mut self) -> Addr {
        self.get_multiple(1)
    }

    /// Allocates multiple pages from the memory pool
    fn get_multiple(&mut self, num_pages: usize) -> Addr {
        let ptr = self
            .bmap
            .find_and_flip(num_pages, false)
            .map(|idx| unsafe { NonZeroUsize::new_unchecked(idx * kernel_paging::PAGE_SIZE_MIN) });
        Addr::new(ptr)
    }

    fn free_multiple(&mut self, addr: Addr, num_pages: usize) {
        let start = addr.as_usize() / kernel_paging::PAGE_SIZE_MIN;
        let end = start + num_pages;
        self.bmap.flip_range(&BitmapRange::new(start, end))
    }

    fn free_one(&mut self, addr: Addr) {
        self.free_multiple(addr, 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use kernel_boot_interface::BootInfo;

    #[test_case]
    fn largest_usable_entry(boot_info: &BootInfo) {
        let entry = memmap::get_largest_memmap_entry(&boot_info.hhdm, &boot_info.memmap);
        let hhdm_size = usize::MAX - boot_info.hhdm.base;
        assert!(entry.base < hhdm_size);
        assert!(entry.typ == BootMemType::Usable);
    }

    #[test_case]
    fn check_memory_pages(boot_info: &BootInfo) {
        assert!(memmap::get_num_memory_pages(&boot_info.memmap) > 0);
    }

    fn test_initialise() {
        //init(&boot_info.hhdm, &boot_info.memmap);
    }
}
