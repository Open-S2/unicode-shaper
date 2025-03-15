/// Check if a unicode character is Thai
pub fn is_thai(c: &u16) -> bool {
    *c >= 0xfe70 && *c <= 0xfeff
}
