// https://www.unicode.org/charts/PDF/U1780.pdf
// https://r12a.github.io/scripts/khmr/km.html

use crate::WHITESPACE;
use alloc::vec::Vec;

pub fn is_khmer(c: &u16) -> bool {
    // 1780–17FF
    *c >= 0x1780 && *c <= 0x17FF
}

#[derive(Debug, Clone, PartialEq)]
enum MType {
    Cs1, /* Consonant - SubscriptType1 (U+1780-U+1782, U+1784-U+1787, U+1789-U+178C, U+178E-U+1793, U+1795-U+1798, U+179B-U+179D, U+17A0, U+17A2) */
    Cs2, /* Consonant - SubscriptType2 (U+179A, U+1783, U+1788, U+178D, U+1794, U+1799, U+179E-U+179F, U+17A1) */
    V,   // Independent Vowel (U+17B4-U+17B5)
    Vs1, // Idependent Vowel - SubscriptType1 (U+17A3-U+17B3)
    VAbv, // Above base vowel (U+17B7-U+17BA, U+17BE (split))
    VBlw, // Below base vowel (U+17BB-U+17BD)
    VPre, // Pre base vowel (U+17C1-U+17C3)
    VPst, // Post base vowel (U+17B6, U+17BF-U+17C0 (split), U+17C4-U+17C5 (split))
    Coeng, // U+17D2
    RS,  // Register Shifter (U+17C9-U+17CA)
    Robat, // U+17CC
    SAbv, // Above base Sign (U+17C6, U+17CB, U+17CD-U+17D1, U+17DD)
    SPst, // Post base Sign (U+17C7-U+17C8)
    SAbvN, // Above base Sign for numbers (U+17D3)
    P,   // Punctuation (U+17D4-U+17DA, U+17DC, U+19E0-U+19FF)
    C,   // Currency (U+17DB)
    N,   // Number (U+17E0-U+17E9, U+17F0-U+17F9)
    R,   // Reserved (U+17DE-U+17DF, U+17EA-U+17EF, U+17FA-U+17FF)
    J,   // Joiners (200D ZWJ (Zero Width Joiner) & 034F CGJ (COMBINING GRAPHEME JOINER))
    VS,  // Variation selectors (FE00–FE0F)
    WJ,  // Word joiner (2060)
    NJ,  // Non-joiner (200C) [Zero Width Non-Joiner]
    WS,  // Whitespace (0020, 0009, 000A, 000D, 000C, 0085, 3000, 200B)
    O,   // other characters with no relation to khmer
}
impl MType {
    // note: isSpecialSequence is for K, if true, '103A, 1039' come after c
    fn from_u16(c: &u16) -> MType {
        match c {
            // Consonant - SubscriptType1 (U+1780-U+1782, U+1784-U+1787, U+1789-U+178C, U+178E-U+1793, U+1795-U+1798, U+179B-U+179D, U+17A0, U+17A2)
            0x1780..=0x1782
            | 0x1784..=0x1787
            | 0x1789..=0x178C
            | 0x178E..=0x1793
            | 0x1795..=0x1798
            | 0x179B..=0x179D
            | 0x17A0
            | 0x17A2 => MType::Cs1,
            // Consonant - SubscriptType2 (U+179A, U+1783, U+1788, U+178D, U+1794, U+1799, U+179E-U+179F, U+17A1)
            0x179A | 0x1783 | 0x1788 | 0x178D | 0x1794 | 0x1799 | 0x179E..=0x179F | 0x17A1 => {
                MType::Cs2
            }
            // Independent Vowel (U+17B4-U+17B5)
            0x17B4..=0x17B5 => MType::V,
            // Idependent Vowel - SubscriptType1 (U+17A3-U+17B3)
            0x17A3..=0x17B3 => MType::Vs1,
            // Above base vowel (U+17B7-U+17BA, U+17BE (split))
            0x17B7..=0x17BA | 0x17BE => MType::VAbv,
            // Below base vowel (U+17BB-U+17BD)
            0x17BB..=0x17BD => MType::VBlw,
            // Pre base vowel (U+17C1-U+17C3)
            0x17C1..=0x17C3 => MType::VPre,
            // Post base vowel (U+17B6, U+17BF-U+17C0 (split), U+17C4-U+17C5 (split))
            0x17B6 | 0x17BF..=0x17C0 | 0x17C4..=0x17C5 => MType::VPst,
            // U+17D2
            0x17D2 => MType::Coeng,
            // Register Shifter (U+17C9-U+17CA)
            0x17C9..=0x17CA => MType::RS,
            // Robat (U+17CC)
            0x17CC => MType::Robat,
            // Above base Sign (U+17C6, U+17CB, U+17CD-U+17D1, U+17DD)
            0x17C6 | 0x17CB | 0x17CD..=0x17D1 | 0x17DD => MType::SAbv,
            // Post base Sign (U+17C7-U+17C8)
            0x17C7..=0x17C8 => MType::SPst,
            // Above base Sign for numbers (U+17D3)
            0x17D3 => MType::SAbvN,
            // Punctuation (U+17D4-U+17DA, U+17DC, U+19E0-U+19FF)
            0x17D4..=0x17DA | 0x17DC | 0x19E0..=0x19FF => MType::P,
            // Currency (U+17DB)
            0x17DB => MType::C,
            // Number (U+17E0-U+17E9, U+17F0-U+17F9)
            0x17E0..=0x17E9 | 0x17F0..=0x17F9 => MType::N,
            // Reserved (U+17DE-U+17DF, U+17EA-U+17EF, U+17FA-U+17FF)
            0x17DE..=0x17DF | 0x17EA..=0x17EF | 0x17FA..=0x17FF => MType::R,
            // Joiners (200C, 200D)
            0x200D | 0x034F => MType::J,
            // Variation selectors (FE00–FE0F)
            0xFE00..=0xFE0F => MType::VS,
            // Word joiner (2060)
            0x2060 => MType::WJ,
            // Non-joiner (200C) [Zero Width Non-Joiner]
            0x200C => MType::NJ,
            // Whitespace (0020, 0009, 000A, 000D, 000C, 0085, 3000, 200B)
            &WHITESPACE => MType::WS,
            _ => MType::O,
        }
    }

    fn is_same(&self, other: &u16) -> bool {
        let other_type = MType::from_u16(other);
        *self == other_type || other_type == MType::J
    }
}

#[derive(Clone)]
struct Definition<'a> {
    // cluster definition
    m_type: MType,
    // reference to a slice of an original string
    code: &'a [u16],
}
impl<'a> Definition<'a> {
    fn new(m_type: MType, code: &'a [u16]) -> Self {
        Self { m_type, code }
    }

    fn build_definition(input: &[u16]) -> Vec<Definition> {
        let mut clusters = Vec::new();

        let mut idx: usize = 0;
        while idx < input.len() {
            let start_idx = idx;
            let code = &input[idx];
            let m_type = MType::from_u16(code);
            let mut end_idx = idx + 1;
            // while we find joiners and the same type, continue
            while end_idx < input.len() && MType::is_same(&m_type, &input[end_idx]) {
                end_idx += 1;
            }
            clusters.push(Definition::new(m_type, &input[start_idx..end_idx]));
            idx = end_idx;
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
            if defs[idx].m_type == MType::WS || defs[idx].m_type == MType::NJ {
                clusters.push(Cluster::new(defs[def_idx..idx].to_vec(), Some(&defs[idx].code[0])));
                def_idx = idx + 1;
            }
        }
        // store last
        if def_idx < defs.len() {
            clusters.push(Cluster::new(defs[def_idx..].to_vec(), None));
        }

        clusters
    }

    /// Once the Buginese shaping engine has analyzed the run into
    /// clusters as described above, it performs any required reordering.
    /// Pre-base vowels (VPre) are reordered to the start of the syllable
    /// cluster. A sequence of multiple pre-base vowels is permitted.
    /// Such sequences are moved as a block to the beginning of the cluster.
    /// In the following example, the run of code points represents a
    /// single cluster.
    fn get_sorted(&mut self) -> Vec<u16> {
        // sort
        let mut idx: usize = 0;
        while idx < self.defs.len() {
            match self.defs[idx].m_type {
                MType::VPre => {
                    // Pre-base vowels (VPre) are reordered to the start of the syllable cluster.
                    let v_pre = self.defs.remove(idx);
                    self.defs.insert(0, v_pre);
                }
                MType::VAbv | MType::VBlw | MType::VPst => {
                    // // always put the head position consonant infront of the head letter
                    // let mut head_idx = idx;
                    // while head_idx > 0 && self.defs[head_idx].m_type != MType::C && self.defs[head_idx].m_type != MType::GB { head_idx -= 1; }
                    // let vowel_sign = self.defs.remove(idx);
                    // self.defs.insert(head_idx, vowel_sign);
                }
                _ => {}
            }
            idx += 1;
        }

        // store
        let mut reordered = Vec::with_capacity(self.defs.len());
        for def in &self.defs {
            reordered.extend_from_slice(def.code)
        }

        reordered
    }
}

/// Shape/Reordering characters
/// The shaping engine inserts a placeholder glyph (U+25CC) wherever
/// combining marks occur without a valid base. The character U+25CC
/// belongs to the class of generic bases (GB). Well-formed Buginese
/// character clusters are defined as follows:
///
/// Cases:
/// 1) Consonant based syllables: Cons + {COENG + (Cons | IndV)} + [PreV | BlwV] + [RegShift] + [AbvV] + {AbvS} + [PstV] + [PstS]
///
/// Ex. រាជធានីភ្នំពេញ
pub fn shape_khmer(input: &mut [u16]) {
    let mut res: Vec<u16> = Vec::with_capacity(input.len());
    // Step 1: Convert input to clusters
    let defs = Definition::build_definition(input);
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
    fn khmer_test() {
        let input: &[u16] = &[0x1A00, 0x1A19, 0x034F, 0x1A19, 0x034F, 0x1A17];
        // TODO: Is this correct?
        // let expected: &[u16] = &[0x1A19, 0x034F, 0x1A19, 0x034F, 0x1A00, 0x1A17];
        let expected: &[u16] = &[0x1A00, 0x1A19, 0x034F, 0x1A19, 0x034F, 0x1A17];
        let mut result = input.to_vec();
        shape_khmer(&mut result);
        assert_eq!(result, expected);
    }

    #[test]
    fn khmer_2_test() {
        let input = "ᨔᨗᨔᨗᨊᨗᨊ";
        let expected: &[u16] = &[6676, 6679, 6676, 6679, 6666, 6679, 6666];
        // Encode the string as UTF-16 and obtain a slice of u16 values
        let input_utf16_slice: Vec<u16> = input.encode_utf16().collect();
        // Create a reference to the slice
        let input_utf16_ref: &[u16] = &input_utf16_slice;
        let mut result = input_utf16_ref.to_vec();
        shape_khmer(&mut result);
        assert_eq!(result, expected);
    }
}
