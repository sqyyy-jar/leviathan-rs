use std::collections::HashMap;

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

pub fn get_key_by_value<'a, K, V: PartialEq>(map: &'a HashMap<K, V>, value: &V) -> Option<&'a K> {
    let Some((k, _)) = map.iter().find(|(_, v)| *v == value) else {
        return None;
    };
    Some(k)
}
