/* eslint-disable no-irregular-whitespace */
// https://www.unicode.org/charts/PDF/UA980.pdf
// https://r12a.github.io/scripts/java/jv.html

import { WHITESPACE } from './index.js';
import { buildClusters, buildDefinitions, commonGetSorted } from './shared.js';

import type { Cluster } from './shared.js';

/**
 * Check if a character is a "Javanese" unicode character
 * @param c - input unicode character
 * @returns - True if Javanese
 */
export function isJavanese(c: number): boolean {
  return c >= 0xa980 && c <= 0xa9df;
}

/** The type of a character in a Javanese run */
enum MType {
  C, // Consonants (A984, A989–A98B, A98F–A9B2)
  GB, // Generic base characters (00A0, 00D7, 2012–2015, 2022, 25CC, 25FB–25FE)
  H, // Halant/virama (A9C0)
  IV, // Independent vowel (A985–A988, A98C–A98E)
  J, // Joiners (200D ZWJ (Zero Width Joiner) & 034F CGJ (COMBINING GRAPHEME JOINER))
  M, // Modifiers (A980–A983)
  MR, // Medial consonants Ra (A9BF)
  MY, // Medial consonant Ya (A9BE)
  N, // Nukta/Cecak Telu (A9B3)
  O, // SCRIPT_COMMON characters in a Javanese run
  P, // Punctuation (A9C1–A9CD)
  R, // Reserved characters from the Javanese block (A9CE, A9DA–A9DD)
  S, // Symbols (A9CF, A9DE, A9DF)
  VAbv, // Above base dependent vowel (A9B6, A9B7, A9BC)
  VBlw, // Below base dependent vowel (A9B8, A9B9)
  VPre, // Pre base dependent vowel (A9BA, A9BB)
  VPst, // Post base dependent vowel (A9B4, A9B5, A9BD)
  VS, // Variation selectors (FE00–FE0F)
  WJ, // Word joiner (2060)
  NJ, // Non-joiner (200C) [Zero Width Non-Joiner]
  WS, // Whitespace (0020, 0009, 000A, 000D, 000C, 0085, 3000, 200B)
}

/**
 * Find the type of a Javanese character
 * NOTE: isSpecialSequence is for K, if true, '103A, 1039' come after c
 * @param c - input unicode character
 * @returns - The type of the character
 */
function toMType(c: number): MType {
  // note: isSpecialSequence is for K, if true, '103A, 1039' come after c
  // Consonants (A984, A989–A98B, A98F–A9B2)
  if (c >= 0xa984 || (c >= 0xa989 && c <= 0xa98b) || (c >= 0xa98f && c <= 0xa9b2)) return MType.C;
  // Generic base characters (00A0, 00D7, 2012–2015, 2022, 25CC, 25FB–25FE)
  if (
    c === 0x00a0 ||
    c === 0x00d7 ||
    (c >= 0x2012 && c <= 0x2015) ||
    c === 0x2022 ||
    c === 0x25cc ||
    (c >= 0x25fb && c <= 0x25fe)
  )
    return MType.GB;
  // Halant/virama (A9C0)
  if (c === 0xa9c0) return MType.H;
  // Independent vowel (A985–A988, A98C–A98E)
  if ((c >= 0xa985 && c <= 0xa988) || (c >= 0xa98c && c <= 0xa98e)) return MType.IV;
  // Joiners (200D ZWJ (Zero Width Joiner) & 034F CGJ (COMBINING GRAPHEME JOINER))
  if (c === 0x200d || c === 0x034f) return MType.J;
  // Modifiers (A980–A983)
  if (c >= 0xa980 && c <= 0xa983) return MType.M;
  // Medial consonants Ra (A9BF)
  if (c === 0xa9bf) return MType.MR;
  // Medial consonant Ya (A9BE)
  if (c === 0xa9be) return MType.MY;
  // Nukta/Cecak Telu (A9B3)
  if (c === 0xa9b3) return MType.N;
  // Punctuation (A9C1–A9CD)
  if (c >= 0xa9c1 && c <= 0xa9cd) return MType.P;
  // Reserved characters from the Javanese block (A9CE, A9DA–A9DD)
  if (c === 0xa9ce || (c >= 0xa9da && c <= 0xa9dd)) return MType.R;
  // Symbols (A9CF, A9DE, A9DF)
  if (c === 0xa9cf || c === 0xa9de || c === 0xa9df) return MType.S;
  // Above base dependent vowel (A9B6, A9B7, A9BC)
  if (c === 0xa9b6 || c === 0xa9b7 || c === 0xa9bc) return MType.VAbv;
  // Below base dependent vowel (A9B8, A9B9)
  if (c === 0xa9b8 || c === 0xa9b9) return MType.VBlw;
  // Pre base dependent vowel (A9BA, A9BB)
  if (c === 0xa9ba || c === 0xa9bb) return MType.VPre;
  // Post base dependent vowel (A9B4, A9B5, A9BD)
  if (c === 0xa9b4 || c === 0xa9b5 || c === 0xa9bd) return MType.VPst;
  // Variation selectors (FE00–FE0F)
  if (c >= 0xfe00 && c <= 0xfe0f) return MType.VS;
  // Word joiner (2060)
  if (c === 0x2060) return MType.WJ;
  // Non-joiner (200C) [Zero Width Non-Joiner]
  if (c === 0x200c) return MType.NJ;
  // Whitespace (0020, 0009, 000A, 000D, 000C, 0085, 3000, 200B)
  if (WHITESPACE.includes(c)) return MType.WS;
  // Script common
  return MType.O;
}

/**
 * Once the Javanese shaping engine has analyzed the run into
 * clusters as described above, it performs any required reordering.
 * Pre-base vowels (VPre) are reordered to the start of the
 * syllable cluster. A sequence of multiple pre-base vowels is
 * permitted. Such sequences are moved as a block to the beginning
 * of the cluster. In the following example, the run of code points
 * represents a single cluster.
 * @param cluster - cluster of definitions
 * @returns - Returns the cluster sorted by dominant type
 */
function getSorted<T>(cluster: Cluster<T>): number[] {
  return commonGetSorted(cluster as Cluster<number>, MType);
}

/**
 * Shape/Reordering characters
 *
 * @param input - input unicode buffer to shape in place
 *
 * The shaping engine inserts a placeholder glyph (U+25CC) wherever
 * combining marks occur without a valid base. The character U+25CC
 * belongs to the class of generic bases (GB). Well-formed Javanese
 * character clusters are defined as follows:
 *
 * Cases:
 * 1) Simple non-compounding cluster: < IV | P | D | S | R | WS | O | WJ >
 * 2) Cluster terminating in Halant:  < C | GB > [VS] [N] (H C [VS] [N])* H
 * 3) Complex cluster:                < C | GB > [VS] [N] (H C [VS] [N]) [MCR] [MCY] (VPre) (VAbv) (VBlw) (M)*
 *
 * Ex. ꦧꦺꦲꦏ꧀ꦠꦸꦩꦿꦥ꧀ꦲ​
 */
export function shapeJavanese(input: number[]): void {
  const res: number[] = [];
  // Step 1: Convert input to clusters
  const defs = buildDefinitions(input, toMType);
  // Step 2: Split clusters by WS (white space)
  const clustersSets = buildClusters(defs, (mType) => mType === MType.WS || mType === MType.NJ);
  // Step 3: Reorder the clusters and add them to result
  clustersSets.forEach((c) => {
    res.push(...getSorted(c));
    // append whitespace of cluster if it exists
    if (c.whitespace !== undefined) res.push(c.whitespace);
  });

  // now map the result to the original input
  for (let i = 0; i < input.length; i++) input[i] = res[i];
}
