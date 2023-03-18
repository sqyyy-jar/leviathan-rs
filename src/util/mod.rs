pub mod ariadne;
pub mod source;

#[inline(always)]
pub const fn align(size: usize, alignment: usize) -> usize {
    if size % alignment == 0 {
        size
    } else {
        size - size % alignment + alignment
    }
}

#[inline(always)]
pub const fn alignment(size: usize, alignment: usize) -> usize {
    align(size, alignment) - size
}
