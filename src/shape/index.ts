import { processBidiText } from '../ubidi';
import { shapeArabic } from './arabic';
import { shapeBuginese } from './buginese';
import { shapeJavanese } from './javanese';
import { shapeKhmer } from './khmer';
import { shapeMyanmar } from './myanmar';
import { shapeTamil } from './tamil';
import { shapeTibetan } from './tibetan';
import { U_SHAPE_DIRECTION_OUTPUT_BIDI, U_SHAPE_LETTERS_MASK } from './internal';

export * from './arabic';
export * from './buginese';
export * from './cjk';
export * from './internal';
export * from './javanese';
export * from './khmer';
export * from './myanmar';
export * from './tamil';
export * from './thai';
export * from './tibetan';

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
