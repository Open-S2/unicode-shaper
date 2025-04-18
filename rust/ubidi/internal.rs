/// If a RTL set is reversed, but surounded by (), [], {}, or <>, then mirror the sets.
/// This also tracks other special characters that need to be mirrored. Some examples:
/// '«', '»', '∕', '∟', '∠', '∡', '∢', '∤', '≃', '≅', '≌', '⊘', '⊦', '⊨',
pub static MIRROR_CHAR: [[u16; 2]; 48] = [
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

/// Adjust string characters that have mirrored characters. Examples:
/// '«', '»', '∕', '∟', '∠', '∡', '∢', '∤', '≃', '≅', '≌', '⊘', '⊦', '⊨',
pub fn mirror_adjust_string(s: &mut [u16]) {
    for c in s {
        for from_to in MIRROR_CHAR {
            if *c == from_to[0] {
                *c = from_to[1];
                break;
            }
        }
    }
}

/// A RandALCat character is a character with unambiguously right-to-left directionality.
pub static RAND_AL_CAT: [[u16; 2]; 45] = [
    [0x05BE, 0x05BE],
    [0x05C0, 0x05C0],
    [0x05C3, 0x05C3],
    [0x05C6, 0x05C6],
    [0x05D0, 0x05EA],
    [0x05F0, 0x05F4],
    [0x0608, 0x0608],
    [0x060B, 0x060B],
    [0x060D, 0x060D],
    [0x061B, 0x061B],
    [0x061E, 0x064A],
    [0x066D, 0x066F],
    [0x0671, 0x06D5],
    [0x06E5, 0x06E6],
    [0x06EE, 0x06EF],
    [0x06FA, 0x070D],
    [0x0710, 0x0710],
    [0x0712, 0x072F],
    [0x074D, 0x07A5],
    [0x07B1, 0x07B1],
    [0x07C0, 0x07EA],
    [0x07F4, 0x07F5],
    [0x07FA, 0x07FA],
    [0x0800, 0x0815],
    [0x081A, 0x081A],
    [0x0824, 0x0824],
    [0x0828, 0x0828],
    [0x0830, 0x083E],
    [0x0840, 0x0858],
    [0x085E, 0x085E],
    [0x200F, 0x200F],
    [0xFB1D, 0xFB1D],
    [0xFB1F, 0xFB28],
    [0xFB2A, 0xFB36],
    [0xFB38, 0xFB3C],
    [0xFB3E, 0xFB3E],
    [0xFB40, 0xFB41],
    [0xFB43, 0xFB44],
    [0xFB46, 0xFBC1],
    [0xFBD3, 0xFD3D],
    [0xFD50, 0xFD8F],
    [0xFD92, 0xFDC7],
    [0xFDF0, 0xFDFC],
    [0xFE70, 0xFE74],
    [0xFE76, 0xFEFC],
];
// 168 bytes

/// Check if a character is RandALCat (Right-to-left reading characters)
pub fn is_rtl(c: &u16) -> bool {
    for arr in RAND_AL_CAT {
        if *c >= arr[0] && *c <= arr[1] {
            return true;
        }
    }
    false
}

/// NEUTRAL characters are those with no inherent directionality, which can be
/// treated as being part of any adjacent runs of text with other directionality.
pub static NEUTRAL: [[u16; 2]; 137] = [
    [0x0009, 0x000D],
    [0x001C, 0x0022],
    [0x0026, 0x002A],
    [0x003B, 0x0040],
    [0x005B, 0x0060],
    [0x007B, 0x007E],
    [0x0085, 0x0085],
    [0x00A1, 0x00A1],
    [0x00A6, 0x00A9],
    [0x00AB, 0x00AC],
    [0x00AE, 0x00AF],
    [0x00B4, 0x00B4],
    [0x00B6, 0x00B8],
    [0x00BB, 0x00BF],
    [0x00D7, 0x00D7],
    [0x00F7, 0x00F7],
    [0x02B9, 0x02BA],
    [0x02C2, 0x02CF],
    [0x02D2, 0x02DF],
    [0x02E5, 0x02ED],
    [0x02EF, 0x02FF],
    [0x0374, 0x0375],
    [0x037E, 0x037E],
    [0x0384, 0x0385],
    [0x0387, 0x0387],
    [0x03F6, 0x03F6],
    [0x058A, 0x058A],
    [0x0606, 0x0607],
    [0x060E, 0x060F],
    [0x06DE, 0x06DE],
    [0x06E9, 0x06E9],
    [0x07F6, 0x07F9],
    [0x0BF3, 0x0BF8],
    [0x0BFA, 0x0BFA],
    [0x0C78, 0x0C7E],
    [0x0F3A, 0x0F3D],
    [0x1390, 0x1399],
    [0x1400, 0x1400],
    [0x1680, 0x1680],
    [0x169B, 0x169C],
    [0x17F0, 0x17F9],
    [0x1800, 0x180A],
    [0x180E, 0x180E],
    [0x1940, 0x1940],
    [0x1944, 0x1945],
    [0x19DE, 0x19FF],
    [0x1FBD, 0x1FBD],
    [0x1FBF, 0x1FC1],
    [0x1FCD, 0x1FCF],
    [0x1FDD, 0x1FDF],
    [0x1FED, 0x1FEF],
    [0x1FFD, 0x1FFE],
    [0x2000, 0x200A],
    [0x2010, 0x202E],
    [0x2035, 0x2043],
    [0x2045, 0x205F],
    [0x207C, 0x207E],
    [0x208C, 0x208E],
    [0x2100, 0x2101],
    [0x2103, 0x2106],
    [0x2108, 0x2109],
    [0x2114, 0x2114],
    [0x2116, 0x2118],
    [0x211E, 0x2123],
    [0x2125, 0x2125],
    [0x2127, 0x2127],
    [0x2129, 0x2129],
    [0x213A, 0x213B],
    [0x2140, 0x2144],
    [0x214A, 0x214D],
    [0x2150, 0x215F],
    [0x2189, 0x2189],
    [0x2190, 0x2211],
    [0x2214, 0x2335],
    [0x237B, 0x2394],
    [0x2396, 0x23F3],
    [0x2400, 0x2426],
    [0x2440, 0x244A],
    [0x2460, 0x2487],
    [0x24EA, 0x26AB],
    [0x26AD, 0x26FF],
    [0x2701, 0x27CA],
    [0x27CC, 0x27CC],
    [0x27CE, 0x27FF],
    [0x2900, 0x2B4C],
    [0x2B50, 0x2B59],
    [0x2CE5, 0x2CEA],
    [0x2CF9, 0x2CFF],
    [0x2E00, 0x2E31],
    [0x2E80, 0x2E99],
    [0x2E9B, 0x2EF3],
    [0x2F00, 0x2FD5],
    [0x2FF0, 0x2FFB],
    [0x3000, 0x3004],
    [0x3008, 0x3020],
    [0x3030, 0x3030],
    [0x3036, 0x3037],
    [0x303D, 0x303F],
    [0x309B, 0x309C],
    [0x30A0, 0x30A0],
    [0x30FB, 0x30FB],
    [0x31C0, 0x31E3],
    [0x321D, 0x321E],
    [0x3250, 0x325F],
    [0x327C, 0x327E],
    [0x32B1, 0x32BF],
    [0x32CC, 0x32CF],
    [0x3377, 0x337A],
    [0x33DE, 0x33DF],
    [0x33FF, 0x33FF],
    [0x4DC0, 0x4DFF],
    [0xA490, 0xA4C6],
    [0xA60D, 0xA60F],
    [0xA673, 0xA673],
    [0xA67E, 0xA67F],
    [0xA700, 0xA721],
    [0xA788, 0xA788],
    [0xA828, 0xA82B],
    [0xA874, 0xA877],
    [0xFD3E, 0xFD3F],
    [0xFDFD, 0xFDFD],
    [0xFE10, 0xFE19],
    [0xFE30, 0xFE4F],
    [0xFE51, 0xFE51],
    [0xFE54, 0xFE54],
    [0xFE56, 0xFE5E],
    [0xFE60, 0xFE61],
    [0xFE64, 0xFE66],
    [0xFE68, 0xFE68],
    [0xFE6B, 0xFE6B],
    [0xFF01, 0xFF02],
    [0xFF06, 0xFF0A],
    [0xFF1B, 0xFF20],
    [0xFF3B, 0xFF40],
    [0xFF5B, 0xFF65],
    [0xFFE2, 0xFFE4],
    [0xFFE8, 0xFFEE],
];
// 536 bytes

/// Check if a character is NeutralCat (Neutral characters)
pub fn is_neutral(c: &u16) -> bool {
    for arr in NEUTRAL {
        if *c >= arr[0] && *c <= arr[1] {
            return true;
        }
    }
    false
}

/// List of WeakCat (Weak characters)
pub static WEAK: [[u16; 2]; 228] = [
    [0x0000, 0x0008],
    [0x000E, 0x001B],
    [0x0023, 0x0025],
    [0x002B, 0x003A],
    [0x007F, 0x0084],
    [0x0086, 0x00A0],
    [0x00A2, 0x00A5],
    [0x00AD, 0x00AD],
    [0x00B0, 0x00B3],
    [0x00B9, 0x00B9],
    [0x0300, 0x036F],
    [0x0483, 0x0489],
    [0x0591, 0x05BD],
    [0x05BF, 0x05BF],
    [0x05C1, 0x05C2],
    [0x05C4, 0x05C5],
    [0x05C7, 0x05C7],
    [0x0600, 0x0603],
    [0x0609, 0x060A],
    [0x060C, 0x060C],
    [0x0610, 0x061A],
    [0x064B, 0x066C],
    [0x0670, 0x0670],
    [0x06D6, 0x06DD],
    [0x06DF, 0x06E4],
    [0x06E7, 0x06E8],
    [0x06EA, 0x06ED],
    [0x06F0, 0x06F9],
    [0x070F, 0x070F],
    [0x0711, 0x0711],
    [0x0730, 0x074A],
    [0x07A6, 0x07B0],
    [0x07EB, 0x07F3],
    [0x0816, 0x0819],
    [0x081B, 0x0823],
    [0x0825, 0x0827],
    [0x0829, 0x082D],
    [0x0859, 0x085B],
    [0x0900, 0x0902],
    [0x093A, 0x093A],
    [0x093C, 0x093C],
    [0x0941, 0x0948],
    [0x094D, 0x094D],
    [0x0951, 0x0957],
    [0x0962, 0x0963],
    [0x0981, 0x0981],
    [0x09BC, 0x09BC],
    [0x09C1, 0x09C4],
    [0x09CD, 0x09CD],
    [0x09E2, 0x09E3],
    [0x09F2, 0x09F3],
    [0x09FB, 0x09FB],
    [0x0A01, 0x0A02],
    [0x0A3C, 0x0A3C],
    [0x0A41, 0x0A42],
    [0x0A47, 0x0A48],
    [0x0A4B, 0x0A4D],
    [0x0A51, 0x0A51],
    [0x0A70, 0x0A71],
    [0x0A75, 0x0A75],
    [0x0A81, 0x0A82],
    [0x0ABC, 0x0ABC],
    [0x0AC1, 0x0AC5],
    [0x0AC7, 0x0AC8],
    [0x0ACD, 0x0ACD],
    [0x0AE2, 0x0AE3],
    [0x0AF1, 0x0AF1],
    [0x0B01, 0x0B01],
    [0x0B3C, 0x0B3C],
    [0x0B3F, 0x0B3F],
    [0x0B41, 0x0B44],
    [0x0B4D, 0x0B4D],
    [0x0B56, 0x0B56],
    [0x0B62, 0x0B63],
    [0x0B82, 0x0B82],
    [0x0BC0, 0x0BC0],
    [0x0BCD, 0x0BCD],
    [0x0BF9, 0x0BF9],
    [0x0C3E, 0x0C40],
    [0x0C46, 0x0C48],
    [0x0C4A, 0x0C4D],
    [0x0C55, 0x0C56],
    [0x0C62, 0x0C63],
    [0x0CBC, 0x0CBC],
    [0x0CCC, 0x0CCD],
    [0x0CE2, 0x0CE3],
    [0x0D41, 0x0D44],
    [0x0D4D, 0x0D4D],
    [0x0D62, 0x0D63],
    [0x0DCA, 0x0DCA],
    [0x0DD2, 0x0DD4],
    [0x0DD6, 0x0DD6],
    [0x0E31, 0x0E31],
    [0x0E34, 0x0E3A],
    [0x0E3F, 0x0E3F],
    [0x0E47, 0x0E4E],
    [0x0EB1, 0x0EB1],
    [0x0EB4, 0x0EB9],
    [0x0EBB, 0x0EBC],
    [0x0EC8, 0x0ECD],
    [0x0F18, 0x0F19],
    [0x0F35, 0x0F35],
    [0x0F37, 0x0F37],
    [0x0F39, 0x0F39],
    [0x0F71, 0x0F7E],
    [0x0F80, 0x0F84],
    [0x0F86, 0x0F87],
    [0x0F8D, 0x0F97],
    [0x0F99, 0x0FBC],
    [0x0FC6, 0x0FC6],
    [0x102D, 0x1030],
    [0x1032, 0x1037],
    [0x1039, 0x103A],
    [0x103D, 0x103E],
    [0x1058, 0x1059],
    [0x105E, 0x1060],
    [0x1071, 0x1074],
    [0x1082, 0x1082],
    [0x1085, 0x1086],
    [0x108D, 0x108D],
    [0x109D, 0x109D],
    [0x135D, 0x135F],
    [0x1712, 0x1714],
    [0x1732, 0x1734],
    [0x1752, 0x1753],
    [0x1772, 0x1773],
    [0x17B7, 0x17BD],
    [0x17C6, 0x17C6],
    [0x17C9, 0x17D3],
    [0x17DB, 0x17DB],
    [0x17DD, 0x17DD],
    [0x180B, 0x180D],
    [0x18A9, 0x18A9],
    [0x1920, 0x1922],
    [0x1927, 0x1928],
    [0x1932, 0x1932],
    [0x1939, 0x193B],
    [0x1A17, 0x1A18],
    [0x1A56, 0x1A56],
    [0x1A58, 0x1A5E],
    [0x1A60, 0x1A60],
    [0x1A62, 0x1A62],
    [0x1A65, 0x1A6C],
    [0x1A73, 0x1A7C],
    [0x1A7F, 0x1A7F],
    [0x1B00, 0x1B03],
    [0x1B34, 0x1B34],
    [0x1B36, 0x1B3A],
    [0x1B3C, 0x1B3C],
    [0x1B42, 0x1B42],
    [0x1B6B, 0x1B73],
    [0x1B80, 0x1B81],
    [0x1BA2, 0x1BA5],
    [0x1BA8, 0x1BA9],
    [0x1BE6, 0x1BE6],
    [0x1BE8, 0x1BE9],
    [0x1BED, 0x1BED],
    [0x1BEF, 0x1BF1],
    [0x1C2C, 0x1C33],
    [0x1C36, 0x1C37],
    [0x1CD0, 0x1CD2],
    [0x1CD4, 0x1CE0],
    [0x1CE2, 0x1CE8],
    [0x1CED, 0x1CED],
    [0x1DC0, 0x1DE6],
    [0x1DFC, 0x1DFF],
    [0x200B, 0x200D],
    [0x202F, 0x2034],
    [0x2044, 0x2044],
    [0x2060, 0x2064],
    [0x206A, 0x2070],
    [0x2074, 0x207B],
    [0x2080, 0x208B],
    [0x20A0, 0x20B9],
    [0x20D0, 0x20F0],
    [0x212E, 0x212E],
    [0x2212, 0x2213],
    [0x2488, 0x249B],
    [0x2CEF, 0x2CF1],
    [0x2D7F, 0x2D7F],
    [0x2DE0, 0x2DFF],
    [0x302A, 0x302F],
    [0x3099, 0x309A],
    [0xA66F, 0xA672],
    [0xA67C, 0xA67D],
    [0xA6F0, 0xA6F1],
    [0xA802, 0xA802],
    [0xA806, 0xA806],
    [0xA80B, 0xA80B],
    [0xA825, 0xA826],
    [0xA838, 0xA839],
    [0xA8C4, 0xA8C4],
    [0xA8E0, 0xA8F1],
    [0xA926, 0xA92D],
    [0xA947, 0xA951],
    [0xA980, 0xA982],
    [0xA9B3, 0xA9B3],
    [0xA9B6, 0xA9B9],
    [0xA9BC, 0xA9BC],
    [0xA9BC, 0xA9BC],
    [0xAA29, 0xAA2E],
    [0xAA31, 0xAA32],
    [0xAA35, 0xAA36],
    [0xAA43, 0xAA43],
    [0xAA4C, 0xAA4C],
    [0xAAB0, 0xAAB0],
    [0xAAB2, 0xAAB4],
    [0xAAB7, 0xAAB8],
    [0xAABE, 0xAABF],
    [0xAAC1, 0xAAC1],
    [0xABE5, 0xABE5],
    [0xABE8, 0xABE8],
    [0xABED, 0xABED],
    [0xFB1E, 0xFB1E],
    [0xFB29, 0xFB29],
    [0xFE00, 0xFE0F],
    [0xFE20, 0xFE26],
    [0xFE50, 0xFE50],
    [0xFE52, 0xFE52],
    [0xFE55, 0xFE55],
    [0xFE5F, 0xFE5F],
    [0xFE62, 0xFE63],
    [0xFE69, 0xFE6A],
    [0xFEFF, 0xFEFF],
    [0xFF03, 0xFF05],
    [0xFF0B, 0xFF1A],
    [0xFFE0, 0xFFE1],
    [0xFFE5, 0xFFE6],
];
// 900 bytes

/// Check if a character is WeakCat (Weak characters)
pub fn is_weak(c: &u16) -> bool {
    for arr in WEAK {
        if *c >= arr[0] && *c <= arr[1] {
            return true;
        }
    }
    false
}

/// Text types
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Type {
    /// Right-to-left
    Rtl,
    /// Weak (affected relative to the direction of text flow)
    Weak,
    /// Neutral
    Neutral,
    /// Left-to-right
    Ltr,
}

/// Get the type of a character
pub fn get_type(c: &u16) -> Type {
    if is_rtl(c) {
        return Type::Rtl;
    }
    if is_neutral(c) {
        return Type::Neutral;
    }
    if is_weak(c) {
        return Type::Weak;
    }
    Type::Ltr
}

/// Find the dominant type in the string. skip past CtrChar until RTL or LTR is found.
pub fn find_dominant_type(str: &[u16]) -> Type {
    for c in str {
        let t: Type = get_type(c);
        if t == Type::Rtl || t == Type::Ltr {
            return t;
        }
    }
    Type::Ltr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_is_rtl() {
        assert!(is_rtl(&0x05C3));
        assert!(!is_rtl(&0x01));
    }
}
