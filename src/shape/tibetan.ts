// https://learn.microsoft.com/en-us/typography/script-development/tibetan
// https://www.unicode.org/charts/PDF/U0F00.pdf
// https://r12a.github.io/scripts/tibt/bo.html

import { WHITESPACE } from '.';
import { buildClusters } from './shared';

import type { Cluster, Definition } from './shared';

/**
 * Check if a character is Tibetan
 * @param c - input unicode character
 * @returns - True if Tibetan
 */
export function isTibetan(c: number): boolean {
  return c >= 0x0f00 && c <= 0x0fff;
}

/** The type of a character in a Tibetan run */
enum MType {
  Lh, // Head letters (0F40–0F6C, 0F88–0F8C)
  // Ls, // Subjoined letters (0F8D–0F8F, 0F90–0FBC)
  Va, // Vowel marks: Above-base (0F72, 0F7A–D, 0F80)
  Vb, // Vowel marks: Below-base (0F71, 0F74)
  Vc /* Vowel marks: Compound vowels (0F73, 0F75–0F79, 0F81) [NOTE: Use of these characters is discouraged in favor of their decomposed equivalents.] */,
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

/**
 * Get the type of a Tibetan character
 * @param c - input unicode character
 * @returns - The type
 */
function toMType(c: number): MType {
  // note: isSpecialSequence is for K, if true, '103A, 1039' come after c
  // Head letters (0F40–0F6C, 0F88–0F8C)
  if ((c >= 0x0f40 && c <= 0x0f6c) || (c >= 0x0f88 && c <= 0x0f8c)) return MType.Lh;
  // Above-base (0F72, 0F7A–D, 0F80)
  if (c === 0x0f72 || (c >= 0x0f7a && c <= 0x0f7d) || c === 0x0f80) return MType.Va;
  // Below-base (0F71, 0F74)
  if (c === 0x0f71 || c === 0x0f74) return MType.Vb;
  // Compound vowels (0F73, 0F75–0F79, 0F81) [NOTE: Use of these characters is discouraged in favor of their decomposed equivalents.]
  if (c === 0x0f73 || (c >= 0x0f75 && c <= 0x0f79) || c === 0x0f81) return MType.Vc;
  if (WHITESPACE.includes(c)) return MType.WS;
  return MType.U;
}

/**
 * Convert input to clusters
 * @param input - input unicode buffer
 * @returns - array of clusters
 */
function buildFromUnicodes(input: number[]): Definition<MType>[] {
  const clusters: Definition<MType>[] = [];

  let idx = 0;
  while (idx < input.length) {
    const code = input[idx];
    clusters.push({ mType: toMType(code), code: [code] });
    idx++;
  }

  return clusters;
}

/**
 * The correct coding order for a stream of text is as follows:
 *
 * - head position consonant
 * - first sub-joined consonant
 * - ....intermediate sub-joined consonants (if any)
 * - last sub-joined consonant
 * - sub-joined vowel (a-chung U+0F71)
 * - standard or compound vowel sign (including virama U+0F84 in the case of Sanskrit transliteration)
 * - additional vowel signs (if any)
 * - vowel modifier signs (rjes su nga ro U+0F7E, rnam bcad U+0F7F)
 * @param cluster - cluster of definitions
 * @returns - reordered/sorted cluster
 */
function getSorted<T>(cluster: Cluster<T>): number[] {
  // sort
  let idx = 0;
  while (idx < cluster.defs.length) {
    const { mType } = cluster.defs[idx];
    if (mType === MType.Va || mType === MType.Vb || mType === MType.Vc) {
      // always put the head position consonant infront of the head letter
      let headIDX = idx;
      while (headIDX > 0 && cluster.defs[headIDX].mType !== MType.Lh) {
        headIDX--;
      }
      const vowelMark = cluster.defs.splice(idx, 1);
      cluster.defs.splice(headIDX, 0, vowelMark[0]);
    }
    idx++;
  }

  // store
  const reordered: number[] = [];
  for (const def of cluster.defs) reordered.push(...def.code);

  return reordered;
}

/**
 * Shape/Reordering characters
 * Once the Myanmar shaping engine has analyzed the run as described above,
 * it creates a buffer of appropriately reordered elements (glyphs) representing the
 * cluster according to the rules given:
 *
 * 1) Kinzi sequences (K) are reordered directly after the cluster base
 * 2) The medial ra (MR) is reordered before the base consonant
 * 3) Pre-base vowels (VPre) are reordered to the start of the syllable cluster.
 *    A sequence of multiple prebase vowels is permitted. Such sequences are moved
 *    as a block to the beginning of the cluster.
 * 4) Anusvara (A) coming immediately after one or more below-base vowels (VBlw)
 *    will reorder immediately before them.
 *
 * Cases:
 * 1) Letters: Lh [Ls*] <[Va*] | [Vb] | [Vc] > [Ml]
 * 2) Digits: D [Md]
 *
 * Ex. བོད་རང་སྐྱོང་ལྗོངས།
 * @param input - input unicode buffer to be modified/shaped
 */
export function shapeTibetan(input: number[]): void {
  const res: number[] = [];
  // Step 1: Convert input to clusters
  const defs = buildFromUnicodes(input);
  // Step 2: Split clusters by WS (white space)
  const clustersSets = buildClusters(defs, (t) => t === MType.WS);
  // Step 3: Reorder the clusters and add them to result
  clustersSets.forEach((c) => {
    res.push(...getSorted(c));
    // append whitespace of cluster if it exists
    if (c.whitespace !== undefined) res.push(c.whitespace);
  });

  // now map the result to the original input
  for (let i = 0; i < input.length; i++) input[i] = res[i];
}
