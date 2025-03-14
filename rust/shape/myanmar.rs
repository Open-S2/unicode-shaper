// https://learn.microsoft.com/en-us/typography/script-development/myanmar
// https://www.unicode.org/charts/PDF/U1000.pdf
// https://www.unicode.org/notes/tn11/UTN11_4.pdf
// https://r12a.github.io/scripts/mymr/my.html
// https://r12a.github.io/scripts/mymr/shn.html

use crate::WHITESPACE;
use alloc::vec::Vec;

pub fn is_myanmar(c: &u16) -> bool {
    // main
    *c >= 0x1000 && *c <= 0x109F ||
    // extended A
    *c >= 0xAA60 && *c <= 0xAA7F ||
    // extended B
    *c >= 0xA9E0 && *c <= 0xA9FF
}

#[derive(Clone, PartialEq)]
enum MType {
    A, // Anusvara class (1032, 1036)
    // As, // Asat (103A)
    C, /* Consonants and Independent vowels (1000-1020, 103F, 104E, 1050, 1051, 105A-105D, 1061, 1065, 1066, 106E-1070, 1075-1081, 108E, AA60-AA6F, AA71-AA76, AA7A) */
    // D, // Myanmar digits except zero (1041-1049, 1090-1099)
    // D0, // Myanmar digit zero (1040)
    // DB, // Dot below (1037)
    // GB, // Generic base characters (00A0, 00D7, 2012–2015, 2022, 25CC, 25FB–25FE)
    // H, // Halant/virama (1039)
    // IV, // Independent vowel (1021-102A, 1052-1055)
    // J, // Joiners (200C, 200D)
    K, // A Kinzi sequence of three characters (<1004 | 101B | 105A>, 103A, 1039)
    // MH, // Medial consonants Ha, Mon La (103E, 1060)
    MR, // Medial consonants Ra (103C)
    // MW, // Medial consonants Wa, Shan Wa (103D, 1082)
    // MY, // Medial consonants Ya, Mon Na, Mon Ma (103B, 105E, 105F)
    O, // SCRIPT_COMMON characters in a Myanmar run
    // P, // Punctuation (104A, 104B)
    // PT, // Pwo and other tones (1063, 1064, 1069-106D, AA7B)
    // R, // Reserved characters from the Myanmar Extended-A block (AA7C-AA7F) & Extended-B block (A9E0-A9FF)
    // S, // Symbols (104C, 104D, 104F, 109E, 109F, AA70, AA77-AA79)
    // V, // Visarga and Shan tones (1038, 1087-108D, 108F, 109A-109C)
    // VAbv, // Above base dependent vowel (102D, 102E, 1033-1035, 1071-1074, 1085, 1086, 109D)
    VBlw, // Below base dependent vowel (102F, 1030, 1058, 1059)
    VPre, // Pre base dependent vowel (1031, 1084)
    // VPst, // Post base dependent vowel (102B, 102C, 1056, 1057, 1062, 1067, 1068, 1083)
    // VS, // Variation selectors (FE00–FE0F)
    // WJ, // Word joiner (2060)
    WS, // Whitespace (0020, 0009, 000A, 000D, 000C, 0085, 3000, 200B)
}
impl MType {
    // note: isSpecialSequence is for K, if true, '103A, 1039' come after c
    fn from_u16(c: &u16, may_be_kinzi_sequence: bool) -> MType {
        if may_be_kinzi_sequence {
            return match c {
                0x1004 | 0x101B | 0x105A => MType::K,
                _ => Self::from_u16(c, false),
            };
        }
        match c {
            // Anusvara class (1032, 1036)
            0x1032 | 0x1036 => MType::A,
            // // Asat (103A)
            // 0x103A => MType::As,
            // Consonants and Independent vowels (1000-1020, 103F, 104E, 1050, 1051, 105A-105D, 1061, 1065, 1066, 106E-1070, 1075-1081, 108E, AA60-AA6F, AA71-AA76, AA7A)
            0x1000..=0x1020
            | 0x103F
            | 0x104E
            | 0x01050
            | 0x01051
            | 0x105A..=0x105D
            | 0x1061
            | 0x1065
            | 0x1066
            | 0x106E..=0x1070
            | 0x1075..=0x1081
            | 0x108E
            | 0xAA60..=0xAA6F
            | 0xAA71..=0xAA76
            | 0xAA7A => MType::C,
            // // Myanmar digits except zero (1041-1049, 1090-1099)
            // 0x1041..=0x1049 | 0x1090..=0x1099 => MType::D,
            // // Myanmar digit zero (1040)
            // 0x1040 => MType::D0,
            // // Dot below (1037)
            // 0x1037 => MType::DB,
            // // Generic base characters (00A0, 00D7, 2012–2015, 2022, 25CC, 25FB–25FE)
            // 0x00A0 | 0x00D7 | 0x2012..=0x2015 | 0x2022 | 0x25CC | 0x25FB..=0x25FE => MType::GB,
            // // Halant/virama (1039)
            // 0x1039 => MType::H,
            // // Independent vowel (1021-102A, 1052-1055)
            // 0x1021..=0x102A | 0x1052..=0x1055 => MType::IV,
            // // Joiners (200C, 200D)
            // 0x200C | 0x200D => MType::J,
            // // Medial consonants Ha, Mon La (103E, 1060)
            // 0x103E | 0x1060 => MType::MH,
            // Medial consonants Ra (103C)
            0x103C => MType::MR,
            // // Medial consonants Wa, Shan Wa (103D, 1082)
            // 0x103D | 0x1082 => MType::MW,
            // // Medial consonants Ya, Mon Na, Mon Ma (103B, 105E, 105F)
            // 0x103B | 0x105E | 0x105F => MType::MY,
            // // Punctuation (104A, 104B)
            // 0x104A | 0x104B => MType::P,
            // // Pwo and other tones (1063, 1064, 1069-106D, AA7B)
            // 0x1063 | 0x1064 | 0x1069..=0x106D | 0xAA7B => MType::PT,
            // // Reserved characters from the Myanmar Extended-A block (AA7C-AA7F) & Extended-B block (A9E0-A9FF)
            // 0xAA7C..=0xAA7F | 0xA9E0..=0xA9FF => MType::R,
            // // Symbols (104C, 104D, 104F, 109E, 109F, AA70, AA77-AA79)
            // 0x104C | 0x104D | 0x104F | 0x109E | 0x109F | 0xAA70 | 0xAA77..=0xAA79 => MType::S,
            // // Visarga and Shan tones (1038, 1087-108D, 108F, 109A-109C)
            // 0x1038 | 0x1087..=0x108D | 0x108F | 0x109A..=0x109C => MType::V,
            // // Above base dependent vowel (102D, 102E, 1033-1035, 1071-1074, 1085, 1086, 109D)
            // 0x102D | 0x102E | 0x1033..=0x1035 | 0x1071..=0x1074 | 0x1085 | 0x1086 | 0x109D => MType::VAbv,
            // Below base dependent vowel (102F, 1030, 1058, 1059)
            0x102F | 0x1030 | 0x1058 | 0x1059 => MType::VBlw,
            // Pre base dependent vowel (1031, 1084)
            0x1031 | 0x1084 => MType::VPre,
            // // Post base dependent vowel (102B, 102C, 1056, 1057, 1062, 1067, 1068, 1083)
            // 0x102B | 0x102C | 0x1056 | 0x1057 | 0x1062 | 0x1067 | 0x1068 | 0x1083 => MType::VPst,
            // // Variation selectors (FE00–FE0F)
            // 0xFE00..=0xFE0F => MType::VS,
            // // Word joiner (2060)
            // 0x2060 => MType::WJ,
            // Whitespace (0020, 0009, 000A, 000D, 000C, 0085, 3000, 200B)
            &WHITESPACE => MType::WS,
            _ => MType::O,
        }
    }
}

#[derive(Clone)]
struct Definition<'a> {
    // cluster definition
    m_type: MType,
    // reference to a slice of an original string
    code: &'a u16,
}
impl<'a> Definition<'a> {
    fn new(m_type: MType, code: &'a u16) -> Self {
        Self { m_type, code }
    }

    fn build_from_unicodes(input: &[u16]) -> Vec<Definition> {
        let mut clusters = Vec::new();

        let mut idx: usize = 0;
        while idx < input.len() {
            let code = &input[idx];
            let may_be_kinzi_sequence: bool =
                idx + 2 < input.len() && input[idx + 1] == 0x103A && input[idx + 2] == 0x1039;
            clusters.push(Definition::new(MType::from_u16(code, may_be_kinzi_sequence), code));
            if may_be_kinzi_sequence {
                idx += 3;
            } else {
                idx += 1;
            }
        }

        clusters
    }
}

struct Cluster<'a> {
    pub defs: Vec<Definition<'a>>,
    pub whitespace: Option<&'a u16>,
}
impl<'a> Cluster<'a> {
    fn new(defs: Vec<Definition<'a>>, whitespace: Option<&'a u16>) -> Self {
        Self { defs, whitespace }
    }

    fn build_clusters(defs: &'a [Definition<'a>]) -> Vec<Cluster<'a>> {
        let mut clusters = Vec::new();

        let mut def_idx = 0;
        for idx in 0..defs.len() {
            if defs[idx].m_type == MType::WS {
                clusters.push(Cluster::new(defs[def_idx..idx].to_vec(), Some(defs[idx].code)));
                def_idx = idx + 1;
            }
        }
        // store last
        if def_idx < defs.len() {
            clusters.push(Cluster::new(defs[def_idx..].to_vec(), None));
        }

        clusters
    }

    /// 1) Kinzi sequences (K) are reordered directly after the cluster base
    /// 2) The medial ra (MR) is reordered before the base consonant
    /// 3) Pre-base vowels (VPre) are reordered to the start of the syllable cluster.
    ///    A sequence of multiple prebase vowels is permitted. Such sequences are moved
    ///    as a block to the beginning of the cluster.
    /// 4) Anusvara (A) coming immediately after one or more below-base vowels (VBlw)
    ///    will reorder immediately before them.
    fn get_sorted(&mut self) -> Vec<u16> {
        // sort
        let mut idx: usize = 0;
        while idx < self.defs.len() {
            match self.defs[idx].m_type {
                MType::K => {
                    // Kinzi sequences (K) are reordered directly after the cluster base.
                    // K always precedes the base consonant
                    if idx + 1 < self.defs.len() {
                        self.defs.swap(idx, idx + 1);
                        idx += 1;
                    }
                }
                MType::MR => {
                    // The medial ra (MR) is reordered before the base consonant
                    let mut base_c_idx = 0;
                    while base_c_idx + 1 < self.defs.len()
                        && self.defs[base_c_idx].m_type != MType::C
                    {
                        base_c_idx += 1;
                    }
                    if base_c_idx != idx {
                        let v_pre = self.defs.remove(idx);
                        self.defs.insert(base_c_idx, v_pre);
                    }
                }
                MType::VPre => {
                    // Pre-base vowels (VPre) are reordered to the start of the syllable cluster.
                    let v_pre = self.defs.remove(idx);
                    self.defs.insert(0, v_pre);
                }
                MType::A => {
                    // Anusvara (A) coming immediately after one or more below-base vowels (VBlw)
                    let mut prev_idx = idx;
                    while prev_idx - 1 > 0 && self.defs[prev_idx - 1].m_type == MType::VBlw {
                        prev_idx -= 1;
                    }
                    if prev_idx != idx {
                        self.defs.swap(prev_idx, idx);
                    }
                }
                _ => {}
            }
            idx += 1;
        }

        // store
        let mut reordered = Vec::with_capacity(self.defs.len());
        for def in &self.defs {
            match def.m_type {
                MType::K => {
                    reordered.extend_from_slice(&[*def.code, 0x103A, 0x1039]);
                }
                _ => reordered.push(*def.code),
            }
        }

        reordered
    }
}

/// Shape/Reordering characters
/// Once the Myanmar shaping engine has analyzed the run as described above,
/// it creates a buffer of appropriately reordered elements (glyphs) representing the
/// cluster according to the rules given:
///
/// 1) Kinzi sequences (K) are reordered directly after the cluster base
/// 2) The medial ra (MR) is reordered before the base consonant
/// 3) Pre-base vowels (VPre) are reordered to the start of the syllable cluster.
///    A sequence of multiple prebase vowels is permitted. Such sequences are moved
///    as a block to the beginning of the cluster.
/// 4) Anusvara (A) coming immediately after one or more below-base vowels (VBlw)
///    will reorder immediately before them.
///
/// Cases:
/// 1) Simple non-compounding cluster:   <P | S | R | WJ| WS | O | D0 >
/// 2) MType terminating in Halant: [K] <C | IV | D | GB>[VS] (H <C | IV> [VS])* H
/// 3) Complex cluster:                  [K] <C | IV | D | GB>[VS] (H <C | IV> [VS]) (As) [MY [As]] [MR] [<MW [As] | [MW] MH [As]>] (VPre) (VAbv)* (VBlw) (A) [DB [As]] (VPst [MH] (As)* (VAbv)* (A)* [DB [As]]) (PT < [A] [DB] [As] | [As] [A] > ) (V)* [J]
///
/// Ex. င်္က္ကျြွှေို့်ာှီ့ၤဲံ့းႍ
/// INPUT - 1004 103A 1039 1000 1039 1000 103B 103C 103D 1031 1031 102D 102F 1036 102C 1036
/// I-EXPLAINED - ([K] 1004 103A 1039) - ([C] 1000) - ([H] 1039) - ([C] 1000) - ([MY] 103B) - ([MR] 103C) - ([MW] 103D) - ([VPre] 1031) - ([VPre] 1031) - ([VAbv] 102D) - ([VBlw] 102F) - ([A] 1036) - ([VPst] 102C) - ([A] 1036)
/// REORDERED - 1031 1031 103C 1000 1004 103A 1039 1039 1000 103B 103D 102D 1036 102F 102C 1036
/// R-EXPLAINED - ([VPre] 1031) - ([VPre] 1031) - ([MR] 103C) - ([C] 1000) - ([K] 1004 103A 1039) - ([H] 1039) - ([C] 1000) - ([MY] 103B) - ([MW] 103D) - ([VAbv] 102D) - ([A] 1036) - ([VPst] 102F) - ([VPst] 102C) - ([A] 1036)
pub fn shape_myanmar(input: &mut [u16]) {
    let mut res: Vec<u16> = Vec::with_capacity(input.len());
    // Step 1: Convert input to clusters
    let defs = Definition::build_from_unicodes(input);
    // Step 2: Split clusters by WS (white space)
    let mut clusters_sets = Cluster::build_clusters(&defs);
    // Step 3: Reorder the clusters and add them to result
    clusters_sets.iter_mut().for_each(|c| {
        res.append(&mut c.get_sorted());
        // append whitespace of cluster if it exists
        if let Some(ws) = c.whitespace {
            res.push(*ws);
        }
    });

    // now map the result to the original input
    input.copy_from_slice(&res[..input.len()]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn myanmar_complex_2_test() {
        let input: &[u16] = &[
            0x1004, 0x103A, 0x1039, 0x1000, 0x1039, 0x1000, 0x103B, 0x103C, 0x103D, 0x1031, 0x1031,
            0x102D, 0x102F, 0x1036, 0x102C, 0x1036,
        ];
        //                      [1031, 1031, 103c, 1000, 1004, 103a, 1039, 1039, 1000, 103b, 1036, 103d, 102c, 1036, 102d, 102f]
        let expected: &[u16] = &[
            0x1031, 0x1031, 0x103C, 0x1000, 0x1004, 0x103A, 0x1039, 0x1039, 0x1000, 0x103B, 0x103D,
            0x102D, 0x1036, 0x102F, 0x102C, 0x1036,
        ];
        let mut result = input.to_vec();
        shape_myanmar(&mut result);
        assert_eq!(result, expected);
    }

    #[test]
    fn myanmar_complex_test() {
        let input = "င်္က္ကျြွှေို့်ာှီ့ၤဲံ့းႍ";
        let expected: &[u16] = &[
            4145, 4156, 4096, 4100, 4154, 4153, 4153, 4096, 4155, 4157, 4158, 4141, 4143, 4151,
            4154, 4140, 4158, 4142, 4151, 4196, 4146, 4150, 4151, 4152, 4237,
        ];
        // Encode the string as UTF-16 and obtain a slice of u16 values
        let input_utf16_slice: Vec<u16> = input.encode_utf16().collect();
        // Create a reference to the slice
        let input_utf16_ref: &[u16] = &input_utf16_slice;
        let mut result = input_utf16_ref.to_vec();
        shape_myanmar(&mut result);
        assert_eq!(result, expected);
    }

    #[test]
    fn myanmar_complex_3_test() {
        let input = "မြန်မာ"; // 4121, 4156, 4116, 4154, 4121, 4140
        let expected: &[u16] = &[4156, 4121, 4116, 4154, 4121, 4140];
        // Encode the string as UTF-16 and obtain a slice of u16 values
        let input_utf16_slice: Vec<u16> = input.encode_utf16().collect();
        // Create a reference to the slice
        let input_utf16_ref: &[u16] = &input_utf16_slice;
        let mut result = input_utf16_ref.to_vec();
        shape_myanmar(&mut result);
        assert_eq!(result, expected);
    }
}
