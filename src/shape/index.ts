import { processBidiText } from '../ubidi/index.js';
import { shapeArabic } from './arabic.js';
import { shapeBuginese } from './buginese.js';
import { shapeJavanese } from './javanese.js';
import { shapeKhmer } from './khmer.js';
import { shapeMyanmar } from './myanmar.js';
import { shapeTamil } from './tamil.js';
import { shapeTibetan } from './tibetan.js';
import { U_SHAPE_DIRECTION_OUTPUT_BIDI, U_SHAPE_LETTERS_MASK } from './internal.js';

export * from './arabic.js';
export * from './buginese.js';
export * from './cjk.js';
export * from './internal.js';
export * from './javanese.js';
export * from './khmer.js';
export * from './myanmar.js';
export * from './tamil.js';
export * from './thai.js';
export * from './tibetan.js';

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
    // Buginese shaping
    shapeBuginese(output);
    // Javanese shaping
    shapeJavanese(output);
    // khmer
    shapeKhmer(output);
    // Myanmar shaping
    shapeMyanmar(output);
    // Tamil shaping
    shapeTamil(output);
    // Tibetan shaping
    shapeTibetan(output);
  }

  // if option to process bidirectional text is set, then reorder the output
  if ((options & U_SHAPE_DIRECTION_OUTPUT_BIDI) !== 0) {
    return processBidiText(output);
  }

  return output;
}
