#![no_std]

// TODO: write some tests
const GIB: usize = 1024 * 1024 * 1024;
const MIB: usize = 1024 * 1024;
const KIB: usize = 1024;

/// x86_64 supports page sizes of 1GB, 2MB and 4MB. This array
/// should be in descending order.
pub const PAGE_SIZES: [usize; 3] = [1 * GIB, 2 * MIB, 4 * KIB];

pub const PAGE_SIZE_MIN: usize = PAGE_SIZES[2];
pub const PAG_SIZE_MAX: usize = PAGE_SIZES[0];

pub const fn page_mask(page_size: usize) -> usize {
    page_size - 1
}

pub const fn page_round_down(page: usize, page_size: usize) -> usize {
    page & !page_mask(page_size)
}

pub const fn page_min_round_down(page: usize) -> usize {
    page_round_down(page, PAGE_SIZE_MIN)
}

pub const fn page_round_up(page: usize, page_size: usize) -> usize {
    if page == page_round_down(page, page_size) {
        page
    } else {
        page_round_down(page, page_size) + page_size
    }
}

pub const fn page_min_round_up(page: usize) -> usize {
    page_round_up(page, PAGE_SIZE_MIN)
}

pub const fn page_no(page: usize, page_size: usize) -> usize {
    page / page_size
}

pub const fn page_min_no(page: usize) -> usize {
    page_no(page, PAGE_SIZE_MIN)
}
