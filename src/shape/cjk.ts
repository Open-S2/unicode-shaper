/// Chinese-Japanese-Korean (CJK) characters
export const CJK = [
  // CJK Unified Ideographs [Han] (Range: 4E00–9FFF)
  [0x4e00, 0x9fff],
  // CJK Unified Ideographs Extension A (Range: 3400–4DBF)
  [0x3400, 0x4dbf],
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
  [0xf900, 0xfaff],
  // NOTE: These are u32 values
  // CJK Compatibility Ideographs Supplement (2F800–2FA1F)
  // [0x2F800, 0x2FA1F],
  // Kangxi Radicals (Range: 2F00–2FDF)
  [0x2f00, 0x2fdf],
  // CJK Radicals Supplement (Range: 2E80–2EFF)
  [0x2e80, 0x2eff],
  // CJK Strokes (Range: 31C0–31EF)
  [0x31c0, 0x31ef],
  // Ideographic Description Characters (Range: 2FF0–2FFF)
  [0x2ff0, 0x2fff],
  // CJK Symbols and Punctuation (Range: 3000–303F)
  [0x3000, 0x303f],
  // NOTE: These are u32 values
  // Ideographic Symbols and Punctuation (Range: 16FE0–16FFF)
  // [0x16FE0, 0x16FFF],
  // CJK Compatibility Forms (Range: FE30–FE4F)
  [0xfe30, 0xfe4f],
  // Halfwidth and Fullwidth Forms (Range: FF00–FFEF)
  [0xff00, 0xffef],
  // Small Form Variants (Range: FE50–FE6F)
  [0xfe50, 0xfe6f],
  // Vertical Forms (Range: FE10–FE1F)
  [0xfe10, 0xfe1f],
  // Enclosed CJK Letters and Months (Range: 3200–32FF)
  [0x3200, 0x32ff],
  // CJK Compatibility (Range: 3300–33FF)
  [0x3300, 0x33ff],
];
// 56 bytes

/**
 * Check if a character is CJK (Chinese, Japanese, or Korean)
 * @param c - input unicode character
 * @returns - true if the character is CJK
 */
export function isCJK(c: number): boolean {
  for (const arr of CJK) {
    if (c >= arr[0] && c <= arr[1]) return true;
  }
  return false;
}
