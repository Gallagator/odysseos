pub const MAX_MEM_REGIONS: usize = 256;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum BootMemType {
    Usable,
    Reserved,
    AcpiReclaimable,
    AcpiNvs,
    BootloaderReclaimable,
}

#[derive(Clone, Copy)]
pub struct MemmapEntry {
    pub base: usize,
    pub len: usize,
    pub typ: BootMemType,
}

pub struct Memmap {
    pub entries: [MemmapEntry; MAX_MEM_REGIONS],
    pub entry_count: usize,
}

impl MemmapEntry {
    /// Returns range exclusive
    pub fn end(&self) -> usize {
        self.base + self.len
    }
}

impl Memmap {
    pub fn iter(&self) -> core::slice::Iter<MemmapEntry> {
        self.entries[0..self.entry_count].iter()
    }
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
