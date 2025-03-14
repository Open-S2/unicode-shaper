import { DEFAULT_OPTIONS, shapeUnicode } from './shape';

export * from './shape';
export * from './textShaperWasm';
export * from './ubidi';

/**
 * Converts a string into a shaped string
 * @param str - input string
 * @param options - shaping options
 * @returns - shaped string
 */
export function shapeString(str: string, options = DEFAULT_OPTIONS): string {
  if (str.length === 0) return str;

  const unicodes: number[] = new Array(str.length);
  for (let i = 0, len = str.length; i < len; i++) unicodes[i] = str.charCodeAt(i);
  const result = shapeUnicode(unicodes, options);

  return String.fromCharCode(...result);
}
