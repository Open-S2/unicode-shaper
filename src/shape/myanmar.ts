// https://learn.microsoft.com/en-us/typography/script-development/myanmar
// https://www.unicode.org/charts/PDF/U1000.pdf
// https://www.unicode.org/notes/tn11/UTN11_4.pdf
// https://r12a.github.io/scripts/mymr/my.html
// https://r12a.github.io/scripts/mymr/shn.html

import { WHITESPACE } from '.';
import { buildClusters } from './shared';

import type { Cluster, Definition } from './shared';

/**
 * Check if a character is a "Myanmar" unicode character
 * @param c - input unicode character
 * @returns - True if Myanmar
 */
export function isMyanmar(c: number): boolean {
  return (
    // main
    (c >= 0x1000 && c <= 0x109f) ||
    // extended A
    (c >= 0xaa60 && c <= 0xaa7f) ||
    // extended B
    (c >= 0xa9e0 && c <= 0xa9ff)
  );
}

/** The type of a character in a Myanmar run */
enum MType {
  A, // Anusvara class (1032, 1036)
  // As, // Asat (103A)
  C /* Consonants and Independent vowels (1000-1020, 103F, 104E, 1050, 1051, 105A-105D, 1061, 1065, 1066, 106E-1070, 1075-1081, 108E, AA60-AA6F, AA71-AA76, AA7A) */,
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

/**
 * Find the type of a Myanmar character
 * @param c - input unicode character
 * @param maybeKinziSequence - true if the character is part of a Kinzi sequence
 * @returns - The type of the character
 */
function toMType(c: number, maybeKinziSequence: boolean): MType {
  if (maybeKinziSequence) {
    if (c === 0x1004 || c === 0x101b || c === 0x105a) return MType.K;
    return toMType(c, false);
  }
  // Anusvara class (1032, 1036)
  if (c === 0x1032 || c === 0x1036) return MType.A;
  // Consonants and Independent vowels (1000-1020, 103F, 104E, 1050, 1051, 105A-105D, 1061, 1065, 1066, 106E-1070, 1075-1081, 108E, AA60-AA6F, AA71-AA76, AA7A)
  if (
    (c >= 0x1000 && c <= 0x1020) ||
    c === 0x103f ||
    c === 0x104e ||
    c === 0x1050 ||
    c === 0x1051 ||
    (c >= 0x105a && c <= 0x105d) ||
    c === 0x1061 ||
    c === 0x1065 ||
    c === 0x1066 ||
    (c >= 0x106e && c <= 0x1070) ||
    (c >= 0x1075 && c <= 0x1081) ||
    c === 0x108e ||
    (c >= 0xaa60 && c <= 0xaa6f) ||
    (c >= 0xaa71 && c <= 0xaa76) ||
    c === 0xaa7a
  )
    return MType.C;
  // Medial consonants Ra (103C)
  if (c === 0x103c) return MType.MR;
  // Below base dependent vowel (102F, 1030, 1058, 1059)
  if (c === 0x102f || c === 0x1030 || c === 0x1058 || c === 0x1059) return MType.VBlw;
  // Pre base dependent vowel (1031, 1084)
  if (c === 0x1031 || c === 0x1084) return MType.VPre;
  // Whitespace (0020, 0009, 000A, 000D, 000C, 0085, 3000, 200B)
  if (WHITESPACE.includes(c)) return MType.WS;
  return MType.O;
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
    const maybeKinziSequence: boolean =
      idx + 2 < input.length && input[idx + 1] === 0x103a && input[idx + 2] === 0x1039;
    clusters.push({ mType: toMType(code, maybeKinziSequence), code: [code] });
    if (maybeKinziSequence) {
      idx += 3;
    } else {
      idx++;
    }
  }

  return clusters;
}

/**
 * 1) Kinzi sequences (K) are reordered directly after the cluster base
 * 2) The medial ra (MR) is reordered before the base consonant
 * 3) Pre-base vowels (VPre) are reordered to the start of the syllable cluster.
 *    A sequence of multiple prebase vowels is permitted. Such sequences are moved
 *    as a block to the beginning of the cluster.
 * 4) Anusvara (A) coming immediately after one or more below-base vowels (VBlw)
 *    will reorder immediately before them.
 * @param cluster - cluster of definitions
 * @returns - reordered/sorted cluster
 */
function getSorted<T>(cluster: Cluster<T>): number[] {
  // sort
  let idx = 0;
  while (idx < cluster.defs.length) {
    const { mType } = cluster.defs[idx];
    if (mType === MType.K) {
      // Kinzi sequences (K) are reordered directly after the cluster base.
      // K always precedes the base consonant
      if (idx + 1 < cluster.defs.length) {
        [cluster.defs[idx], cluster.defs[idx + 1]] = [cluster.defs[idx + 1], cluster.defs[idx]];
        idx += 1;
      }
    }
    if (mType === MType.MR) {
      // The medial ra (MR) is reordered before the base consonant
      let baseCIdx = 0;
      while (baseCIdx + 1 < cluster.defs.length && cluster.defs[baseCIdx].mType !== MType.C) {
        baseCIdx++;
      }
      if (baseCIdx !== idx) {
        const vPre = cluster.defs.splice(idx, 1);
        cluster.defs.splice(baseCIdx, 0, vPre[0]);
      }
    }
    if (mType === MType.VPre) {
      // Pre-base vowels (VPre) are reordered to the start of the syllable cluster.
      const vPre = cluster.defs.splice(idx, 1);
      cluster.defs.unshift(vPre[0]);
    }
    if (mType === MType.A) {
      // Anusvara (A) coming immediately after one or more below-base vowels (VBlw)
      let prevIdx = idx;
      while (prevIdx - 1 > 0 && cluster.defs[prevIdx - 1].mType === MType.VBlw) {
        prevIdx--;
      }
      if (prevIdx !== idx) {
        [cluster.defs[prevIdx], cluster.defs[idx]] = [cluster.defs[idx], cluster.defs[prevIdx]];
      }
    }
    idx++;
  }

  // store
  const reordered: number[] = [];
  for (const def of cluster.defs) {
    if (def.mType === MType.K) {
      reordered.push(...def.code, 0x103a, 0x1039);
    } else {
      reordered.push(...def.code);
    }
  }

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
 * 1) Simple non-compounding cluster:   <P | S | R | WJ| WS | O | D0 >
 * 2) MType terminating in Halant: [K] <C | IV | D | GB>[VS] (H <C | IV> [VS])* H
 * 3) Complex cluster:                  [K] <C | IV | D | GB>[VS] (H <C | IV> [VS]) (As) [MY [As]] [MR] [<MW [As] | [MW] MH [As]>] (VPre) (VAbv)* (VBlw) (A) [DB [As]] (VPst [MH] (As)* (VAbv)* (A)* [DB [As]]) (PT < [A] [DB] [As] | [As] [A] > ) (V)* [J]
 *
 * Ex. င်္က္ကျြွှေို့်ာှီ့ၤဲံ့းႍ
 * INPUT - 1004 103A 1039 1000 1039 1000 103B 103C 103D 1031 1031 102D 102F 1036 102C 1036
 * I-EXPLAINED - ([K] 1004 103A 1039) - ([C] 1000) - ([H] 1039) - ([C] 1000) - ([MY] 103B) - ([MR] 103C) - ([MW] 103D) - ([VPre] 1031) - ([VPre] 1031) - ([VAbv] 102D) - ([VBlw] 102F) - ([A] 1036) - ([VPst] 102C) - ([A] 1036)
 * REORDERED - 1031 1031 103C 1000 1004 103A 1039 1039 1000 103B 103D 102D 1036 102F 102C 1036
 * R-EXPLAINED - ([VPre] 1031) - ([VPre] 1031) - ([MR] 103C) - ([C] 1000) - ([K] 1004 103A 1039) - ([H] 1039) - ([C] 1000) - ([MY] 103B) - ([MW] 103D) - ([VAbv] 102D) - ([A] 1036) - ([VPst] 102F) - ([VPst] 102C) - ([A] 1036)
 * @param input - input unicode buffer to be modified/shaped
 */
export function shapeMyanmar(input: number[]): void {
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
