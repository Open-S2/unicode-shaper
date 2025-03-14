/// If a RTL set is reversed, but surounded by (), [], {}, or <>, then mirror the sets.
/// This also tracks other special characters that need to be mirrored. Some examples:
/// '«', '»', '∕', '∟', '∠', '∡', '∢', '∤', '≃', '≅', '≌', '⊘', '⊦', '⊨',
export const MIRROR_CHAR = [
  [40, 41],
  [41, 40],
  [60, 62],
  [62, 60],
  [91, 93],
  [93, 91],
  [123, 125],
  [125, 123],
  [171, 187],
  [187, 171],
  [8725, 10741],
  [8735, 11262],
  [8736, 10659],
  [8737, 10651],
  [8738, 10656],
  [8740, 10990],
  [8771, 8909],
  [8773, 8780],
  [8780, 8773],
  [8856, 10680],
  [8870, 10974],
  [8872, 10980],
  [8873, 10979],
  [8875, 10981],
  [8888, 10204],
  [8909, 8771],
  [8946, 8954],
  [8947, 8955],
  [8948, 8956],
  [8950, 8957],
  [8951, 8958],
  [8954, 8946],
  [8955, 8947],
  [8956, 8948],
  [8957, 8950],
  [8958, 8951],
  [10204, 8888],
  [10651, 8737],
  [10656, 8738],
  [10659, 8736],
  [10680, 8856],
  [10741, 8725],
  [10974, 8870],
  [10979, 8873],
  [10980, 8872],
  [10981, 8875],
  [10990, 8740],
  [11262, 8735],
];
// 192 bytes

/**
 * Mirror adjust string (things like parens, brackets, quotes, etc.)
 * @param s - input string
 */
export function mirrorAdjustString(s: number[]): void {
  for (let i = 0, sl = s.length; i < sl; i++) {
    const c = s[i];
    for (const from_to of MIRROR_CHAR) {
      if (c === from_to[0]) {
        s[i] = from_to[1];
        break;
      }
    }
  }
}

/// A RandALCat character is a character with unambiguously right-to-left directionality.
export const RAND_AL_CAT = [
  [0x05be, 0x05be],
  [0x05c0, 0x05c0],
  [0x05c3, 0x05c3],
  [0x05c6, 0x05c6],
  [0x05d0, 0x05ea],
  [0x05f0, 0x05f4],
  [0x0608, 0x0608],
  [0x060b, 0x060b],
  [0x060d, 0x060d],
  [0x061b, 0x061b],
  [0x061e, 0x064a],
  [0x066d, 0x066f],
  [0x0671, 0x06d5],
  [0x06e5, 0x06e6],
  [0x06ee, 0x06ef],
  [0x06fa, 0x070d],
  [0x0710, 0x0710],
  [0x0712, 0x072f],
  [0x074d, 0x07a5],
  [0x07b1, 0x07b1],
  [0x07c0, 0x07ea],
  [0x07f4, 0x07f5],
  [0x07fa, 0x07fa],
  [0x0800, 0x0815],
  [0x081a, 0x081a],
  [0x0824, 0x0824],
  [0x0828, 0x0828],
  [0x0830, 0x083e],
  [0x0840, 0x0858],
  [0x085e, 0x085e],
  [0x200f, 0x200f],
  [0xfb1d, 0xfb1d],
  [0xfb1f, 0xfb28],
  [0xfb2a, 0xfb36],
  [0xfb38, 0xfb3c],
  [0xfb3e, 0xfb3e],
  [0xfb40, 0xfb41],
  [0xfb43, 0xfb44],
  [0xfb46, 0xfbc1],
  [0xfbd3, 0xfd3d],
  [0xfd50, 0xfd8f],
  [0xfd92, 0xfdc7],
  [0xfdf0, 0xfdfc],
  [0xfe70, 0xfe74],
  [0xfe76, 0xfefc],
];
// 168 bytes

/**
 * Check if a character is a "right-to-left" unicode character
 * @param c - input unicode character
 * @returns - True if right-to-left
 */
export function isRTL(c: number): boolean {
  for (const arr of RAND_AL_CAT) {
    if (c >= arr[0] && c <= arr[1]) return true;
  }
  return false;
}

/// NEUTRAL characters are those with no inherent directionality, which can be
/// treated as being part of any adjacent runs of text with other directionality.
export const NEUTRAL = [
  [0x0009, 0x000d],
  [0x001c, 0x0022],
  [0x0026, 0x002a],
  [0x003b, 0x0040],
  [0x005b, 0x0060],
  [0x007b, 0x007e],
  [0x0085, 0x0085],
  [0x00a1, 0x00a1],
  [0x00a6, 0x00a9],
  [0x00ab, 0x00ac],
  [0x00ae, 0x00af],
  [0x00b4, 0x00b4],
  [0x00b6, 0x00b8],
  [0x00bb, 0x00bf],
  [0x00d7, 0x00d7],
  [0x00f7, 0x00f7],
  [0x02b9, 0x02ba],
  [0x02c2, 0x02cf],
  [0x02d2, 0x02df],
  [0x02e5, 0x02ed],
  [0x02ef, 0x02ff],
  [0x0374, 0x0375],
  [0x037e, 0x037e],
  [0x0384, 0x0385],
  [0x0387, 0x0387],
  [0x03f6, 0x03f6],
  [0x058a, 0x058a],
  [0x0606, 0x0607],
  [0x060e, 0x060f],
  [0x06de, 0x06de],
  [0x06e9, 0x06e9],
  [0x07f6, 0x07f9],
  [0x0bf3, 0x0bf8],
  [0x0bfa, 0x0bfa],
  [0x0c78, 0x0c7e],
  [0x0f3a, 0x0f3d],
  [0x1390, 0x1399],
  [0x1400, 0x1400],
  [0x1680, 0x1680],
  [0x169b, 0x169c],
  [0x17f0, 0x17f9],
  [0x1800, 0x180a],
  [0x180e, 0x180e],
  [0x1940, 0x1940],
  [0x1944, 0x1945],
  [0x19de, 0x19ff],
  [0x1fbd, 0x1fbd],
  [0x1fbf, 0x1fc1],
  [0x1fcd, 0x1fcf],
  [0x1fdd, 0x1fdf],
  [0x1fed, 0x1fef],
  [0x1ffd, 0x1ffe],
  [0x2000, 0x200a],
  [0x2010, 0x202e],
  [0x2035, 0x2043],
  [0x2045, 0x205f],
  [0x207c, 0x207e],
  [0x208c, 0x208e],
  [0x2100, 0x2101],
  [0x2103, 0x2106],
  [0x2108, 0x2109],
  [0x2114, 0x2114],
  [0x2116, 0x2118],
  [0x211e, 0x2123],
  [0x2125, 0x2125],
  [0x2127, 0x2127],
  [0x2129, 0x2129],
  [0x213a, 0x213b],
  [0x2140, 0x2144],
  [0x214a, 0x214d],
  [0x2150, 0x215f],
  [0x2189, 0x2189],
  [0x2190, 0x2211],
  [0x2214, 0x2335],
  [0x237b, 0x2394],
  [0x2396, 0x23f3],
  [0x2400, 0x2426],
  [0x2440, 0x244a],
  [0x2460, 0x2487],
  [0x24ea, 0x26ab],
  [0x26ad, 0x26ff],
  [0x2701, 0x27ca],
  [0x27cc, 0x27cc],
  [0x27ce, 0x27ff],
  [0x2900, 0x2b4c],
  [0x2b50, 0x2b59],
  [0x2ce5, 0x2cea],
  [0x2cf9, 0x2cff],
  [0x2e00, 0x2e31],
  [0x2e80, 0x2e99],
  [0x2e9b, 0x2ef3],
  [0x2f00, 0x2fd5],
  [0x2ff0, 0x2ffb],
  [0x3000, 0x3004],
  [0x3008, 0x3020],
  [0x3030, 0x3030],
  [0x3036, 0x3037],
  [0x303d, 0x303f],
  [0x309b, 0x309c],
  [0x30a0, 0x30a0],
  [0x30fb, 0x30fb],
  [0x31c0, 0x31e3],
  [0x321d, 0x321e],
  [0x3250, 0x325f],
  [0x327c, 0x327e],
  [0x32b1, 0x32bf],
  [0x32cc, 0x32cf],
  [0x3377, 0x337a],
  [0x33de, 0x33df],
  [0x33ff, 0x33ff],
  [0x4dc0, 0x4dff],
  [0xa490, 0xa4c6],
  [0xa60d, 0xa60f],
  [0xa673, 0xa673],
  [0xa67e, 0xa67f],
  [0xa700, 0xa721],
  [0xa788, 0xa788],
  [0xa828, 0xa82b],
  [0xa874, 0xa877],
  [0xfd3e, 0xfd3f],
  [0xfdfd, 0xfdfd],
  [0xfe10, 0xfe19],
  [0xfe30, 0xfe4f],
  [0xfe51, 0xfe51],
  [0xfe54, 0xfe54],
  [0xfe56, 0xfe5e],
  [0xfe60, 0xfe61],
  [0xfe64, 0xfe66],
  [0xfe68, 0xfe68],
  [0xfe6b, 0xfe6b],
  [0xff01, 0xff02],
  [0xff06, 0xff0a],
  [0xff1b, 0xff20],
  [0xff3b, 0xff40],
  [0xff5b, 0xff65],
  [0xffe2, 0xffe4],
  [0xffe8, 0xffee],
];
// 536 bytes

/**
 * Check if character is neutral
 * @param c - input unicode character
 * @returns - True if neutral
 */
export function isNeutral(c: number): boolean {
  for (const arr of NEUTRAL) {
    if (c >= arr[0] && c <= arr[1]) return true;
  }
  return false;
}

export const WEAK = [
  [0x0000, 0x0008],
  [0x000e, 0x001b],
  [0x0023, 0x0025],
  [0x002b, 0x003a],
  [0x007f, 0x0084],
  [0x0086, 0x00a0],
  [0x00a2, 0x00a5],
  [0x00ad, 0x00ad],
  [0x00b0, 0x00b3],
  [0x00b9, 0x00b9],
  [0x0300, 0x036f],
  [0x0483, 0x0489],
  [0x0591, 0x05bd],
  [0x05bf, 0x05bf],
  [0x05c1, 0x05c2],
  [0x05c4, 0x05c5],
  [0x05c7, 0x05c7],
  [0x0600, 0x0603],
  [0x0609, 0x060a],
  [0x060c, 0x060c],
  [0x0610, 0x061a],
  [0x064b, 0x066c],
  [0x0670, 0x0670],
  [0x06d6, 0x06dd],
  [0x06df, 0x06e4],
  [0x06e7, 0x06e8],
  [0x06ea, 0x06ed],
  [0x06f0, 0x06f9],
  [0x070f, 0x070f],
  [0x0711, 0x0711],
  [0x0730, 0x074a],
  [0x07a6, 0x07b0],
  [0x07eb, 0x07f3],
  [0x0816, 0x0819],
  [0x081b, 0x0823],
  [0x0825, 0x0827],
  [0x0829, 0x082d],
  [0x0859, 0x085b],
  [0x0900, 0x0902],
  [0x093a, 0x093a],
  [0x093c, 0x093c],
  [0x0941, 0x0948],
  [0x094d, 0x094d],
  [0x0951, 0x0957],
  [0x0962, 0x0963],
  [0x0981, 0x0981],
  [0x09bc, 0x09bc],
  [0x09c1, 0x09c4],
  [0x09cd, 0x09cd],
  [0x09e2, 0x09e3],
  [0x09f2, 0x09f3],
  [0x09fb, 0x09fb],
  [0x0a01, 0x0a02],
  [0x0a3c, 0x0a3c],
  [0x0a41, 0x0a42],
  [0x0a47, 0x0a48],
  [0x0a4b, 0x0a4d],
  [0x0a51, 0x0a51],
  [0x0a70, 0x0a71],
  [0x0a75, 0x0a75],
  [0x0a81, 0x0a82],
  [0x0abc, 0x0abc],
  [0x0ac1, 0x0ac5],
  [0x0ac7, 0x0ac8],
  [0x0acd, 0x0acd],
  [0x0ae2, 0x0ae3],
  [0x0af1, 0x0af1],
  [0x0b01, 0x0b01],
  [0x0b3c, 0x0b3c],
  [0x0b3f, 0x0b3f],
  [0x0b41, 0x0b44],
  [0x0b4d, 0x0b4d],
  [0x0b56, 0x0b56],
  [0x0b62, 0x0b63],
  [0x0b82, 0x0b82],
  [0x0bc0, 0x0bc0],
  [0x0bcd, 0x0bcd],
  [0x0bf9, 0x0bf9],
  [0x0c3e, 0x0c40],
  [0x0c46, 0x0c48],
  [0x0c4a, 0x0c4d],
  [0x0c55, 0x0c56],
  [0x0c62, 0x0c63],
  [0x0cbc, 0x0cbc],
  [0x0ccc, 0x0ccd],
  [0x0ce2, 0x0ce3],
  [0x0d41, 0x0d44],
  [0x0d4d, 0x0d4d],
  [0x0d62, 0x0d63],
  [0x0dca, 0x0dca],
  [0x0dd2, 0x0dd4],
  [0x0dd6, 0x0dd6],
  [0x0e31, 0x0e31],
  [0x0e34, 0x0e3a],
  [0x0e3f, 0x0e3f],
  [0x0e47, 0x0e4e],
  [0x0eb1, 0x0eb1],
  [0x0eb4, 0x0eb9],
  [0x0ebb, 0x0ebc],
  [0x0ec8, 0x0ecd],
  [0x0f18, 0x0f19],
  [0x0f35, 0x0f35],
  [0x0f37, 0x0f37],
  [0x0f39, 0x0f39],
  [0x0f71, 0x0f7e],
  [0x0f80, 0x0f84],
  [0x0f86, 0x0f87],
  [0x0f8d, 0x0f97],
  [0x0f99, 0x0fbc],
  [0x0fc6, 0x0fc6],
  [0x102d, 0x1030],
  [0x1032, 0x1037],
  [0x1039, 0x103a],
  [0x103d, 0x103e],
  [0x1058, 0x1059],
  [0x105e, 0x1060],
  [0x1071, 0x1074],
  [0x1082, 0x1082],
  [0x1085, 0x1086],
  [0x108d, 0x108d],
  [0x109d, 0x109d],
  [0x135d, 0x135f],
  [0x1712, 0x1714],
  [0x1732, 0x1734],
  [0x1752, 0x1753],
  [0x1772, 0x1773],
  [0x17b7, 0x17bd],
  [0x17c6, 0x17c6],
  [0x17c9, 0x17d3],
  [0x17db, 0x17db],
  [0x17dd, 0x17dd],
  [0x180b, 0x180d],
  [0x18a9, 0x18a9],
  [0x1920, 0x1922],
  [0x1927, 0x1928],
  [0x1932, 0x1932],
  [0x1939, 0x193b],
  [0x1a17, 0x1a18],
  [0x1a56, 0x1a56],
  [0x1a58, 0x1a5e],
  [0x1a60, 0x1a60],
  [0x1a62, 0x1a62],
  [0x1a65, 0x1a6c],
  [0x1a73, 0x1a7c],
  [0x1a7f, 0x1a7f],
  [0x1b00, 0x1b03],
  [0x1b34, 0x1b34],
  [0x1b36, 0x1b3a],
  [0x1b3c, 0x1b3c],
  [0x1b42, 0x1b42],
  [0x1b6b, 0x1b73],
  [0x1b80, 0x1b81],
  [0x1ba2, 0x1ba5],
  [0x1ba8, 0x1ba9],
  [0x1be6, 0x1be6],
  [0x1be8, 0x1be9],
  [0x1bed, 0x1bed],
  [0x1bef, 0x1bf1],
  [0x1c2c, 0x1c33],
  [0x1c36, 0x1c37],
  [0x1cd0, 0x1cd2],
  [0x1cd4, 0x1ce0],
  [0x1ce2, 0x1ce8],
  [0x1ced, 0x1ced],
  [0x1dc0, 0x1de6],
  [0x1dfc, 0x1dff],
  [0x200b, 0x200d],
  [0x202f, 0x2034],
  [0x2044, 0x2044],
  [0x2060, 0x2064],
  [0x206a, 0x2070],
  [0x2074, 0x207b],
  [0x2080, 0x208b],
  [0x20a0, 0x20b9],
  [0x20d0, 0x20f0],
  [0x212e, 0x212e],
  [0x2212, 0x2213],
  [0x2488, 0x249b],
  [0x2cef, 0x2cf1],
  [0x2d7f, 0x2d7f],
  [0x2de0, 0x2dff],
  [0x302a, 0x302f],
  [0x3099, 0x309a],
  [0xa66f, 0xa672],
  [0xa67c, 0xa67d],
  [0xa6f0, 0xa6f1],
  [0xa802, 0xa802],
  [0xa806, 0xa806],
  [0xa80b, 0xa80b],
  [0xa825, 0xa826],
  [0xa838, 0xa839],
  [0xa8c4, 0xa8c4],
  [0xa8e0, 0xa8f1],
  [0xa926, 0xa92d],
  [0xa947, 0xa951],
  [0xa980, 0xa982],
  [0xa9b3, 0xa9b3],
  [0xa9b6, 0xa9b9],
  [0xa9bc, 0xa9bc],
  [0xa9bc, 0xa9bc],
  [0xaa29, 0xaa2e],
  [0xaa31, 0xaa32],
  [0xaa35, 0xaa36],
  [0xaa43, 0xaa43],
  [0xaa4c, 0xaa4c],
  [0xaab0, 0xaab0],
  [0xaab2, 0xaab4],
  [0xaab7, 0xaab8],
  [0xaabe, 0xaabf],
  [0xaac1, 0xaac1],
  [0xabe5, 0xabe5],
  [0xabe8, 0xabe8],
  [0xabed, 0xabed],
  [0xfb1e, 0xfb1e],
  [0xfb29, 0xfb29],
  [0xfe00, 0xfe0f],
  [0xfe20, 0xfe26],
  [0xfe50, 0xfe50],
  [0xfe52, 0xfe52],
  [0xfe55, 0xfe55],
  [0xfe5f, 0xfe5f],
  [0xfe62, 0xfe63],
  [0xfe69, 0xfe6a],
  [0xfeff, 0xfeff],
  [0xff03, 0xff05],
  [0xff0b, 0xff1a],
  [0xffe0, 0xffe1],
  [0xffe5, 0xffe6],
];
// 900 bytes

/**
 * Check if a character is a "weak" unicode character
 * @param c - input unicode character
 * @returns - True if weak
 */
export function isWeak(c: number): boolean {
  for (const arr of WEAK) {
    if (c >= arr[0] && c <= arr[1]) return true;
  }
  return false;
}

/** Unicode character types */
export const Type = {
  Rtl: 0,
  Weak: 1,
  Neutral: 2,
  Ltr: 3,
} as const;
/**
 * Enum representing the unicode character types
 * - 0 = Right to Left
 * - 1 = Weak
 * - 2 = Neutral
 * - 3 = Left to Right
 */
export type Type = (typeof Type)[keyof typeof Type];

/**
 * Get the type of a unicode character
 * @param c - input unicode character
 * @returns - The type
 */
export function getType(c: number): Type {
  if (isRTL(c)) {
    return Type.Rtl;
  } else if (isNeutral(c)) {
    return Type.Neutral;
  } else if (isWeak(c)) {
    return Type.Weak;
  }
  return Type.Ltr;
}

/**
 * Find the dominant type in the string. skip past CtrChar until RTL or LTR is found.
 * @param str - Array of unicode characters
 * @returns - The dominant type
 */
export function findDominantType(str: number[]): Type {
  for (const c of str) {
    const t = getType(c);
    if (t === Type.Rtl || t === Type.Ltr) return t;
  }
  return Type.Ltr;
}

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

export const THAI_CHARS = [0xfe70, 0xfeff];
// 4 bytes

/**
 * Check if a character is Thai
 * @param c - input unicode character
 * @returns - true if the character is Thai
 */
export function isThai(c: number): boolean {
  return c >= THAI_CHARS[0] && c <= THAI_CHARS[1];
}
