pub struct BootHhdm {
    pub base: usize,
}

impl BootHhdm {
    pub const fn max_len(&self) -> usize {
        usize::MAX - self.base
    }
}
