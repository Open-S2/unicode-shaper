// https://www.unicode.org/charts/PDF/U1A00.pdf

use alloc::vec::Vec;

pub fn is_buginese(c: &u16) -> bool {
    // main 1A00–1A1F
    *c >= 0x1A00 && *c <= 0x1A1F
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
    0x202F | // NARROW NO-BREAK SPACE
    0x2060 | // WORD JOINER
    0xFEFF; // ZERO WIDTH NO-BREAK SPACE

#[derive(Debug, Clone, PartialEq)]
enum MType {
    C, // Consonants (1A00-1A16)
    GB, // Generic base characters (00A0, 00D7, 2012–2015, 2022, 25CC, 25FB–25FE)
    J, // Joiners (200D ZWJ (Zero Width Joiner) & 034F CGJ (COMBINING GRAPHEME JOINER))
    O, // SCRIPT_COMMON characters in a Buginese run
    R, // Reserved characters from the Buginese block (1A1C, 1A1D)
    S, // Symbols (1A1E, 1A1F, A9CF)
    VAbv, // Above base dependent vowel (1A17, 1A1B)
    VBlw, // Below base dependent vowel (1A18)
    VPre, // Pre base dependent vowel (1A19)
    VPst, // Post base dependent vowel (1A1A)
    VS, // Variation selectors (FE00–FE0F)
    WJ, // Word joiner (2060)
    NJ, // Non-joiner (200C) [Zero Width Non-Joiner]
    WS, // Whitespace (0020, 0009, 000A, 000D, 000C, 0085, 3000, 200B)
}
impl MType {
    // note: isSpecialSequence is for K, if true, '103A, 1039' come after c
    fn from_u16(c: &u16) -> MType {
        match c {
            // Consonants (1A00-1A16)
            0x1A00..=0x1A16 => MType::C,
            // Generic base characters (00A0, 00D7, 2012–2015, 2022, 25CC, 25FB–25FE)
            0x00A0 | 0x00D7 | 0x2012..=0x2015 | 0x2022 | 0x25CC | 0x25FB..=0x25FE => MType::GB,
            // Joiners (200C, 200D)
            0x200D | 0x034F => MType::J,
            // Reserved characters from the Buginese block (1A1C, 1A1D)
            0x1A1C | 0x1A1D => MType::R,
            // Symbols (1A1E, 1A1F, A9CF)
            0x1A1E | 0x1A1F | 0xA9CF => MType::S,
            // Above base dependent vowel (1A17, 1A1B)
            0x1A17 | 0x1A1B => MType::VAbv,
            // Below base dependent vowel (1A18)
            0x1A18 => MType::VBlw,
            // Pre base dependent vowel (1A19)
            0x1A19 => MType::VPre,
            // Post base dependent vowel (1A1A)
            0x1A1A => MType::VPst,
            // Variation selectors (FE00–FE0F)
            0xFE00..=0xFE0F => MType::VS,
            // Word joiner (2060)
            0x2060 => MType::WJ,
            // Whitespace (0020, 0009, 000A, 000D, 000C, 0085, 3000, 200B)
            &WHITESPACE => MType::WS,
            _ => MType::O,
        }
    }

    fn is_same (&self, other: &u16) -> bool {
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
            clusters.push(
                Definition::new(m_type, &input[start_idx..end_idx])
            );
            idx = end_idx;
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
                },
                MType::VAbv | MType::VBlw | MType::VPst => {
                    // always put the head position consonant infront of the head letter
                    let mut head_idx = idx;
                    while head_idx > 0 && self.defs[head_idx].m_type != MType::C && self.defs[head_idx].m_type != MType::GB { head_idx -= 1; }
                    let vowel_sign = self.defs.remove(idx);
                    self.defs.insert(head_idx, vowel_sign);
                },
                _ => {},
            }
            idx += 1;
        }

        // store
        let mut reordered = Vec::with_capacity(self.defs.len());
        for def in &self.defs { reordered.extend_from_slice(def.code) }

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
/// 1) Simple non-compounding cluster: < S | Rsv | WS | O | J | WJ >
/// 2) Clusters:                       < C | GB > [VS] (VPre)* (VAbv)* (VBlv)* (VPst)* [J]
///
/// Ex. ᨔᨗᨔᨗᨊᨗᨊ
pub fn shape_buginese(input: &mut [u16]) {
    let mut res: Vec<u16> = Vec::with_capacity(input.len());
    // Step 1: Convert input to clusters
    let defs = Definition::build_definition(input);
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
    fn buginese_test() {
        let input: &[u16] = &[0x1A00, 0x1A19, 0x034F, 0x1A19, 0x034F, 0x1A17];
        let expected: &[u16] = &[0x1A19, 0x034F, 0x1A19, 0x034F, 0x1A17, 0x1A00];
        let mut result = input.to_vec();
        shape_buginese(&mut result);
        assert_eq!(result, expected);
    }

    #[test]
    fn buginese_2_test() {
        let input = "ᨔᨗᨔᨗᨊᨗᨊ";
        let expected: &[u16] = &[6679, 6676, 6679, 6676, 6679, 6666, 6666];
        // Encode the string as UTF-16 and obtain a slice of u16 values
        let input_utf16_slice: Vec<u16> = input.encode_utf16().collect();
        // Create a reference to the slice
        let input_utf16_ref: &[u16] = &input_utf16_slice;
        let mut result = input_utf16_ref.to_vec();
        shape_buginese(&mut result);
        assert_eq!(result, expected);
    }

    #[test]
    fn buginese_3_test() {
        let input = "ᨑᨗ ᨍᨍᨗᨕᨂᨗ";
        let expected: &[u16] = &[6679, 6673, 32, 6679, 6669, 6669, 6679, 6677, 6658];
        // Encode the string as UTF-16 and obtain a slice of u16 values
        let input_utf16_slice: Vec<u16> = input.encode_utf16().collect();
        // Create a reference to the slice
        let input_utf16_ref: &[u16] = &input_utf16_slice;
        let mut result = input_utf16_ref.to_vec();
        shape_buginese(&mut result);
        assert_eq!(result, expected);
    }
}
