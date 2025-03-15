/// Chinese-Japanese-Korean (CJK) characters
pub static CJK: [[u16; 2]; 14] = [
    // CJK Unified Ideographs [Han] (Range: 4E00–9FFF)
    [0x4E00, 0x9FFF],
    // CJK Unified Ideographs Extension A (Range: 3400–4DBF)
    [0x3400, 0x4DBF],
    // NOTE: These are u32 values
    // CJK Unified Ideographs Extension B (Range: 20000–2A6DF)
    // [0x20000, 0x2A6DF],
    // CJK Unified Ideographs Extension C (Range: 2A700–2B739)
    // [0x2A700, 0x2B739],
    // CJK Unified Ideographs Extension D (Range: 2B740–2B81D)
    // [0x2B740, 0x2B81D],
    // CJK Unified Ideographs Extension E (Range: 2B820–2CEA1)
    // [0x2B820, 0x2CEA1],
    // CJK Unified Ideographs Extension F (Range: 2CEB0–2EBE0)
    // [0x2CEB0, 0x2EBE0],
    // CJK Unified Ideographs Extension G (Range: 30000–3134A)
    // [0x30000, 0x3134A],
    // CJK Unified Ideographs Extension H (Range: 31350–323AF)
    // [0x31350, 0x323AF],
    // CJK Compatibility Ideographs (Range: F900–FAFF)
    [0xF900, 0xFAFF],
    // NOTE: These are u32 values
    // CJK Compatibility Ideographs Supplement (2F800–2FA1F)
    // [0x2F800, 0x2FA1F],
    // Kangxi Radicals (Range: 2F00–2FDF)
    [0x2F00, 0x2FDF],
    // CJK Radicals Supplement (Range: 2E80–2EFF)
    [0x2E80, 0x2EFF],
    // CJK Strokes (Range: 31C0–31EF)
    [0x31C0, 0x31EF],
    // Ideographic Description Characters (Range: 2FF0–2FFF)
    [0x2FF0, 0x2FFF],
    // CJK Symbols and Punctuation (Range: 3000–303F)
    [0x3000, 0x303F],
    // NOTE: These are u32 values
    // Ideographic Symbols and Punctuation (Range: 16FE0–16FFF)
    // [0x16FE0, 0x16FFF],
    // CJK Compatibility Forms (Range: FE30–FE4F)
    [0xFE30, 0xFE4F],
    // Halfwidth and Fullwidth Forms (Range: FF00–FFEF)
    [0xFF00, 0xFFEF],
    // Small Form Variants (Range: FE50–FE6F)
    [0xFE50, 0xFE6F],
    // Vertical Forms (Range: FE10–FE1F)
    [0xFE10, 0xFE1F],
    // Enclosed CJK Letters and Months (Range: 3200–32FF)
    [0x3200, 0x32FF],
    // CJK Compatibility (Range: 3300–33FF)
    [0x3300, 0x33FF],
];
// 56 bytes

/// Check if a character is CJK (Chinese, Japanese, or Korean)
pub fn is_cjk(c: &u16) -> bool {
    for arr in CJK {
        if *c >= arr[0] && *c <= arr[1] {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_is_cjk() {
        assert!(is_cjk(&0x4E00));
        assert!(!is_cjk(&0x01));
    }
}
