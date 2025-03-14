/** A Definition tracks a cluster and its type */
export interface Definition<T> {
  /** cluster definition */
  mType: T;
  /** reference to a slice of an original string */
  code: number[];
}

/** Cluster of definitions */
export interface Cluster<T> {
  /** array of definitions */
  defs: Definition<T>[];
  /** unicode whitespace (optional) */
  whitespace?: number;
}

/**
 * Shared shaper - swaps characters
 * @param input - input unicode buffer
 * @param comparitor - characters to swap
 */
export function sharedShaper(input: number[], comparitor: number[]): void {
  for (let i = 0; i < input.length; i++) {
    if (i === 0) continue;
    if (comparitor.includes(input[i])) {
      [input[i - 1], input[i]] = [input[i], input[i - 1]];
    }
  }
}

/**
 * Build clusters
 * @param defs - array of definitions
 * @param cmp - comparator
 * @returns - array of clusters
 */
export function buildClusters<T>(defs: Definition<T>[], cmp: (t: T) => boolean): Cluster<T>[] {
  const clusters: Cluster<T>[] = [];

  let defIDX = 0;
  for (let idx = 0; idx < defs.length; idx++) {
    if (cmp(defs[idx].mType)) {
      clusters.push({ defs: defs.slice(defIDX, idx), whitespace: defs[idx].code[0] });
      defIDX = idx + 1;
    }
  }
  // store last
  if (defIDX < defs.length) clusters.push({ defs: defs.slice(defIDX) });

  return clusters;
}

/**
 * Check if two types are the same or if the second type is J
 * @param a - first type
 * @param b - second type as a unicode
 * @param toMType - function to convert unicode to type
 * @returns - true if both types are the same
 */
export function mTypeIsSame<T>(a: T, b: number, toMType: (unicode: number) => T): boolean {
  // if b is joiner, return true
  if (b === 0x200d || b === 0x034f) return true;
  // otherwise compare the two
  return a === toMType(b);
}

/**
 * Convert input to clusters of definitions
 * @param input - input unicode buffer
 * @param toMType - function to convert unicode to type
 * @returns - array of clusters
 */
export function buildDefinitions<T>(
  input: number[],
  toMType: (unicode: number) => T,
): Definition<T>[] {
  const clusters: Definition<T>[] = [];

  let idx = 0;
  while (idx < input.length) {
    const startIDX = idx;
    const code = input[idx];
    const mType = toMType(code);
    let endIDX = idx + 1;
    // while we find joiners and the same type, continue
    while (endIDX < input.length && mTypeIsSame(mType, input[endIDX], toMType)) {
      endIDX++;
    }
    clusters.push({ mType, code: input.slice(startIDX, endIDX) });
    idx = endIDX;
  }

  return clusters;
}

/** BaseMType for shared shaper */
interface BaseMType {
  VPre: number;
  VAbv: number;
  VBlw: number;
  VPst: number;
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
 * @param enumType - the enum to use
 * @returns - Returns the cluster sorted by dominant type
 */
export function commonGetSorted(cluster: Cluster<number>, enumType: BaseMType): number[] {
  // sort
  let idx = 0;
  while (idx < cluster.defs.length) {
    const { mType } = cluster.defs[idx];
    if (mType === enumType.VPre) {
      // Pre-base vowels (VPre) are reordered to the start of the syllable cluster.
      const vPre = cluster.defs.splice(idx, 1);
      cluster.defs.unshift(vPre[0]);
    } else if (mType === enumType.VAbv || mType === enumType.VBlw || mType === enumType.VPst) {
      // TODO: always put the head position consonant infront of the head letter
      // let headIDX = idx;
      // while (
      //   headIDX > 0 &&
      //   cluster.defs[headIDX].mType !== MType.C &&
      //   cluster.defs[headIDX].mType !== MType.GB
      // ) {
      //   headIDX--;
      // }
      // const vowelSign = cluster.defs.splice(idx, 1);
      // cluster.defs.splice(headIDX, 0, vowelSign[0]);
    }
    idx++;
  }

  // store
  const reordered: number[] = [];
  for (const def of cluster.defs) reordered.push(...def.code);

  return reordered;
}
