use kernel_boot_interface::{
    hhdm::BootHhdm,
    memmap::{BootMemType, Memmap, MemmapEntry},
};
use teensy_std::addr::Addr;

pub fn get_num_memory_pages(memmap: &Memmap) -> usize {
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

pub fn get_largest_memmap_entry<'a>(hhdm: &BootHhdm, memmap: &'a Memmap) -> &'a MemmapEntry {
    let entry_size_in_hdmm = |entry: &MemmapEntry| {
        let hhdm_size = usize::MAX - hhdm.base;
        usize::min(hhdm_size, entry.base + entry.len) - entry.base
    };
    let largest_entry = memmap
        .iter()
        .reduce(|cur_max, entry| {
            let cur_max_size = entry_size_in_hdmm(cur_max);
            let entry_size = entry_size_in_hdmm(entry);
            if BootMemType::Usable != cur_max.typ || cur_max_size < entry_size {
                entry
            } else {
                cur_max
            }
        })
        .unwrap();

    debug_assert!(largest_entry.typ == BootMemType::Usable);
    largest_entry
}

pub fn get_addr_entry<'a>(memmap: &'a Memmap, addr: &Addr) -> &'a MemmapEntry {
    let offset = addr.as_usize();
    memmap
        .iter()
        .find(|entry| offset >= entry.base && offset < entry.end())
        .expect("This address should be in the memory map")
}
