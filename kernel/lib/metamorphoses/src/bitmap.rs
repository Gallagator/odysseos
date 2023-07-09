pub const WORD_SIZE: usize = core::mem::size_of::<u64>();
pub const WORD_SIZE_BITS: usize = WORD_SIZE * 8;

pub struct Bitmap<'a> {
    bits: &'a mut [u64],
    len: usize,
}

/// Represents the bitmap with a range that can act on it
/// start is inclusive, end is exclusive.
pub struct BitmapRange {
    start: usize,
    end: usize,
}

impl<'a> Bitmap<'a> {
    /// LEN is passed in bits
    pub fn new(bits: &'a mut [u64], len: usize) -> Self {
        debug_assert!(len <= bits.len() * WORD_SIZE_BITS);
        bits.fill(0);
        Self { bits, len }
    }

    pub fn fill(&mut self, is_set: bool) {
        let value = if is_set { 1 } else { 0 };
        self.bits.fill(value);
    }
    /// Gets bit at idx
    pub fn get(&self, idx: usize) -> bool {
        (self.bits[idx / WORD_SIZE_BITS] & (1u64 << (idx % WORD_SIZE_BITS))) > 0
    }

    /// Sets bit at idx
    pub fn set(&mut self, idx: usize) {
        self.bits[idx / WORD_SIZE_BITS] |= 1u64 << (idx % WORD_SIZE_BITS)
    }

    /// Clears bit at idx
    pub fn clear(&mut self, idx: usize) {
        self.bits[idx / WORD_SIZE_BITS] &= !(1u64 << (idx % WORD_SIZE_BITS))
    }

    pub fn len(&self) -> usize {
        return self.len;
    }

    /// Returns a BitmapRange for the first range that can be flipped
    pub fn find_first_fit(&self, size: usize, is_set: bool) -> Option<BitmapRange> {
        debug_assert!(size != 0);
        let mut start = None;
        let mut end = 0;
        for i in 0..(self.len()) {
            if self.get(i) == is_set {
                let beginning = if let Some(val) = start {
                    end += 1;
                    val
                } else {
                    start = Some(i);
                    end = i + 1;
                    i
                };
                if end - beginning >= size {
                    return Some(BitmapRange {
                        start: beginning,
                        end,
                    });
                }
            } else {
                start = None
            }
        }
        None
    }

    pub fn flip_range(&mut self, range: &BitmapRange) {
        debug_assert!(range.start < self.len());
        debug_assert!(range.end <= self.len());
        debug_assert!(self.all_same(range));

        let start_idx = range.start / WORD_SIZE_BITS;
        let end_idx = range.end / WORD_SIZE_BITS;

        let leading_bits = !((1 << (range.start % WORD_SIZE_BITS)) - 1);
        let trailing_bits = (1 << (range.end % WORD_SIZE_BITS)) - 1;

        if start_idx == end_idx {
            self.bits[start_idx] ^= leading_bits & trailing_bits;
        } else {
            self.bits[start_idx] ^= leading_bits;
            for i in (start_idx + 1)..end_idx {
                self.bits[i] ^= !0u64;
            }
            if end_idx < self.bits.len() {
                self.bits[end_idx] ^= trailing_bits;
            }
        }
    }

    pub fn find_and_flip(&mut self, size: usize, is_set: bool) -> Option<usize> {
        if let Some(range) = self.find_first_fit(size, is_set) {
            self.flip_range(&range);
            Some(range.start)
        } else {
            None
        }
    }

    fn all_same(&self, range: &BitmapRange) -> bool {
        let mut bits = (range.start..range.end).map(|idx| self.get(idx));
        if let Some(first) = bits.next() {
            bits.all(|bit| bit == first)
        } else {
            true
        }
    }
}

impl BitmapRange {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn alloc_one() {
        let mut buf = [0u64; 10];
        let mut bmap = Bitmap::new(&mut buf, 10 * WORD_SIZE_BITS);
        assert_eq!(bmap.find_and_flip(1, false), Some(0));
        assert_eq!(bmap.get(0), true);
    }

    #[test_case]
    fn alloc_two() {
        let mut buf = [0u64; 10];
        let mut bmap = Bitmap::new(&mut buf, 10 * WORD_SIZE_BITS);
        assert_eq!(bmap.find_and_flip(1, false), Some(0));
        assert_eq!(bmap.find_and_flip(1, false), Some(1));
        assert_eq!(bmap.get(0), true);
        assert_eq!(bmap.get(1), true);
    }

    #[test_case]
    fn alloc_all() {
        let mut buf = [0u64; 10];
        let mut bmap = Bitmap::new(&mut buf, 10 * WORD_SIZE_BITS);
        for i in 0..(bmap.len()) {
            assert_eq!(bmap.find_and_flip(1, false), Some(i));
        }
        for i in 0..(bmap.len()) {
            assert_eq!(bmap.get(i), true);
        }
    }

    #[test_case]
    fn alloc_big() {
        let mut buf = [0u64; 10];
        let mut bmap = Bitmap::new(&mut buf, 10 * WORD_SIZE_BITS);
        assert_eq!(bmap.find_and_flip(64 * 10, false), Some(0));
        assert_eq!(bmap.len(), 64 * 10);
        for i in 0..(bmap.len()) {
            assert_eq!(bmap.get(i), true);
        }
        bmap.find_and_flip(64, true);
        for i in 0..64 {
            assert_eq!(bmap.get(i), false);
        }
        for i in 64..(bmap.len()) {
            assert_eq!(bmap.get(i), true);
        }
        bmap.find_and_flip(12, false);
        for i in 0..12 {
            assert_eq!(bmap.get(i), true);
        }

        assert_eq!(bmap.find_and_flip(53, false), None);
    }
}
