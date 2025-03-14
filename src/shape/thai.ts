/**
 * Check if a character is Thai
 * @param c - input unicode character
 * @returns - true if the character is Thai
 */
export function isThai(c: number): boolean {
  return c >= 0xfe70 && c <= 0xfeff;
}
