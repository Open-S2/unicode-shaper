# unicode-shaper-rust

A [Rust](https://github.com/rust-lang/rust) port of a subset of the functionality of [International Components for Unicode (ICU)](http://site.icu-project.org/). Supports right-to-left langauges like Arabic and Hebrew.

## Purpose

This library is intended to be used in the browser, and can be compiled to WebAssembly. [Mapbox uses emscripten](https://bundlejs.com/?q=%40mapbox%2Fmapbox-gl-rtl-text%2C%40mapbox%2Fmapbox-gl-rtl-text&treeshake=%5B*%5D%2C%5B%7B+default+%7D%5D) but as you can see, the bundle size is 186.21kB [44.08kB (gzip)]. Because of it's size, it's imported as a separate module. Due to Rusts's first class WASM support the bundle size is 12kB bytes of wasm + 3.257 kB JS (example case). This is a 91.8065624832% reduction in size.

## Using the unicode shaper

The unicode shaper exposes three functions for modules:

### `pub fn shape_arabic(dest: &mut [u16], tashkeel_flag: i8, shapeVars: UShapeVariables)`

Takes an input string in "logical order" (i.e. characters in the order they are typed, not the order they will be displayed) and replaces unicodes like Arabic characters with the "presentation form" of the character that represents the appropriate glyph based on the character's location within a word.

### `pub fn process_bidi_text(input: &[u16]) -> Vec<u16>`

Takes an input string with characters in "logical order", along with a set of chosen line break points, and applies the [Unicode Bidirectional Algorithm](http://unicode.org/reports/tr9/) to the string. Returns a new line in "visual order" (i.e. characters in the order they are displayed, left-to-right).

### `pub fn shape_unicode(source: &[u16], options: &u32) -> Vec<u16>`

Combines `shape_arabic` and `process_bidi_text` to process a string with both shaping and bidirectional processing.
Takes an input string with characters in "logical order", and applies the [Unicode Bidirectional Algorithm](http://unicode.org/reports/tr9/) to the string. Returns a new line with characters in "visual order" (i.e. characters in the order they are displayed, left-to-right) and replaces Arabic characters with the "presentation form" of the character that represents the appropriate glyph based on the character's location within a word.

### isRTL(input: u16) bool

Check if the unicode character is right to left.

### isCJK(input: u16) bool

Check if the unicode character is Chinese, Japanese, or Korean.
Useful if you want to draw text as a vertical line for CJK characters.

## Install

```sh
npm install unicode-shaper-rust
pnpm add unicode-shaper-rust
yarn add unicode-shaper-rust
bun add unicode-shaper-rust
```

## Example

```ts
    import WASM from 'unicode-shaper-rust'
    const wasm = new WASM()
    const text = 'سلام۳۹' // [1587, 1604, 1575, 1605, 1779, 1785]
    const output = wasm.processString(input)
    console.log(output) // [1779, 1785, 65249, 65276, 65203]
    // => '۳۹ﻡﻼﺳ'
```

## Prerequisites for Contributing

### Bun

Follow the installation guides for [Bun](https://bun.sh/docs/install).

### Rust

Follow the installation guides for [Rust](https://rustup.rs/).

### WABT

Follow the installation guides for [WABT](https://github.com/WebAssembly/wabt). Be sure to expose the tools in your path.

## CPP TESTS: Build

### 1 Download and build ICU

```sh
# feel free to check https://github.com/unicode-org/icu/releases for latest version
# just be sure to use the RC `release-XX-rc/icu4c-XXrc-src.tgz`
wget https://github.com/unicode-org/icu/releases/download/release-75-rc/icu4c-75rc-src.tgz
tar xzf icu4c-75rc-src.tgz
rm icu4c-75rc-src.tgz

cd icu/source

./runConfigureICU --help
# MACOSX
CXXFLAGS=-std=c++20 ./runConfigureICU MacOSX --disable-renaming
# Linux
CXXFLAGS=-std=c++20 ./runConfigureICU Linux/gcc --disable-renaming

make clean
make -j4
sudo make install

```

### 2 Run comparison experiments via test.cpp

```sh
g++ -stdlib=libc++ -std=c++20 -w -fPIC -I/usr/local/include -L/usr/local/lib -licuuc test.cpp -o test
```

## Supported Lanaguages

[x] [Standard (Latin, Cyrillic, Greek, etc.)](https://learn.microsoft.com/en-us/typography/script-development/standard)
[x] [Arabic](https://learn.microsoft.com/en-us/typography/script-development/arabic)
[ ] [Buginese](https://learn.microsoft.com/en-us/typography/script-development/buginese)
[ ] [Hangul](https://learn.microsoft.com/en-us/typography/script-development/hangul)
[x] [Hebrew](https://learn.microsoft.com/en-us/typography/script-development/hebrew)
[ ] [Indic: Bengali](https://learn.microsoft.com/en-us/typography/script-development/bengali)
[ ] [Indic: Devanagari](https://learn.microsoft.com/en-us/typography/script-development/devanagari)
[ ] [Indic: Gujarati](https://learn.microsoft.com/en-us/typography/script-development/gujarati)
[ ] [Indic: Gurmukhi](https://learn.microsoft.com/en-us/typography/script-development/gurmukhi)
[ ] [Indic: Kannada](https://learn.microsoft.com/en-us/typography/script-development/kannada)
[ ] [Indic: Malayalam](https://learn.microsoft.com/en-us/typography/script-development/malayalam)
[ ] [Indic: Odia](https://learn.microsoft.com/en-us/typography/script-development/odia)
[ ] [Indic: Tamil](https://learn.microsoft.com/en-us/typography/script-development/tamil)
[ ] [Indic: Telugu](https://learn.microsoft.com/en-us/typography/script-development/telugu)
[ ] [Javanese](https://learn.microsoft.com/en-us/typography/script-development/javanese)
[ ] [Khmer](https://learn.microsoft.com/en-us/typography/script-development/khmer)
[ ] [Lao](https://learn.microsoft.com/en-us/typography/script-development/lao)
[x] [Myanmar](https://learn.microsoft.com/en-us/typography/script-development/myanmar)
[ ] [Sinhala](https://learn.microsoft.com/en-us/typography/script-development/sinhala)
[ ] [Syric](https://learn.microsoft.com/en-us/typography/script-development/syriac)
[ ] [Thaana](https://learn.microsoft.com/en-us/typography/script-development/thaana)
[ ] [Thai](https://learn.microsoft.com/en-us/typography/script-development/thai)
[x] [Tibetan](https://learn.microsoft.com/en-us/typography/script-development/tibetan)
