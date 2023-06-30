pub struct BootFrameBuf {
    pub phys_address: usize,
    pub width: u64,
    pub height: u64,
    pub pitch: u64,
    pub bpp: u16,
}
