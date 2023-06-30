const WORD_SIZE: usize = core::mem::size_of::<u64>();

pub struct Bitmap<'a> {
    bits: &'a mut [u64],
}

/// Represents the bitmap with a range that can act on it
/// start is inclusive, end is exclusive.
pub struct BitmapRange<'a> {
    bitmap: &'a mut Bitmap<'a>,
    start: usize,
    end: usize,
}

impl<'a> Bitmap<'a> {
    pub fn new(bits: &'a mut [u64]) -> Self {
        bits.fill(0);
        Self { bits }
    }

    /// Gets bit at idx
    pub fn get(&self, idx: usize) -> bool {
        (self.bits[idx / WORD_SIZE] & (1u64 << (idx % WORD_SIZE))) > 0
    }

    /// Sets bit at idx
    pub fn set(&mut self, idx: usize) {
        self.bits[idx / WORD_SIZE] |= 1u64 << (idx % WORD_SIZE)
    }

    /// Clears bit at idx
    pub fn clear(&mut self, idx: usize) {
        self.bits[idx / WORD_SIZE] &= !(1u64 << (idx % WORD_SIZE))
    }

    /// Returns a BitmapRange for the first range that can be flipped
    pub fn find_first_fit(&'a mut self, size: usize, set: bool) -> Option<BitmapRange<'a>> {
        debug_assert!(size != 0);
        let mut start = None;
        let mut end = 0;
        for i in 0..(self.len()) {
            if self.get(i) == set {
                let beginning = if let Some(val) = start {
                    end += 1;
                    val
                } else {
                    start = Some(i);
                    end = i + 1;
                    i
                };
                if end - beginning > size {
                    return Some(BitmapRange {
                        bitmap: self,
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

    pub fn range(&'a mut self, start: usize, end: usize) -> BitmapRange<'a> {
        debug_assert!(start < self.len());
        debug_assert!(end < self.len());

        BitmapRange {
            bitmap: self,
            start,
            end,
        }
    }

    pub fn len(&self) -> usize {
        return self.bits.len() * WORD_SIZE;
    }
}

impl<'a> BitmapRange<'a> {
    pub fn flip_range(&mut self) {
        let start_idx = self.start / WORD_SIZE;
        let end_idx = self.end / WORD_SIZE;

        let leading_bits = (1 << (WORD_SIZE - (self.start % WORD_SIZE))) - 1;
        let trailing_bits = (1 << (self.end % WORD_SIZE)) - 1;

        if start_idx == end_idx {
            self.bitmap.bits[start_idx] ^= leading_bits & trailing_bits;
        } else {
            self.bitmap.bits[start_idx] ^= leading_bits;
            for i in (start_idx + 1)..end_idx {
                self.bitmap.bits[i] ^= !0u64;
            }
            self.bitmap.bits[end_idx] ^= trailing_bits;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alloc_one() {
        let mut buf = [0u64; 10];
        let mut bmap = Bitmap::new(&mut buf);
        bmap.find_first_fit(1).unwrap().flip_range();
        assert_eq!(bmap.get(0), true);
    }
}
