// https://www.unicode.org/charts/PDF/UA980.pdf
// https://r12a.github.io/scripts/java/jv.html

use crate::shape::*;
use alloc::vec::Vec;

/// Check if a character is Javanese
pub fn is_javanese(c: &u16) -> bool {
    // A980–A9DF
    *c >= 0xA980 && *c <= 0xA9DF
}

#[derive(Debug, Clone, PartialEq)]
enum MType {
    C,    // Consonants (A984, A989–A98B, A98F–A9B2)
    GB,   // Generic base characters (00A0, 00D7, 2012–2015, 2022, 25CC, 25FB–25FE)
    H,    // Halant/virama (A9C0)
    IV,   // Independent vowel (A985–A988, A98C–A98E)
    J,    // Joiners (200D ZWJ (Zero Width Joiner) & 034F CGJ (COMBINING GRAPHEME JOINER))
    M,    // Modifiers (A980–A983)
    MR,   // Medial consonants Ra (A9BF)
    MY,   // Medial consonant Ya (A9BE)
    N,    // Nukta/Cecak Telu (A9B3)
    O,    // SCRIPT_COMMON characters in a Javanese run
    P,    // Punctuation (A9C1–A9CD)
    R,    // Reserved characters from the Javanese block (A9CE, A9DA–A9DD)
    S,    // Symbols (A9CF, A9DE, A9DF)
    VAbv, // Above base dependent vowel (A9B6, A9B7, A9BC)
    VBlw, // Below base dependent vowel (A9B8, A9B9)
    VPre, // Pre base dependent vowel (A9BA, A9BB)
    VPst, // Post base dependent vowel (A9B4, A9B5, A9BD)
    VS,   // Variation selectors (FE00–FE0F)
    WJ,   // Word joiner (2060)
    NJ,   // Non-joiner (200C) [Zero Width Non-Joiner]
    WS,   // Whitespace (0020, 0009, 000A, 000D, 000C, 0085, 3000, 200B)
}
impl MType {
    // note: isSpecialSequence is for K, if true, '103A, 1039' come after c
    fn from_u16(c: &u16) -> MType {
        match c {
            // Consonants (A984, A989–A98B, A98F–A9B2)
            0xA984 | 0xA989..=0xA98B | 0xA98F..=0xA9B2 => MType::C,
            // Generic base characters (00A0, 00D7, 2012–2015, 2022, 25CC, 25FB–25FE)
            0x00A0 | 0x00D7 | 0x2012..=0x2015 | 0x2022 | 0x25CC | 0x25FB..=0x25FE => MType::GB,
            // Halant/virama (A9C0)
            0xA9C0 => MType::H,
            // Independent vowel (A985–A988, A98C–A98E)
            0xA985..=0xA988 | 0xA98C..=0xA98E => MType::IV,
            // Joiners (200C, 200D)
            0x200D | 0x034F => MType::J,
            // Modifiers (A980–A983)
            0xA980..=0xA983 => MType::M,
            // Medial consonants Ra (A9BF)
            0xA9BF => MType::MR,
            // Medial consonant Ya (A9BE)
            0xA9BE => MType::MY,
            // Nukta/Cecak Telu (A9B3)
            0xA9B3 => MType::N,
            // Punctuation (A9C1–A9CD)
            0xA9C1..=0xA9CD => MType::P,
            // Reserved characters from the Javanese block (A9CE, A9DA–A9DD)
            0xA9CE | 0xA9DA..=0xA9DD => MType::R,
            // Symbols (A9CF, A9DE, A9DF)
            0xA9CF | 0xA9DE | 0xA9DF => MType::S,
            // Above base dependent vowel (A9B6, A9B7, A9BC)
            0xA9B6 | 0xA9B7 | 0xA9BC => MType::VAbv,
            // Below base dependent vowel (A9B8, A9B9)
            0xA9B8 | 0xA9B9 => MType::VBlw,
            // Pre base dependent vowel (A9BA, A9BB)
            0xA9BA | 0xA9BB => MType::VPre,
            // Post base dependent vowel (A9B4, A9B5, A9BD)
            0xA9B4 | 0xA9B5 | 0xA9BD => MType::VPst,
            // Variation selectors (FE00–FE0F)
            0xFE00..=0xFE0F => MType::VS,
            // Word joiner (2060)
            0x2060 => MType::WJ,
            // Non-joiner (200C) [Zero Width Non-Joiner]
            0x200C => MType::NJ,
            // Whitespace (0020, 0009, 000A, 000D, 000C, 0085, 3000, 200B)
            &WHITESPACE => MType::WS,
            // Script common
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

    /// Once the Javanese shaping engine has analyzed the run into
    /// clusters as described above, it performs any required reordering.
    /// Pre-base vowels (VPre) are reordered to the start of the
    /// syllable cluster. A sequence of multiple pre-base vowels is
    /// permitted. Such sequences are moved as a block to the beginning
    /// of the cluster. In the following example, the run of code points
    /// represents a single cluster.
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
                    // always put the head position consonant infront of the head letter
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
/// 1) Simple non-compounding cluster: < IV | P | D | S | R | WS | O | WJ >
/// 2) Cluster terminating in Halant:  < C | GB > [VS] [N] (H C [VS] [N])* H
/// 3) Complex cluster:                < C | GB > [VS] [N] (H C [VS] [N]) [MCR] [MCY] (VPre) (VAbv) (VBlw) (M)*
///
/// Ex. ꦧꦺꦲꦏ꧀ꦠꦸꦩꦿꦥ꧀ꦲ​
pub fn shape_javanese(input: &mut [u16]) {
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
    fn javanese_test() {
        let input: &[u16] = &[0xA98F, 0xA9C0, 0xA98F, 0xA9BF, 0xA9BE, 0xA9BA, 0xA9BA, 0xA9B7];
        let expected: &[u16] = &[0xA9BA, 0xA9BA, 0xA98F, 0xA9C0, 0xA98F, 0xA9BF, 0xA9BE, 0xA9B7];
        let mut result = input.to_vec();
        shape_javanese(&mut result);
        assert_eq!(result, expected);
    }

    #[test]
    fn javanese_2_test() {
        let input = "ꦧꦺꦲꦏ꧀ꦠꦸꦩꦿꦥ꧀ꦲ";
        let expected: &[u16] =
            &[43450, 43431, 43442, 43407, 43456, 43424, 43448, 43433, 43455, 43429, 43456, 43442];
        // Encode the string as UTF-16 and obtain a slice of u16 values
        let input_utf16_slice: Vec<u16> = input.encode_utf16().collect();
        // Create a reference to the slice
        let input_utf16_ref: &[u16] = &input_utf16_slice;
        let mut result = input_utf16_ref.to_vec();
        shape_javanese(&mut result);
        assert_eq!(result, expected);
    }
}
