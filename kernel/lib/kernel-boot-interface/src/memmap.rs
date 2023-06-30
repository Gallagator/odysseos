pub const MAX_MEM_REGIONS: usize = 256;

pub enum MemType {
    Usable,
    Reserved,
    AcpiReclaimable,
    AcpiNvs,
    BootloaderReclaimable,
}

pub struct Entry {
    pub base: u64,
    pub len: u64,
    pub typ: MemType,
}

pub struct Entries {
    pub regions: [Entry; MAX_MEM_REGIONS],
    pub entry_count: usize,
}

//impl Default for Entry {
//    fn default() -> Self {
//        Self {
//            base: 0,
//            len: 0,
//            typ: MemType::Reserved,
//        }
//    }
//}
//
//impl Default for Entries {
//    fn default() -> Self {
//        Self {
//            regions: [Entry::default(); MAX_MEM_REGIONS],
//            len: 0,
//        }
//    }
//}
