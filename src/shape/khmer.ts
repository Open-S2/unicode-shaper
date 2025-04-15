// https://www.unicode.org/charts/PDF/U1780.pdf
// https://r12a.github.io/scripts/khmr/km.html

import { WHITESPACE } from './index.js';
import { buildClusters, buildDefinitions, commonGetSorted } from './shared.js';

import type { Cluster } from './shared.js';

/**
 * Check if a character is a "Khmer" unicode character
 * @param c - input unicode character
 * @returns - True if Khmer
 */
export function isKhmer(c: number): boolean {
  return c >= 0x1780 && c <= 0x17ff;
}

/** The type of a character in a Khmer run */
enum MType {
  Cs1 /* Consonant - SubscriptType1 (U+1780-U+1782, U+1784-U+1787, U+1789-U+178C, U+178E-U+1793, U+1795-U+1798, U+179B-U+179D, U+17A0, U+17A2) */,
  Cs2 /* Consonant - SubscriptType2 (U+179A, U+1783, U+1788, U+178D, U+1794, U+1799, U+179E-U+179F, U+17A1) */,
  V, // Independent Vowel (U+17B4-U+17B5)
  Vs1, // Idependent Vowel - SubscriptType1 (U+17A3-U+17B3)
  VAbv, // Above base vowel (U+17B7-U+17BA, U+17BE (split))
  VBlw, // Below base vowel (U+17BB-U+17BD)
  VPre, // Pre base vowel (U+17C1-U+17C3)
  VPst, // Post base vowel (U+17B6, U+17BF-U+17C0 (split), U+17C4-U+17C5 (split))
  Coeng, // U+17D2
  RS, // Register Shifter (U+17C9-U+17CA)
  Robat, // U+17CC
  SAbv, // Above base Sign (U+17C6, U+17CB, U+17CD-U+17D1, U+17DD)
  SPst, // Post base Sign (U+17C7-U+17C8)
  SAbvN, // Above base Sign for numbers (U+17D3)
  P, // Punctuation (U+17D4-U+17DA, U+17DC, U+19E0-U+19FF)
  C, // Currency (U+17DB)
  N, // Number (U+17E0-U+17E9, U+17F0-U+17F9)
  R, // Reserved (U+17DE-U+17DF, U+17EA-U+17EF, U+17FA-U+17FF)
  J, // Joiners (200D ZWJ (Zero Width Joiner) & 034F CGJ (COMBINING GRAPHEME JOINER))
  VS, // Variation selectors (FE00–FE0F)
  WJ, // Word joiner (2060)
  NJ, // Non-joiner (200C) [Zero Width Non-Joiner]
  WS, // Whitespace (0020, 0009, 000A, 000D, 000C, 0085, 3000, 200B)
  O, // other characters with no relation to khmer
}

/**
 * Find the type of a Khmer character
 * @param c - input unicode character
 * @returns - The type of the character
 */
function toMType(c: number): MType {
  // Consonant - SubscriptType1 (U+1780-U+1782, U+1784-U+1787, U+1789-U+178C, U+178E-U+1793, U+1795-U+1798, U+179B-U+179D, U+17A0, U+17A2)
  //             0x1780..=0x1782
  //             | 0x1784..=0x1787
  //             | 0x1789..=0x178C
  //             | 0x178E..=0x1793
  //             | 0x1795..=0x1798
  //             | 0x179B..=0x179D
  //             | 0x17A0
  //             | 0x17A2 => MType::Cs1,
  if (
    (c >= 0x1780 && c <= 0x17a82) ||
    (c >= 0x1784 && c <= 0x1787) ||
    (c >= 0x1789 && c <= 0x178c) ||
    (c >= 0x178e && c <= 0x1793) ||
    (c >= 0x1795 && c <= 0x1798) ||
    (c >= 0x179b && c <= 0x179d) ||
    c === 0x17a0 ||
    c === 0x17a2
  )
    return MType.Cs1;
  if (
    c === 0x179a ||
    c === 0x1783 ||
    c === 0x1788 ||
    c === 0x178d ||
    c === 0x1794 ||
    c === 0x1799 ||
    (c >= 0x179e && c <= 0x179f) ||
    c === 0x17a1
  )
    return MType.Cs2;
  // Independent Vowel (U+17B4-U+17B5)
  if (c >= 0x17b4 && c <= 0x17b5) return MType.V;
  // Idependent Vowel - SubscriptType1 (U+17A3-U+17B3)
  if (c >= 0x17a3 && c <= 0x17b3) return MType.Vs1;
  // Above base vowel (U+17B7-U+17BA, U+17BE (split))
  if ((c >= 0x17b7 && c <= 0x17ba) || c === 0x17be) return MType.VAbv;
  // Below base vowel (U+17BB-U+17BD)
  if (c >= 0x17bb && c <= 0x17bd) return MType.VBlw;
  // Pre base vowel (U+17C1-U+17C3)
  if (c >= 0x17c1 && c <= 0x17c3) return MType.VPre;
  // Post base vowel (U+17B6, U+17BF-U+17C0 (split), U+17C4-U+17C5 (split))
  if (c === 0x17b6 || (c >= 0x17bf && c <= 0x17c0) || (c >= 0x17c4 && c <= 0x17c5))
    return MType.VPst;
  // U+17D2
  if (c === 0x17d2) return MType.Coeng;
  // Register Shifter (U+17C9-U+17CA)
  if (c >= 0x17c9 && c <= 0x17ca) return MType.RS;
  // Robat (U+17CC)
  if (c === 0x17cc) return MType.Robat;
  // Above base Sign (U+17C6, U+17CB, U+17CD-U+17D1, U+17DD)
  if (c === 0x17c6 || c === 0x17cb || (c >= 0x17cd && c <= 0x17d1) || c === 0x17dd)
    return MType.SAbv;
  // Post base Sign (U+17C7-U+17C8)
  if (c === 0x17c7 || c === 0x17c8) return MType.SPst;
  // Above base Sign for numbers (U+17D3)
  if (c === 0x17d3) return MType.SAbvN;
  // Punctuation (U+17D4-U+17DA, U+17DC, U+19E0-U+19FF)
  if ((c >= 0x17d4 && c <= 0x17da) || c === 0x17dc || (c >= 0x19e0 && c <= 0x19ff)) return MType.P;
  // Currency (U+17DB)
  if (c === 0x17db) return MType.C;
  // Number (U+17E0-U+17E9, U+17F0-U+17F9)
  if ((c >= 0x17e0 && c <= 0x17e9) || (c >= 0x17f0 && c <= 0x17f9)) return MType.N;
  // Reserved (U+17DE-U+17DF, U+17EA-U+17EF, U+17FA-U+17FF)
  if (c === 0x17de || c === 0x17df || (c >= 0x17ea && c <= 0x17ef) || (c >= 0x17fa && c <= 0x17ff))
    return MType.R;
  // Joiners (200C, 200D)
  if (c === 0x200d || c === 0x034f) return MType.J;
  // Variation selectors (FE00–FE0F)
  if (c >= 0xfe00 && c <= 0xfe0f) return MType.VS;
  // Word joiner (2060)
  if (c === 0x2060) return MType.WJ;
  // Non-joiner (200C) [Zero Width Non-Joiner]
  if (c === 0x200c) return MType.NJ;
  // Whitespace (0020, 0009, 000A, 000D, 000C, 0085, 3000, 200B)
  if (WHITESPACE.includes(c)) return MType.WS;
  return MType.O;
}

/**
 * Once the Khmer shaping engine has analyzed the run into
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
 * The shaping engine inserts a placeholder glyph (U+25CC) wherever
 * combining marks occur without a valid base. The character U+25CC
 * belongs to the class of generic bases (GB). Well-formed Khmer
 * character clusters are defined as follows:
 *
 * Cases:
 * 1) Consonant based syllables: Cons + {COENG + (Cons | IndV)} + [PreV | BlwV] + [RegShift] + [AbvV] + {AbvS} + [PstV] + [PstS]
 *
 * Ex. រាជធានីភ្នំពេញ
 * @param input - input unicode buffer to shape in place
 */
export function shapeKhmer(input: number[]): void {
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
