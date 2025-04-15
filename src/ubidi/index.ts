import { Type, findDominantType, getType, mirrorAdjustString } from './internal.js';

export * from './internal.js';

/** Basic Line tracker */
interface Line {
  start: number;
  end: number;
}

/** Chunk of unicode text to parse */
class Chunk {
  /**
   * @param start - begin section
   * @param end - end section
   * @param type - the type
   */
  constructor(
    public start: number,
    public end: number,
    public type: Type,
  ) {}

  /**
   * Check if an input type is the same as this Chunk
   * @param t - input type
   * @returns - true if both types are the same
   */
  isType(t: Type): boolean {
    return this.type === t;
  }

  /**
   * Set the current type
   * @param t - type to set
   */
  setType(t: Type): void {
    this.type = t;
  }

  /**
   * Set the type given a chunk if it exists
   * @param c - input chunk
   */
  setChunkType(c?: Chunk): void {
    if (c !== undefined) this.type = c.type;
  }
}

/**
 * Process a string of text and return a new string with the correct bidi ordering.
 * Follows https://www.unicode.org/reports/tr9/#Basic_Display_Algorithm as closely as possible.
 * Some things are not implemented, such as:
 * - Explicit embedding levels
 * - Explicit bracket control
 * @param input - input unicode buffer
 * @returns - output unicode buffer that's been reordered
 */
export function processBidiText(input: number[]): number[] {
  const inputLen = input.length;
  // print input:
  const result: number[] = [];
  // lines are storing [start, end] positions
  const lines: Line[] = [];
  // step 1: split by lines
  let start = 0;
  let end = 0;
  // for (idx, c) in input.iter().enumerate() {
  for (let idx = 0; idx < inputLen; idx++) {
    const c = input[idx];
    if (c === 0x000a || c === 0x000d) {
      if (start === end) {
        start = idx + 1;
      } else {
        lines.push({ start, end: idx });
        start = idx + 1;
      }
    }
    end = idx + 1;
  }
  // store the last line
  if (start < inputLen) {
    lines.push({ start, end: inputLen });
  }
  // step 2: iterate lines
  // for (lineIDX, line) in lines.iter().enumerate() {
  for (let lineIDX = 0; lineIDX < lines.length; lineIDX++) {
    const line = lines[lineIDX];
    const lineStr = input.slice(line.start, line.end);
    const wordChunks: Chunk[] = [];
    // s2.1: define dominant type
    let curType: Type = getType(lineStr[0]);
    const dominantRTL: boolean =
      curType !== Type.Neutral && curType !== Type.Weak
        ? curType === Type.Rtl
        : findDominantType(lineStr) === Type.Rtl;
    if (curType === Type.Neutral || curType === Type.Weak) {
      if (dominantRTL) {
        curType = Type.Rtl;
      } else {
        curType = Type.Ltr;
      }
    }
    // s2.2: group by RTL and LTR chunks
    start = 0;
    for (let idx = 0; idx < lineStr.length; idx++) {
      const uCode = lineStr[idx];
      const uType = getType(uCode);
      if (uType !== curType) {
        wordChunks.push(new Chunk(start, idx, curType));
        start = idx;
        curType = uType;
      }
    }
    // store the last chunk
    if (start !== lineStr.length) {
      wordChunks.push(new Chunk(start, lineStr.length, curType));
    }
    // update neutral and weak chunks
    const wordChunksLen = wordChunks.length;
    // for idx in 0..wordChunksLen {
    for (let idx = 0; idx < wordChunksLen; idx++) {
      const prev: Chunk | undefined = idx === 0 ? undefined : wordChunks[idx - 1];
      const next: Chunk | undefined = idx === wordChunksLen - 1 ? undefined : wordChunks[idx + 1];
      const chunk = wordChunks[idx];
      if (chunk.isType(Type.Neutral)) {
        if (idx === 0) {
          chunk.setChunkType(next);
        } else if (idx === wordChunksLen - 1) {
          continue;
        } else if (prev?.type === next?.type) {
          chunk.setChunkType(prev);
        } else {
          chunk.setType(dominantRTL ? Type.Rtl : Type.Ltr);
        }
      } else if (
        chunk.isType(Type.Weak) &&
        idx !== 0 &&
        (prev?.isType(Type.Rtl) ?? false) &&
        !dominantRTL
      ) {
        // swap chunks
        const tmp = wordChunks[idx - 1];
        wordChunks[idx - 1] = wordChunks[idx];
        wordChunks[idx] = tmp;
      }
    }
    // merge chunks that are the same
    let i = 0;
    while (i < wordChunks.length - 1) {
      if (wordChunks[i].type === wordChunks[i + 1].type) {
        wordChunks[i].end = wordChunks[i + 1].end;
        wordChunks.splice(i + 1, 1);
        i -= 1;
      }
      i += 1;
    }
    // s2.3: If RTL dominant, then reverse the whole sentence (all chunks)
    if (dominantRTL) wordChunks.reverse();
    // s2.4: Store each part, reversing each chunk as needed
    for (const chunk of wordChunks) {
      const chunkVec = [...lineStr.slice(chunk.start, chunk.end)];
      if (chunk.isType(Type.Rtl)) {
        chunkVec.reverse();
        // run through the chunkVec and check for any mirrored characters (e.g. parentheses)
        mirrorAdjustString(chunkVec);
      }
      result.push(...chunkVec);
    }
    // TODO: use the original return (\n or \r) not just \n
    if (lineIDX !== lines.length - 1) result.push(0x000a); // \n
  }

  return result;
}
