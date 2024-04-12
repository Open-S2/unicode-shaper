import wasmBase64 from './u-shaper.wasm.js'

type WasmFree = (ptr: number, len: number) => void
type WasmProcessText = (ptr: number, len: number, options: number) => void
type WasmAllocSentinel = (size: number) => number
type WasmIsRTL = (unicode: number) => number
type WasmIsCJK = (unicode: number) => number

export const U_SHAPE_DIRECTION_OUTPUT_BIDI = 1 << 20
export const U_SHAPE_LETTERS_SHAPE = 8
export const U_SHAPE_LETTERS_MASK = 0x18
export const U_SHAPE_TEXT_DIRECTION_LOGICAL = 0
export const U_SHAPE_TEXT_DIRECTION_MASK = 4

export const DEFAULT_OPTIONS = (U_SHAPE_LETTERS_SHAPE & U_SHAPE_LETTERS_MASK) |
(U_SHAPE_TEXT_DIRECTION_LOGICAL & U_SHAPE_TEXT_DIRECTION_MASK) | U_SHAPE_DIRECTION_OUTPUT_BIDI
export const DEFAULT_OPTIONS_WITHOUT_BIDI_SHAPING = (U_SHAPE_LETTERS_SHAPE & U_SHAPE_LETTERS_MASK) |
(U_SHAPE_TEXT_DIRECTION_LOGICAL & U_SHAPE_TEXT_DIRECTION_MASK)

export default class TextShaper {
  instance!: WebAssembly.Instance
  wasmMemory?: Uint8Array
  tmpString = ''
  constructor () {
    const mod = new WebAssembly.Module(base64ToArrayBuffer(wasmBase64))
    this.instance = new WebAssembly.Instance(mod, {
      env: {
        setUnicodeArray: (ptr: number, len: number): void => {
          this.tmpString = ''
          if (len === 0 || ptr < 0) return
          const buf = this.#get(ptr, len)
          for (let i = 0; i < len; i++) this.tmpString += String.fromCharCode(buf[i])
        }
      }
    })
  }

  shapeString (str: string, options = DEFAULT_OPTIONS): string {
    const processText = this.instance.exports.processText as WasmProcessText
    const free = this.instance.exports.free as WasmFree

    if (str.length === 0) return str

    const len = str.length
    // NOTE: putString allocates memory, but processText will free it for us
    const ptr = this.#putString(str)
    processText(ptr, len, options)
    free(ptr, len)
    return this.tmpString
  }

  isRTL (unicode: number): boolean {
    const isRTL = this.instance.exports.isRTL as WasmIsRTL
    return isRTL(unicode) === 1
  }

  isCJK (unicode: number): boolean {
    const isCJK = this.instance.exports.isCJK as WasmIsCJK
    return isCJK(unicode) === 1
  }

  #putString (str: string): number {
    const len = str.length
    const buf = new Uint16Array(len)
    const ptr = this.#allocUnicodeArray(len)
    for (let i = 0; i < len; i++) buf[i] = str.charCodeAt(i)

    const view = this.#getMemory()
    view.subarray(ptr, ptr + (len * 2)).set(new Uint8Array(buf.buffer))

    return ptr
  }

  #get (ptr: number, len: number): Uint16Array {
    const view = this.#getMemory()
    const view16 = new Uint16Array(view.buffer, ptr, len)
    const copy = new Uint16Array(len)
    for (let i = 0; i < len; i++) copy[i] = view16[i]

    return copy
  }

  #allocUnicodeArray (size: number): number {
    const allocUnicodeArray = this.instance.exports.allocUnicodeArray as WasmAllocSentinel
    return allocUnicodeArray(size)
  }

  #getMemory (): Uint8Array {
    const memory = this.instance.exports.memory as WebAssembly.Memory
    if (this.wasmMemory === undefined || this.wasmMemory.buffer !== memory.buffer) {
      this.wasmMemory = new Uint8Array(memory.buffer)
    }
    return this.wasmMemory
  }
}

// polyfill
function base64ToArrayBuffer (base64: string): ArrayBuffer {
  const binaryString = atob(base64)
  const len = binaryString.length
  const bytes = new Uint8Array(len)
  for (let i = 0; i < len; i++) bytes[i] = binaryString.charCodeAt(i)

  return bytes.buffer as ArrayBuffer
}
