// https://learn.microsoft.com/en-us/typography/script-development/tibetan
// https://www.unicode.org/charts/PDF/U0F00.pdf
// https://r12a.github.io/scripts/tibt/bo.html

use alloc::vec::Vec;

pub fn is_tibetan(c: &u16) -> bool {
    *c >= 0x0F00 && *c <= 0x0FFF
}

const WHITESPACE: u16 =
    0x0020 | // Space
    0x0009 | // Tab
    0x000A | // Line feed
    0x000D | // Carriage return
    0x000C | // Form feed
    0x0085 | // Next line
    0x3000 | // ideographic space
    0x200B | // Zero width space
    0x00A0 | // NO-BREAK SPACE
    0x0F0C | // TIBETAN MARK DELIMITER TSHEG BSTAR
    0x202F | // NARROW NO-BREAK SPACE
    0x2060 | // WORD JOINER
    0xFEFF; // ZERO WIDTH NO-BREAK SPACE

#[derive(Debug, Clone, PartialEq)]
enum MType {
    Lh, // Head letters (0F40–0F6C, 0F88–0F8C)
    // Ls, // Subjoined letters (0F8D–0F8F, 0F90–0FBC)
    Va, // Vowel marks: Above-base (0F72, 0F7A–D, 0F80)
    Vb, // Vowel marks: Below-base (0F71, 0F74)
    Vc, // Vowel marks: Compound vowels (0F73, 0F75–0F79, 0F81) [NOTE: Use of these characters is discouraged in favor of their decomposed equivalents.]
    // Ml, // Letter modifiers (0F35, 0F37, 0F39, 0F7E–0F7F, 0F82–0F84, 0F86–0F87, 0FC6)
    // Md, // Digit modifiers (0F18–0F19, 0F3E–0F3F)
    // SD, // Syllable delimiter (0F0B–0F0C, 0F34, 0FD2)
    // B, // brackets (0F3C–0F3D)
    // GB, // Generic base character (00A0, 00D7, 2012, 2013, 2014, 2022, 25CC, and 25FB–25FE)
    // ZJ, // ZWJ/ZWNJ (200C, 200D)
    // O, // All other chars from the Tibetan block (0F00–0F0A, 0F0D–0F17, 0F1A–0F1F, 0F36, 0F38, 0F3A–0F3B, 0FBE–0FC5, 0FC7–0FD1, 0FD3–0FDA)
    U, // Unicode chars or tibetan chars that don't need to be processed
    WS, // WHITESPACE
}
impl MType {
    // note: isSpecialSequence is for K, if true, '103A, 1039' come after c
    fn from_u16(c: &u16) -> MType {
        match c {
            // Head letters (0F40–0F6C, 0F88–0F8C)
            0x0F40..=0x0F6C | 0x0F88..=0x0F8C => MType::Lh,
            // // Subjoined letters (0F8D–0F8F, 0F90–0FBC)
            // 0x0F8D..=0x0FBC => MType::Ls,
            // // Above-base (0F72, 0F7A–D, 0F80)
            0x0F72 | 0x0F7A..=0x0F7D | 0x0F80 => MType::Va,
            // Below-base (0F71, 0F74)
            0x0F71 | 0x0F74 => MType::Vb,
            // Compound vowels (0F73, 0F75–0F79, 0F81)
            0x0F73 | 0x0F75..=0x0F79 | 0x0F81 => MType::Vc,
            // // Letter modifiers (0F35, 0F37, 0F39, 0F7E–0F7F, 0F82–0F84, 0F86–0F87, 0FC6)
            // 0x0F35 | 0x0F37 | 0x0F39 | 0x0F7E..=0x0F7F | 0x0F82..=0x0F84 | 0x0F86..=0x0F87 | 0x0FC6 => MType::Ml,
            // // Digit modifiers (0F18–0F19, 0F3E–0F3F)
            // 0x0F18..=0x0F19 | 0x0F3E..=0x0F3F => MType::Md,
            // // Syllable delimiter (0F0B–0F0C, 0F34, 0FD2)
            // 0x0F0B..=0x0F0C | 0x0F34 | 0x0FD2 => MType::SD,
            // // brackets (0F3C–0F3D)
            // 0x0F3C..=0x0F3D => MType::B,
            // // Generic base character (00A0, 00D7, 2012, 2013, 2014, 2022, 25CC, and 25FB–25FE)
            // 0x00A0 | 0x00D7 | 0x2012 | 0x2013 | 0x2014 | 0x2022 | 0x25CC | 0x25FB..=0x25FE => MType::GB,
            // // ZWJ/ZWNJ (200C, 200D)
            // 0x200C | 0x200D => MType::ZJ,
            // // All other chars from the Tibetan block (0F00–0F0A, 0F0D–0F17, 0F1A–0F1F, 0F36, 0F38, 0F3A–0F3B, 0FBE–0FC5, 0FC7–0FD1, 0FD3–0FDA)
            // 0x0F00..=0x0F0A | 0x0F0D..=0x0F17 | 0x0F1A..=0x0F1F | 0x0F36 | 0x0F38 | 0x0F3A..=0x0F3B | 0x0FBE..=0x0FC5 | 0x0FC7..=0x0FD1 | 0x0FD3..=0x0FDA => MType::O,
            // whitespace
            &WHITESPACE => MType::WS,
            // Unicode chars not relating to Tibetan
            _ => MType::U,
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
            clusters.push(
                Definition::new(MType::from_u16(code), code)
            );
            idx += 1;
        }

        clusters
    }
}

struct Cluster<'a> {
    pub defs: Vec<Definition<'a>>,
    pub whitespace: Option<&'a u16>,
}
impl <'a> Cluster<'a> {
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

    /// The correct coding order for a stream of text is as follows:
    ///
    /// * head position consonant
    /// * first sub-joined consonant
    /// * ....intermediate sub-joined consonants (if any)
    /// * last sub-joined consonant
    /// * sub-joined vowel (a-chung U+0F71)
    /// * standard or compound vowel sign (including virama U+0F84 in the case of Sanskrit transliteration)
    /// * additional vowel signs (if any)
    /// * vowel modifier signs (rjes su nga ro U+0F7E, rnam bcad U+0F7F)
    fn get_sorted(&mut self) -> Vec<u16> {
        // sort
        let mut idx: usize = 0;
        while idx < self.defs.len() {
            match self.defs[idx].m_type {
                MType::Va | MType::Vb | MType::Vc => {
                    // always put the head position consonant infront of the head letter
                    let mut head_idx = idx;
                    while head_idx > 0 && self.defs[head_idx].m_type != MType::Lh { head_idx -= 1; }
                    let vowel_mark = self.defs.remove(idx);
                    self.defs.insert(head_idx, vowel_mark);
                },
                _ => {},
            }
            idx += 1;
        }

        // store
        let mut reordered = Vec::with_capacity(self.defs.len());
        for def in &self.defs { reordered.push(*def.code) }

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
/// 1) Letters: Lh [Ls*] <[Va*] | [Vb] | [Vc] > [Ml]
/// 2) Digits: D [Md]
///
/// Ex. བོད་རང་སྐྱོང་ལྗོངས།
pub fn shape_tibetan(input: &mut [u16]) {
    let mut res: Vec<u16> = Vec::with_capacity(input.len());
    // Step 1: Convert input to clusters
    let defs = Definition::build_from_unicodes(input);
    // Step 2: Split clusters by WS (white space)
    let mut clusters_sets = Cluster::build_clusters(&defs);
    // Step 2: Reorder the clusters and add them to result
    clusters_sets.iter_mut().for_each(|c| {
        res.append(&mut c.get_sorted());
        // append whitespace of cluster if it exists
        if let Some(ws) = c.whitespace { res.push(*ws); }
    });

    // now map the result to the original input
    input.copy_from_slice(&res[..input.len()]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tibetan_test_above_base() {
        let input = "བོད་རང་སྐྱོང་ལྗོངས།";
        let expected: &[u16] = &[3964, 3926, 3921, 3851, 3938, 3908, 3851, 3964, 3942, 3984, 4017, 3908, 3851, 3964, 3939, 3991, 3908, 3942, 3853];
        // Encode the string as UTF-16 and obtain a slice of u16 values
        let input_utf16_slice: Vec<u16> = input.encode_utf16().collect();
        // Create a reference to the slice
        let input_utf16_ref: &[u16] = &input_utf16_slice;
        let mut result = input_utf16_ref.to_vec();
        shape_tibetan(&mut result);
        assert_eq!(result, expected);
    }

    #[test]
    fn tibetan_test_below_base() {
        let input = "གུསར";
        let expected: &[u16] = &[3956, 3906, 3942, 3938];
        // Encode the string as UTF-16 and obtain a slice of u16 values
        let input_utf16_slice: Vec<u16> = input.encode_utf16().collect();
        // Create a reference to the slice
        let input_utf16_ref: &[u16] = &input_utf16_slice;
        let mut result = input_utf16_ref.to_vec();
        shape_tibetan(&mut result);
        assert_eq!(result, expected);
    }

    #[test]
    fn tibetan_test_compound_vowel() {
        let input = "གཱིསར";
        let expected: &[u16] = &[3955, 3906, 3942, 3938];
        // Encode the string as UTF-16 and obtain a slice of u16 values
        let input_utf16_slice: Vec<u16> = input.encode_utf16().collect();
        // Create a reference to the slice
        let input_utf16_ref: &[u16] = &input_utf16_slice;
        let mut result = input_utf16_ref.to_vec();
        shape_tibetan(&mut result);
        assert_eq!(result, expected);
    }
}
