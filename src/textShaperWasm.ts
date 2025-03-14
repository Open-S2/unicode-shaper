import { DEFAULT_OPTIONS } from './shape/internal';
import wasmBase64 from './uShaper.wasm';

/** WASM Free Command */
type WasmFree = (ptr: number, len: number) => void;
/** WASM Process Text Command */
type WasmProcessText = (ptr: number, len: number, options: number) => void;
/** WASM Alloc Sentinel Command */
type WasmAllocSentinel = (size: number) => number;
/** WASM Is Right-to-Left Command */
type WasmIsRTL = (unicode: number) => number;
/** WASM Is Chinese-Japanese-Korean Command */
type WasmIsCJK = (unicode: number) => number;

/**
 * WASM based text shaper
 */
export class WasmTextShaper {
  instance!: WebAssembly.Instance;
  wasmMemory?: Uint8Array;
  tmpString = '';
  /** Construct the WASM instance */
  constructor() {
    const mod = new WebAssembly.Module(base64ToArrayBuffer(wasmBase64));
    this.instance = new WebAssembly.Instance(mod, {
      env: {
        /**
         * Set a unicode array
         * @param ptr - pointer
         * @param len - length
         */
        setUnicodeArray: (ptr: number, len: number): void => {
          this.tmpString = '';
          if (len === 0 || ptr < 0) return;
          const buf = this.#get(ptr, len);
          for (let i = 0; i < len; i++) this.tmpString += String.fromCharCode(buf[i]);
        },
      },
    });
  }

  /**
   * Converts a string into a shaped string
   * @param str - input string
   * @param options - shaping options
   * @returns - shaped string
   */
  shapeString(str: string, options = DEFAULT_OPTIONS): string {
    const processText = this.instance.exports.processText as WasmProcessText;
    const free = this.instance.exports.free as WasmFree;

    if (str.length === 0) return str;

    const len = str.length;
    // NOTE: putString allocates memory, but processText will free it for us
    const ptr = this.#putString(str);
    processText(ptr, len, options);
    free(ptr, len);
    return this.tmpString;
  }

  /**
   * Check if a character is a "right-to-left" unicode character
   * @param unicode - input unicode character
   * @returns - True if right-to-left
   */
  isRTL(unicode: number): boolean {
    const isRTL = this.instance.exports.isRTL as WasmIsRTL;
    return isRTL(unicode) === 1;
  }

  /**
   * Check if a character is CJK (Chinese, Japanese, or Korean)
   * @param unicode - input unicode character
   * @returns - True if CJK
   */
  isCJK(unicode: number): boolean {
    const isCJK = this.instance.exports.isCJK as WasmIsCJK;
    return isCJK(unicode) === 1;
  }

  /**
   * Converts a string into a unicode array inside the WASM memory. Returns the pointer
   * @param str - input string
   * @returns - pointer
   */
  #putString(str: string): number {
    const len = str.length;
    const buf = new Uint16Array(len);
    const ptr = this.#allocUnicodeArray(len);
    for (let i = 0; i < len; i++) buf[i] = str.charCodeAt(i);

    const view = this.#getMemory();
    view.subarray(ptr, ptr + len * 2).set(new Uint8Array(buf.buffer));

    return ptr;
  }

  /**
   * Reads a unicode array from the WASM memory
   * @param ptr - pointer
   * @param len - length
   * @returns - unicode array
   */
  #get(ptr: number, len: number): Uint16Array {
    const view = this.#getMemory();
    const view16 = new Uint16Array(view.buffer, ptr, len);
    const copy = new Uint16Array(len);
    for (let i = 0; i < len; i++) copy[i] = view16[i];

    return copy;
  }

  /**
   * Allocates a unicode array
   * @param size - size of array
   * @returns - pointer to array memory
   */
  #allocUnicodeArray(size: number): number {
    const allocUnicodeArray = this.instance.exports.allocUnicodeArray as WasmAllocSentinel;
    return allocUnicodeArray(size);
  }

  /**
   * Returns the WASM memory. Rebuilds pointer if needed
   * @returns - WASM memory
   */
  #getMemory(): Uint8Array {
    const memory = this.instance.exports.memory as WebAssembly.Memory;
    if (this.wasmMemory === undefined || this.wasmMemory.buffer !== memory.buffer) {
      this.wasmMemory = new Uint8Array(memory.buffer);
    }
    return this.wasmMemory;
  }
}

/**
 * polyfill to convert wasm base64 binary to ArrayBuffer
 * @param base64 - input base64 string
 * @returns - ArrayBuffer
 */
function base64ToArrayBuffer(base64: string): ArrayBuffer {
  const binaryString = atob(base64);
  const len = binaryString.length;
  const bytes = new Uint8Array(len);
  for (let i = 0; i < len; i++) bytes[i] = binaryString.charCodeAt(i);

  return bytes.buffer as ArrayBuffer;
}
