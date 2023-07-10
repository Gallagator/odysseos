use core::num::NonZeroUsize;
use spin;

use kernel_boot_interface::{
    hhdm::BootHhdm,
    memmap::{MemType, Memmap, MemmapEntry},
};
use metamorphoses::bitmap::{self, Bitmap, BitmapRange};

use kernel_paging;

use crate::synch::Mutex;

// NOTE: Not a fan of using Once to make this safe
static PAGE_POOL: spin::Once<Mutex<PagePool>> = spin::Once::new();

struct PagePool {
    bmap: Bitmap<'static>,
}

pub fn init(hhdm: &BootHhdm, memmap: &Memmap) {
    init_memory_pool(hhdm, memmap);
}

pub fn get_page() -> Option<NonZeroUsize> {
    PAGE_POOL.wait().lock().get_one()
}

/// Search for the largest contiguous memory region to store the MEMORY_POOL bitmap in
fn init_memory_pool(hhdm: &BootHhdm, memmap: &Memmap) {
    let largest_entry = get_largest_memmap_entry(hhdm, memmap);
    let memory_pages = get_num_memory_pages(memmap);

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

fn get_num_memory_pages(memmap: &Memmap) -> usize {
    kernel_paging::page_min_no(kernel_paging::page_min_round_down(
        memmap
            .iter()
            .reduce(|cur_max, entry| {
                // TODO: Check if we need to add unusable regions to palloc pool
                if entry.end() > cur_max.end() {
                    entry
                } else {
                    cur_max
                }
            })
            .unwrap()
            .end(),
    ))
}

fn get_largest_memmap_entry<'a>(hhdm: &BootHhdm, memmap: &'a Memmap) -> &'a MemmapEntry {
    let entry_size_in_hdmm = |entry: &MemmapEntry| {
        let hhdm_size = usize::MAX - hhdm.base;
        usize::min(hhdm_size, entry.base + entry.len) - entry.base
    };
    let largest_entry = memmap
        .iter()
        .reduce(|cur_max, entry| {
            let cur_max_size = entry_size_in_hdmm(cur_max);
            let entry_size = entry_size_in_hdmm(entry);
            if MemType::Usable != cur_max.typ || cur_max_size < entry_size {
                entry
            } else {
                cur_max
            }
        })
        .unwrap();

    debug_assert!(largest_entry.typ == MemType::Usable);
    largest_entry
}

impl PagePool {
    fn new(mut bmap: Bitmap<'static>, memmap: &Memmap) -> Self {
        bmap.fill(true);
        let mut page_pool = Self { bmap };
        memmap
            .iter()
            .filter(|entry| entry.typ == MemType::Usable)
            .for_each(|usable| page_pool.mark_region(usable, false));
        page_pool
    }

    fn mark_region(&mut self, entry: &MemmapEntry, is_set: bool) {
        let page_end_no = kernel_paging::page_min_no(entry.base);
        let page_start_no = kernel_paging::page_min_no(entry.end());
        self.bmap
            .flip_range(&BitmapRange::new(page_start_no, page_end_no));
    }

    /// Allocates a single page from the memory pool
    fn get_one(&mut self) -> Option<NonZeroUsize> {
        self.get_multiple(1)
    }

    /// Allocates multiple pages from the memory pool
    fn get_multiple(&mut self, num_pages: usize) -> Option<NonZeroUsize> {
        self.bmap
            .find_and_flip(num_pages, false)
            .map(|idx| NonZeroUsize::new(idx * kernel_paging::PAGE_SIZE_MIN).unwrap())
        //.map(|idx| NonNull::new((idx * kernel_paging::PAGE_SIZE_MIN) as *mut u8).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use kernel_boot_interface::BootInfo;

    #[test_case]
    fn largest_usable_entry(boot_info: &BootInfo) {
        let entry = get_largest_memmap_entry(&boot_info.hhdm, &boot_info.memmap);
        let hhdm_size = usize::MAX - boot_info.hhdm.base;
        assert!(entry.base < hhdm_size);
        assert!(entry.typ == MemType::Usable);
    }

    #[test_case]
    fn largest_usable_entry(boot_info: &BootInfo) {
        assert!(get_num_memory_pages(&boot_info.memmap) > 0);
    }

    fn test_initialise() {
        //init(&boot_info.hhdm, &boot_info.memmap);
    }
}
