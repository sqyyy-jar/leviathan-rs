/// TODO: return result
/// error if not in range
pub trait MaxBitsU32 {
    fn cut(&self, bits: usize) -> u32;
}

impl MaxBitsU32 for u32 {
    fn cut(&self, bits: usize) -> Self {
        self & ((1 << bits) - 1)
    }
}

impl MaxBitsU32 for i32 {
    fn cut(&self, bits: usize) -> u32 {
        (self & ((1 << bits) - 1)) as u32
    }
}

impl MaxBitsU32 for isize {
    fn cut(&self, bits: usize) -> u32 {
        (self & ((1 << bits) - 1)) as u32
    }
}

pub const fn alignment(size: usize, align: usize) -> usize {
    if size % align == 0 {
        0
    } else {
        align - (size % align)
    }
}
