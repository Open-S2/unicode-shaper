import fs from 'fs'

const data = fs.readFileSync('./UnicodeData.txt', 'utf8')
const lines = data.split('\n')
const types = {
    LTR: 0,
    RTL: 0,
    weak: 0,
    neutral: 0,
};
const weaks = []
const neutrals = []
const leftToRights = []
const rightToLefts = []

for (const line of lines) {
    const [hexCode,,,, bidiClass] = line.split(';')
    if (bidiClass === 'L') {
        types.LTR++
        var int = parseInt(hexCode, 16)
        leftToRights.push(int)
    } else if (bidiClass === 'R' || bidiClass === 'AL') {
        types.RTL++
        var int = parseInt(hexCode, 16)
        // rightToLefts.push(int)
        rightToLefts.push(int)
    } else if (
        bidiClass === 'EN' ||
        bidiClass === 'ES' ||
        bidiClass === 'ET' ||
        bidiClass === 'AN' ||
        bidiClass === 'CS' ||
        bidiClass === 'NSM'
    ) {
        types.weak++
        var int = parseInt(hexCode, 16)
        weaks.push(int)
    }
    else if (
        bidiClass === 'BN' ||
        bidiClass === 'B' ||
        bidiClass === 'S' ||
        bidiClass === 'WS' ||
        bidiClass === 'ON' ||
        bidiClass === 'LRE' ||
        bidiClass === 'LRO' ||
        bidiClass === 'RLE' ||
        bidiClass === 'RLO' ||
        bidiClass === 'PDF' ||
        bidiClass === 'LRI' ||
        bidiClass === 'RLI' ||
        bidiClass === 'FSI' ||
        bidiClass === 'PDI'
    ) {
        types.neutral++
        var int = parseInt(hexCode, 16)
        neutrals.push(int)
    }
}

console.log(types)

// const RTLRanges = createRanges(rightToLefts)

// console.log('RTL: ')
// console.log(RTLRanges)
// // print RTLRanges but convert the numbers to hex
// console.log(RTLRanges.map((range) => range.map((n) => {
//     var str = n.toString(16).toUpperCase()
//     while (str.length < 4) str = '0' + str
//     return '0x' + str
// })))

// const WeakRanges = createRanges(weaks)

// console.log('weak: ')
// // console.log(WeakRanges)
// // print WeakRanges but convert the numbers to hex
// var weakMap = WeakRanges.map((range) => range.map((n) => {
//     var str = n.toString(16).toUpperCase()
//     while (str.length < 4) str = '0' + str
//     return '0x' + str
// }))
// var index = weakMap.findIndex((range) => range[0] === '0xAAB0')
// weakMap = weakMap.slice(index)
// console.log(weakMap)

const NeutralRanges = createRanges(neutrals)

console.log('neutral: ')
// console.log(NeutralRanges)
// print NeutralRanges but convert the numbers to hex
var neutralMap = NeutralRanges.map((range) => range.map((n) => {
    var str = n.toString(16).toUpperCase()
    while (str.length < 4) str = '0' + str
    return '0x' + str
}));
// find the range index whose first number is '0x30A0'
var index = neutralMap.findIndex((range) => range[0] === '0x30A0')
// slice neutralMap from that index to the end
neutralMap = neutralMap.slice(index)
console.log(neutralMap)

// create a function that takes an array of numbers and returns an array of ranges
function createRanges (arr) {
    // step 1: group consecutive numbers
    const ranges = []
    let curRange = []
    for (let i = 0; i < arr.length; i++) {
        const n = arr[i]
        curRange.push(n)
        const next = arr[i + 1]
        if (next - n > 1) {
            ranges.push(curRange)
            curRange = []
        }
    }
    // step 2: convert the groups to ranges
    return ranges.map((range) => {
        if (range.length > 1) return [range[0], range[range.length - 1]]
        else return [range[0], range[0]]
    });
}