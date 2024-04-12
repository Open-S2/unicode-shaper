import { describe, expect, test } from 'bun:test'
import WASM, { DEFAULT_OPTIONS_WITHOUT_BIDI_SHAPING } from '../lib/index.js'

describe ('many different string cases', () => {
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
  test('burmese string', () => {
    expect(wasm.shapeString('ရန်ကုန်တိုင်းဒေသကြီး')).toEqual('ရန်ကုန်တိုင်းဒေသကြီး')
  })
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

  test('potentially bad string', () => {
    const charArr = [65185, 65261, 65198, 65220, 65251, 10, 77, 97, 116, 114, 111, 117, 104, 32, 71, 111, 118, 101, 114, 110, 111, 114, 97, 116, 101]
    const str = String.fromCharCode(...charArr)
    const expected = 'ﻣﻄﺮﻭﺡ\nMatrouh Governorate'
    expect(wasm.shapeString(str)).toEqual(expected)
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
