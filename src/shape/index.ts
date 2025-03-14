import { processBidiText } from '../ubidi';
import { shapeArabic } from './arabic';
import { TAMIL_VOWELS, U_SHAPE_DIRECTION_OUTPUT_BIDI, U_SHAPE_LETTERS_MASK } from './internal';

export * from './internal';

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
 * Converts Tamil Unicode buffer into a shaped Tamil Unicode buffer
 * @param input - input unicode buffer
 */
export function shape_tamil(input: number[]): void {
  sharedShaper(input, TAMIL_VOWELS);
}

/**
 * Converts an Arabic Unicode buffer in 06xx Range into a shaped
 * arabic Unicode buffer in FExx Range
 * @param source - input unicode buffer
 * @param options - shaping options
 * @returns - shaped unicode buffer
 */
export function shapeUnicode(source: number[], options: number): number[] {
  let output: number[] = [...source];

  // all other shaping
  if ((options & U_SHAPE_LETTERS_MASK) !== 0) {
    // arabic shaping
    output = shapeArabic(output, options);
    // TODO:
    // // Buginese shaping
    // shapeBuginese(output);
    // // Javanese shaping
    // shapeJavanese(output);
    // // Myanmar shaping
    // shapeMyanmar(output);
    // // Tamil shaping
    // shapeTamil(output);
    // // Tibetan shaping
    // shapeTibetan(output);
    // // khmer
    // shapeKhmer(output);
  }

  // if option to process bidirectional text is set, then reorder the output
  if ((options & U_SHAPE_DIRECTION_OUTPUT_BIDI) !== 0) {
    return processBidiText(output);
  }

  return output;
}
