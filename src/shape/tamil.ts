import { TAMIL_VOWELS } from './internal.js';
import { sharedShaper } from './shared.js';

/**
 * Check if a character is Tamil
 * @param unicode - input unicode character
 * @returns - True if Tamil
 */
export function isTamil(unicode: number): boolean {
  return unicode >= 0x0b80 && unicode <= 0x0bff;
}

/**
 * Converts Tamil Unicode buffer into a shaped Tamil Unicode buffer
 * @param input - input unicode buffer
 */
export function shapeTamil(input: number[]): void {
  sharedShaper(input, TAMIL_VOWELS);
}
