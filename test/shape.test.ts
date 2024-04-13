import { describe, expect, test } from 'bun:test'
import WASM, { DEFAULT_OPTIONS_WITHOUT_BIDI_SHAPING } from '../lib/index.js'

describe('many different string cases', () => {
  const wasm = new WASM()
  test('empty string', () => {
    expect(wasm.shapeString('')).toEqual('')
  })
  test('ascii string', () => {
    expect(wasm.shapeString('this is normal text')).toEqual('this is normal text')
  })
  test('arabic string', () => {
    expect(wasm.shapeString('سلام۳۹')).toEqual('۳۹ﻡﻼﺳ')
  })
  test('chinese string', () => {
    expect(wasm.shapeString('辽')).toEqual('辽')
  })
  // test('burmese string', () => {
  //   expect(wasm.shapeString('ရန်ကုန်တိုင်းဒေသကြီး')).toEqual('ရန်ကုန်တိုင်းဒေသကြီး')
  // })
  test('arabic string', () => {
    expect(wasm.shapeString('سلام۳۹')).toEqual('۳۹ﻡﻼﺳ')
  })
  test('arabic string but already shaped', () => {
    expect(wasm.shapeString('۳۹ﻡﻼﺳ', DEFAULT_OPTIONS_WITHOUT_BIDI_SHAPING)).toEqual('۳۹ﻡﻼﺳ')
  })
  // test('arabic string', () => {
  //   expect(wasm.shapeString(' بَّترم بَّترم ')).toEqual(' ﺑﹽﹷﺘﺮﻡ ﺑﹷﹽﺘﺮﻡ ')
  // })
  // test('arabic string', () => {
  //   expect(wasm.shapeString('اليَمَن‎‎')).toEqual('ﺍﻟﻴﹷﻤﹷﻦ‎‎')
  // })
  // test('arabic string', () => {
  //   expect(wasm.shapeString('مكتبة الإسكندرية‎‎ Maktabat al-Iskandarīyah'))
  //     .toEqual('ﻣﻜﺘﺒﺔ ﺍﻹﺳﻜﻨﺪﺭﻳﺔ‎‎ Maktabat al-Iskandarīyah')
  // })

  test('myanmar string', () => {
    const input = 'မြန်မာ'
    const expected = String.fromCharCode(...[4156, 4121, 4116, 4154, 4121, 4140])
    expect(wasm.shapeString(input)).toEqual(expected)
  })

  test('tibetan string', () => {
    const input = 'བོད་རང་སྐྱོང་ལྗོངས།'
    const expected = String.fromCharCode(...[3964, 3926, 3921, 3851, 3938, 3908, 3851, 3964, 3942, 3984, 4017, 3908, 3851, 3964, 3939, 3991, 3908, 3942, 3853])
    expect(wasm.shapeString(input)).toEqual(expected)
  })
})

describe('boolean RTL & CJK', () => {
  const wasm = new WASM()
  test('RTL', () => {
    expect(wasm.isRTL(0x01)).toEqual(false)
    expect(wasm.isRTL(0x05C3)).toEqual(true)
  })

  test('CJK', () => {
    expect(wasm.isCJK(0x01)).toEqual(false)
    expect(wasm.isCJK(0x4E00)).toEqual(true)
  })
})
