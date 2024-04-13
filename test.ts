import WASM from './lib/index.js'

const wasm = new WASM()

const MEDIALS_AND_VOWELS = [
  // MYANMAR MEDIALS
  '4155', '4156', '4157', '4158', '4190', '4191', '4192', '4226',
  // TIBETAN VOWELS
  '3953', '3954', '3955', '3956', '3957', '3958', '3959', '3960', '3961', '3962', '3963', '3964', '3965',
  // TAMIL VOWELS
  '3006', '3007', '3008', '3009', '3010', '3011', '3012', '3013', '3014', '3015', '3016'
  // ORIYA VOWELS
  // '2878' to '2888'
  // TODO: Maybe consider Decompose these? Not sure if something along the way is bugged or I'm missing something else
  // 0 decompose 2888(0B48): 2887(0B47) ->  -> previous char
  // 1 decompose 2891(0B4B): 2887(0B47) -> previous char -> 2878(0B3E)
  // 2 decompose 2892(0B4C): 2887(0B47) -> previous char -> 2903(0B57)
  // '2888', '2891', '2892'
]

// const str = 'ព្រះរាជាណាចក្រ​កម្ពុជា'
// 6038, 6098, 6042
// 1796, 17D2, 179A
// 1796 ñ KHMER LETTER PO
// 17D2  KHMER SIGN COENG (functions to indicate that the following Khmer letter is to be rendered subscripted)
// 179A ö KHMER LETTER RO
// const str = 'ម្ពុ'
// 6040, 6098, 6038, 6075
// 1798, 17D2, 1796, 17BB
// 1798 ò KHMER LETTER MO
// 17D2  KHMER SIGN COENG (functions to indicate that the following Khmer letter is to be rendered subscripted)
// 1796 ñ KHMER LETTER PO
// 17BB $ª KHMER VOWEL SIGN U
// const str = ['FB3A'].
// const str = String.fromCharCode(parseInt('FB3A', 16))
const str = String.fromCharCode(parseInt('05DA', 16)) + String.fromCharCode(parseInt('05BC', 16))
console.log(str.split('').map((n) => n.charCodeAt(0)))
const res = wasm.shapeString(str)
console.log(res)
console.log(res.split('').map((n) => n.charCodeAt(0)))
const adjust = adjustMedials(str.split(''))
console.log(adjust.join(''))
console.log(adjust.map((n) => n.charCodeAt(0)))

function adjustMedials (fieldCodes: string[]): string[] {
  for (let i = 1, fl = fieldCodes.length; i < fl; i++) {
    if (MEDIALS_AND_VOWELS.includes(fieldCodes[i])) {
      // swap with previous char
      const prev = fieldCodes[i - 1]
      fieldCodes[i - 1] = fieldCodes[i]
      fieldCodes[i] = prev
    }
  }
  return fieldCodes
}

// 6038, 6098, 6042, 6087, 6042, 6070, 6023, 6070, 6030, 6070, 6021, 6016, 6098, 6042,
// 8203, 6016, 6040, 6098, 6038, 6075, 6023, 6070

// const hexString = ['05D7', '05B7', '05E1', '05B0', '05D3', '05BC', '05B6', '05DA', '05B8']
const hexString = ['179F', '17C9', '17C1', '17B8', '17BB', '17BB', '1794']
const x = hexString.map((n) => String.fromCharCode(parseInt(n, 16)))
console.log(x.join(''))
