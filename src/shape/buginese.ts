// https://www.unicode.org/charts/PDF/U1A00.pdf
// https://r12a.github.io/scripts/bugi/bug.html

import { WHITESPACE } from './index.js';
import { buildClusters, buildDefinitions, commonGetSorted } from './shared.js';

import type { Cluster } from './shared.js';

/**
 * Check if a character is a "buginese" unicode character
 * @param c - input unicode character
 * @returns True if buginese
 */
export function isBuginese(c: number): boolean {
  return c >= 0x1a00 && c <= 0x1a1f;
}

/** The type of a character in a Buginese run */
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

/**
 * Find the type of a buginese character
 * NOTE: isSpecialSequence is for K, if true, '103A, 1039' come after c
 * @param c - input unicode character
 * @returns The type of the character
 */
function toMType(c: number): MType {
  // match c {
  // Consonants (1A00-1A16)
  if (c >= 0x1a00 && c <= 0x1a16) return MType.C;
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
  // Joiners (200C, 200D)
  if (c === 0x200d || c === 0x034f) return MType.J;
  // Reserved characters from the Buginese block (1A1C, 1A1D)
  if (c === 0x1a1c || c === 0x1a1d) return MType.R;
  // Symbols (1A1E, 1A1F, A9CF)
  if (c === 0x1a1e || c === 0x1a1f || c === 0xa9cf) return MType.S;
  // Above base dependent vowel (1A17, 1A1B)
  if (c === 0x1a17 || c === 0x1a1b) return MType.VAbv;
  // Below base dependent vowel (1A18)
  if (c === 0x1a18) return MType.VBlw;
  // Pre base dependent vowel (1A19)
  if (c === 0x1a19) return MType.VPre;
  // Post base dependent vowel (1A1A)
  if (c === 0x1a1a) return MType.VPst;
  // Variation selectors (FE00–FE0F)
  if (c >= 0xfe00 && c <= 0xfe0f) return MType.VS;
  // Word joiner (2060)
  if (c === 0x2060) return MType.WJ;
  // Non-joiner (200C) [Zero Width Non-Joiner]
  if (c === 0x200c) return MType.NJ;
  if (WHITESPACE.includes(c)) return MType.WS;
  return MType.O;
}

/**
 * Once the Buginese shaping engine has analyzed the run into
 * clusters as described above, it performs any required reordering.
 * Pre-base vowels (VPre) are reordered to the start of the syllable
 * cluster. A sequence of multiple pre-base vowels is permitted.
 * Such sequences are moved as a block to the beginning of the cluster.
 * In the following example, the run of code points represents a
 * single cluster.
 * @param cluster - cluster of definitions
 * @returns - Returns the cluster sorted by dominant type
 */
function getSorted<T>(cluster: Cluster<T>): number[] {
  return commonGetSorted(cluster as Cluster<number>, MType);
}

/**
 * Shape/Reordering characters
 *
 * @param input - array of unicode characters to be shaped in place if the input contains buginese
 *
 * The shaping engine inserts a placeholder glyph (U+25CC) wherever
 * combining marks occur without a valid base. The character U+25CC
 * belongs to the class of generic bases (GB). Well-formed Buginese
 * character clusters are defined as follows:
 *
 * Cases:
 * 1) Simple non-compounding cluster: < S | Rsv | WS | O | J | WJ >
 * 2) Clusters:                       < C | GB > [VS] (VPre)* (VAbv)* (VBlv)* (VPst)* [J]
 *
 * Ex. ᨔᨗᨔᨗᨊᨗᨊ
 */
export function shapeBuginese(input: number[]): void {
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
