const UBIDI_MIRROR_INDEX_SHIFT = 21

let mirrors = [
    0x2000ab,0xbb,0x4202215,0x4e0221f,0x3e02220,0x3a02221,0x3c02222,0x4c02224,0x2202243,0x1402245,0x120224c,0x4002298,0x44022a6,0x48022a8,0x46022a9,0x4a022ab,
    0x38022b8,0x10022cd,0x2e022f2,0x30022f3,0x32022f4,0x34022f6,0x36022f7,0x24022fa,0x26022fb,0x28022fc,0x2a022fd,0x2c022fe,0x20027dc,0xa0299b,0xc029a0,0x8029a3,
    0x16029b8,0x4029f5,0x1802ade,0x1c02ae3,0x1a02ae4,0x1e02ae5,0xe02aee,0x602bfe
]
let matches = mirrors.map(findInverse)
mirrors = mirrors.map(toCharDecimal);

function toCharDecimal (c) {
    return c & 0x1fffff
}

function findInverse (c) {
    return toCharDecimal(mirrors[getMirror(c)]);
}

function getMirror (c) {
    return c >> UBIDI_MIRROR_INDEX_SHIFT
}

console.log(mirrors)
console.log(matches)
const blend = mirrors.map((n, i) => {
    return [n, matches[i]]
})
console.log(blend)

console.log(mirrors.map((n) => String.fromCharCode(n)))
console.log(matches.map((n) => String.fromCharCode(n)))
