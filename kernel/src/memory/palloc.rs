//TODO: NEED TO mark certain memory ranges as unusable

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

    #[test_case]
    fn largest_usable_entry() {
        //let largest_entry = get_largest_memmap_entry(&MOCK_HHDM, &get_mock_memmap());
        //assert_eq!(largest_entry, MOCK_MEMMAP.entries[4])
    }

    const MOCK_HHDM: BootHhdm = BootHhdm {
        base: 18446603336221196288,
    };

    fn get_mock_memmap() -> Memmap {
        let mut entries: [MemmapEntry; 256] = unsafe { core::mem::zeroed() };
        let actual_entries = [
            MemmapEntry {
                base: 4096,
                len: 331776,
                typ: kernel_boot_interface::memmap::MemType::BootloaderReclaimable,
            },
            MemmapEntry {
                base: 335872,
                len: 315392,
                typ: kernel_boot_interface::memmap::MemType::Usable,
            },
            MemmapEntry {
                base: 654336,
                len: 1024,
                typ: kernel_boot_interface::memmap::MemType::Reserved,
            },
            MemmapEntry {
                base: 983040,
                len: 65536,
                typ: kernel_boot_interface::memmap::MemType::Reserved,
            },
            MemmapEntry {
                base: 1048576,
                len: 2141167616,
                typ: kernel_boot_interface::memmap::MemType::Usable,
            },
            MemmapEntry {
                base: 2142216192,
                len: 16384,
                typ: kernel_boot_interface::memmap::MemType::BootloaderReclaimable,
            },
            MemmapEntry {
                base: 2142232576,
                len: 28672,
                typ: kernel_boot_interface::memmap::MemType::Reserved,
            },
            MemmapEntry {
                base: 2142261248,
                len: 1265664,
                typ: kernel_boot_interface::memmap::MemType::BootloaderReclaimable,
            },
            MemmapEntry {
                base: 2143526912,
                len: 4096,
                typ: kernel_boot_interface::memmap::MemType::Usable,
            },
            MemmapEntry {
                base: 2143531008,
                len: 4096,
                typ: kernel_boot_interface::memmap::MemType::BootloaderReclaimable,
            },
            MemmapEntry {
                base: 2143535104,
                len: 3215360,
                typ: kernel_boot_interface::memmap::MemType::Usable,
            },
            MemmapEntry {
                base: 2146750464,
                len: 598016,
                typ: kernel_boot_interface::memmap::MemType::BootloaderReclaimable,
            },
            MemmapEntry {
                base: 2147348480,
                len: 135168,
                typ: kernel_boot_interface::memmap::MemType::Reserved,
            },
            MemmapEntry {
                base: 2952790016,
                len: 268435456,
                typ: kernel_boot_interface::memmap::MemType::Reserved,
            },
            MemmapEntry {
                base: 4244635648,
                len: 3145728,
                typ: kernel_boot_interface::memmap::MemType::Reserved,
            },
            MemmapEntry {
                base: 4275159040,
                len: 16384,
                typ: kernel_boot_interface::memmap::MemType::Reserved,
            },
            MemmapEntry {
                base: 4294705152,
                len: 262144,
                typ: kernel_boot_interface::memmap::MemType::Reserved,
            },
        ];
        entries.copy_from_slice(&actual_entries);
        Memmap {
            entries,
            entry_count: 17,
        }
    }
}
