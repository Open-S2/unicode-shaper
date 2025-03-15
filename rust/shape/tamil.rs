use super::{shared::shared_shaper, TAMIL_VOWELS};

/// Check if a unicode character is Tamil
pub fn is_tamil(c: &u16) -> bool {
    *c >= 0x0b80 && *c <= 0x0bff
}

/// Shape Tamil unicode
pub fn shape_tamil(input: &mut [u16]) {
    shared_shaper(input, &TAMIL_VOWELS);
}
