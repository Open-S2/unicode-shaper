#![allow(dead_code)]
// Shape Arabic text on a character basis.
//
// <p>This function performs basic operations for "shaping" Arabic text. It is most
// useful for use with legacy data formats and legacy display technology
// (simple terminals). All operations are performed on Unicode characters.</p>
//
// <p>Text-based shaping means that some character code points in the text are
// replaced by others depending on the context. It transforms one kind of text
// into another. In comparison, modern displays for Arabic text select
// appropriate, context-dependent font glyphs for each text element, which means
// that they transform text into a glyph vector.</p>
//
// <p>Text transformations are necessary when modern display technology is not
// available or when text needs to be transformed to or from legacy formats that
// use "shaped" characters. Since the Arabic script is cursive, connecting
// adjacent letters to each other, computers select images for each letter based
// on the surrounding letters. This usually results in four images per Arabic
// letter: initial, middle, final, and isolated forms. In Unicode, on the other
// hand, letters are normally stored abstract, and a display system is expected
// to select the necessary glyphs. (This makes searching and other text
// processing easier because the same letter has only one code.) It is possible
// to mimic this with text transformations because there are characters in
// Unicode that are rendered as letters with a specific shape
// (or cursive connectivity). They were included for interoperability with
// legacy systems and codepages, and for unsophisticated display systems.</p>
//
// <p>A second kind of text transformations is supported for Arabic digits:
// For compatibility with legacy codepages that only include European digits,
// it is possible to replace one set of digits by another, changing the
// character code points. These operations can be performed for either
// Arabic-Indic Digits (U+0660...U+0669) or Eastern (Extended) Arabic-Indic
// digits (U+06f0...U+06f9).</p>
//
// <p>Some replacements may result in more or fewer characters (code points).
// By default, this means that the destination buffer may receive text with a
// length different from the source length. Some legacy systems rely on the
// length of the text to be constant. They expect extra spaces to be added
// or consumed either next to the affected character or at the end of the
// text.</p>
//
// <p>For details about the available operations, see the description of the
// <code>U_SHAPE_...</code> options.</p>

/// List of White space characters
pub const WHITESPACE: u16 = 0x0020 | // Space
    0x0009 | // Tab
    0x000A | // Line feed
    0x000D | // Carriage return
    0x000C | // Form feed
    0x0085 | // Next line
    0x3000 | // ideographic space
    0x200B | // Zero width space
    0x00A0 | // NO-BREAK SPACE
    0x0F0C | // TIBETAN MARK DELIMITER TSHEG BSTAR
    0x202F | // NARROW NO-BREAK SPACE
    0x2060 | // WORD JOINER
    0xFEFF; // ZERO WIDTH NO-BREAK SPACE

/// definitions for Arabic letter shaping -----------------------------------
pub const IRRELEVANT: u16 = 4;
/// LAMTYPE: 16
pub const LAMTYPE: u16 = 16;
/// ALEFTYPE: 32
pub const ALEFTYPE: u16 = 32;
/// LINK RIGHT
pub const LINKR: u16 = 1;
/// LINK LEFT
pub const LINKL: u16 = 2;
/// A PRESENT
pub const APRESENT: u16 = 8;
/// SHADDA CHAR
pub const SHADDA: u16 = 64;
/// C SHADDA
pub const CSHADDA: u16 = 128;
/// COMBINE
pub const COMBINE: u16 = SHADDA + CSHADDA;

pub const HAMZAFE_CHAR: u16 = 0xfe80;
pub const HAMZA06_CHAR: u16 = 0x0621;
pub const YEH_HAMZA_CHAR: u16 = 0x0626;
pub const YEH_HAMZAFE_CHAR: u16 = 0xFE89;
pub const LAMALEF_SPACE_SUB: u16 = 0xFFFF;
pub const TASHKEEL_SPACE_SUB: u16 = 0xFFFE;
pub const NEW_TAIL_CHAR: u16 = 0xFE73;
pub const OLD_TAIL_CHAR: u16 = 0x200B;
pub const LAM_CHAR: u16 = 0x0644;
pub const SPACE_CHAR: u16 = 0x0020;
pub const SHADDA_CHAR: u16 = 0xFE7C;
pub const TATWEEL_CHAR: u16 = 0x0640;
pub const SHADDA_TATWEEL_CHAR: u16 = 0xFE7D;
pub const SHADDA06_CHAR: u16 = 0x0651;

pub struct UShapeVariables {
    pub tail_char: u16,
    pub u_shape_lamalef_begin: u32,
    pub u_shape_lamalef_end: u32,
    pub u_shape_tashkeel_begin: u32,
    pub u_shape_tashkeel_end: u32,
    pub spaces_relative_to_text_begin_end: i32,
}

impl UShapeVariables {
    pub fn new(t: u16, lb: u32, le: u32, tb: u32, te: u32, sr: i32) -> UShapeVariables {
        UShapeVariables {
            tail_char: t,
            u_shape_lamalef_begin: lb,
            u_shape_lamalef_end: le,
            u_shape_tashkeel_begin: tb,
            u_shape_tashkeel_end: te,
            spaces_relative_to_text_begin_end: sr,
        }
    }

    pub fn base() -> UShapeVariables {
        UShapeVariables {
            tail_char: NEW_TAIL_CHAR,
            u_shape_lamalef_begin: U_SHAPE_LAMALEF_BEGIN,
            u_shape_lamalef_end: U_SHAPE_LAMALEF_END,
            u_shape_tashkeel_begin: U_SHAPE_TASHKEEL_BEGIN,
            u_shape_tashkeel_end: U_SHAPE_TASHKEEL_END,
            spaces_relative_to_text_begin_end: 0,
        }
    }
}

// Memory option: allow the result to have a different length than the source.
// Affects: LamAlef options
// @stable ICU 2.0
pub const U_SHAPE_LENGTH_GROW_SHRINK: u32 = 0;

// Memory option: allow the result to have a different length than the source.
// Affects: LamAlef options
// This option is an alias to U_SHAPE_LENGTH_GROW_SHRINK
// @stable ICU 4.2
pub const U_SHAPE_LAMALEF_RESIZE: u32 = 0;

// Memory option: the result must have the same length as the source.
// If more room is necessary, then try to consume spaces next to modified characters.
// @stable ICU 2.0
pub const U_SHAPE_LENGTH_FIXED_SPACES_NEAR: u32 = 1;

// Memory option: the result must have the same length as the source.
// If more room is necessary, then try to consume spaces next to modified characters.
// Affects: LamAlef options
// This option is an alias to U_SHAPE_LENGTH_FIXED_SPACES_NEAR
// @stable ICU 4.2
pub const U_SHAPE_LAMALEF_NEAR: u32 = 1;

// Memory option: the result must have the same length as the source.
// If more room is necessary, then try to consume spaces at the end of the text.
// @stable ICU 2.0
pub const U_SHAPE_LENGTH_FIXED_SPACES_AT_END: u32 = 2;

// Memory option: the result must have the same length as the source.
// If more room is necessary, then try to consume spaces at the end of the text.
// Affects: LamAlef options
// This option is an alias to U_SHAPE_LENGTH_FIXED_SPACES_AT_END
// @stable ICU 4.2
pub const U_SHAPE_LAMALEF_END: u32 = 2;

// Memory option: the result must have the same length as the source.
// If more room is necessary, then try to consume spaces at the beginning of the text.
// @stable ICU 2.0
pub const U_SHAPE_LENGTH_FIXED_SPACES_AT_BEGINNING: u32 = 3;

// Memory option: the result must have the same length as the source.
// If more room is necessary, then try to consume spaces at the beginning of the text.
// Affects: LamAlef options
// This option is an alias to U_SHAPE_LENGTH_FIXED_SPACES_AT_BEGINNING
// @stable ICU 4.2
pub const U_SHAPE_LAMALEF_BEGIN: u32 = 3;

// Memory option: the result must have the same length as the source.
// Shaping Mode: For each LAMALEF character found, expand LAMALEF using space at end.
//               If there is no space at end, use spaces at beginning of the buffer. If there
//               is no space at beginning of the buffer, use spaces at the near (i.e. the space
//               after the LAMALEF character).
//               If there are no spaces found, an error U_NO_SPACE_AVAILABLE (as defined in utypes.h)
//               will be set in pErrorCode
//
// Deshaping Mode: Perform the same function as the flag equals U_SHAPE_LAMALEF_END.
// Affects: LamAlef options
// @stable ICU 4.2
pub const U_SHAPE_LAMALEF_AUTO: u32 = 0x10000;

// ** Bit mask for memory options. @stable ICU 2.
pub const U_SHAPE_LENGTH_MASK: u32 = 0x10003; // Changed old value

// Bit mask for LamAlef memory options.
// @stable ICU 4.2
pub const U_SHAPE_LAMALEF_MASK: u32 = 0x10003; // update

// ** Direction indicator: the source is in logical (keyboard) order. @stable ICU 2.
pub const U_SHAPE_TEXT_DIRECTION_LOGICAL: u32 = 0;

// Direction indicator:
// the source is in visual RTL order,
// the rightmost displayed character stored first.
// This option is an alias to U_SHAPE_TEXT_DIRECTION_LOGICAL
// @stable ICU 4.2
pub const U_SHAPE_TEXT_DIRECTION_VISUAL_RTL: u32 = 0;

// Direction indicator:
// the source is in visual LTR order,
// the leftmost displayed character stored first.
// @stable ICU 2.0
pub const U_SHAPE_TEXT_DIRECTION_VISUAL_LTR: u32 = 4;

// ** Bit mask for direction output. Made uniquely for this library. - @CraigglesO
pub const U_SHAPE_DIRECTION_OUTPUT_BIDI: u32 = 1 << 20;

// ** Bit mask for direction indicators. @stable ICU 2.
pub const U_SHAPE_TEXT_DIRECTION_MASK: u32 = 4;

// ** Letter shaping option: do not perform letter shaping. @stable ICU 2.
pub const U_SHAPE_LETTERS_NOOP: u32 = 0;

// ** Letter shaping option: replace abstract letter characters by "shaped" ones. @stable ICU 2.
pub const U_SHAPE_LETTERS_SHAPE: u32 = 8;

// ** Letter shaping option: replace "shaped" letter characters by abstract ones. @stable ICU 2.
pub const U_SHAPE_LETTERS_UNSHAPE: u32 = 0x10;

// Letter shaping option: replace abstract letter characters by "shaped" ones.
// The only difference with U_SHAPE_LETTERS_SHAPE is that Tashkeel letters
// are always "shaped" into the isolated form instead of the medial form
// (selecting code points from the Arabic Presentation Forms-B block).
// @stable ICU 2.0
pub const U_SHAPE_LETTERS_SHAPE_TASHKEEL_ISOLATED: u32 = 0x18;

// ** Bit mask for letter shaping options. @stable ICU 2.
pub const U_SHAPE_LETTERS_MASK: u32 = 0x18;

// ** Digit shaping option: do not perform digit shaping. @stable ICU 2.
pub const U_SHAPE_DIGITS_NOOP: u32 = 0;

// Digit shaping option:
// Replace European digits (U+0030...) by Arabic-Indic digits.
// @stable ICU 2.0
pub const U_SHAPE_DIGITS_EN2AN: u32 = 0x20;

// Digit shaping option:
// Replace Arabic-Indic digits by European digits (U+0030...).
// @stable ICU 2.0
pub const U_SHAPE_DIGITS_AN2EN: u32 = 0x40;

// Digit shaping option:
// Replace European digits (U+0030...) by Arabic-Indic digits if the most recent
// strongly directional character is an Arabic letter
// (<code>u_charDirection()</code> result <code>U_RIGHT_TO_LEFT_ARABIC</code> [AL]).<br>
// The direction of "preceding" depends on the direction indicator option.
// For the first characters, the preceding strongly directional character
// (initial state) is assumed to be not an Arabic letter
// (it is <code>U_LEFT_TO_RIGHT</code> [L] or <code>U_RIGHT_TO_LEFT</code> [R]).
// @stable ICU 2.0
pub const U_SHAPE_DIGITS_ALEN2AN_INIT_LR: u32 = 0x60;

// Digit shaping option:
// Replace European digits (U+0030...) by Arabic-Indic digits if the most recent
// strongly directional character is an Arabic letter
// (<code>u_charDirection()</code> result <code>U_RIGHT_TO_LEFT_ARABIC</code> [AL]).<br>
// The direction of "preceding" depends on the direction indicator option.
// For the first characters, the preceding strongly directional character
// (initial state) is assumed to be an Arabic letter.
// @stable ICU 2.0
pub const U_SHAPE_DIGITS_ALEN2AN_INIT_AL: u32 = 0x80;

// ** Not a valid option value. May be replaced by a new option. @stable ICU 2.
pub const U_SHAPE_DIGITS_RESERVED: u32 = 0xa0;

// ** Bit mask for digit shaping options. @stable ICU 2.
pub const U_SHAPE_DIGITS_MASK: u32 = 0xe0;

// ** Digit type option: Use Arabic-Indic digits (U+0660...U+0669). @stable ICU 2.
pub const U_SHAPE_DIGIT_TYPE_AN: u32 = 0;

// ** Digit type option: Use Eastern (Extended) Arabic-Indic digits (U+06f0...U+06f9). @stable ICU 2.
pub const U_SHAPE_DIGIT_TYPE_AN_EXTENDED: u32 = 0x100;

// ** Not a valid option value. May be replaced by a new option. @stable ICU 2.
pub const U_SHAPE_DIGIT_TYPE_RESERVED: u32 = 0x200;

// ** Bit mask for digit type options. @stable ICU 2.
pub const U_SHAPE_DIGIT_TYPE_MASK: u32 = 0x300; // I need to change this from 0x3f00 to 0x30

// Tashkeel aggregation option:
// Replaces any combination of U+0651 with one of
// U+064C, U+064D, U+064E, U+064F, U+0650 with
// U+FC5E, U+FC5F, U+FC60, U+FC61, U+FC62 consecutively.
// @stable ICU 3.6
pub const U_SHAPE_AGGREGATE_TASHKEEL: u32 = 0x4000;
// ** Tashkeel aggregation option: do not aggregate tashkeels. @stable ICU 3.
pub const U_SHAPE_AGGREGATE_TASHKEEL_NOOP: u32 = 0;
// ** Bit mask for tashkeel aggregation. @stable ICU 3.
pub const U_SHAPE_AGGREGATE_TASHKEEL_MASK: u32 = 0x4000;

// Presentation form option:
// Don't replace Arabic Presentation Forms-A and Arabic Presentation Forms-B
// characters with 0+06xx characters, before shaping.
// @stable ICU 3.6
pub const U_SHAPE_PRESERVE_PRESENTATION: u32 = 0x8000;
// ** Presentation form option:
// Replace Arabic Presentation Forms-A and Arabic Presentationo Forms-B with
// their unshaped correspondents in range 0+06xx, before shaping.
// @stable ICU 3.6
pub const U_SHAPE_PRESERVE_PRESENTATION_NOOP: u32 = 0;
// ** Bit mask for preserve presentation form. @stable ICU 3.
pub const U_SHAPE_PRESERVE_PRESENTATION_MASK: u32 = 0x8000;

// Seen Tail option
// Memory option: the result must have the same length as the source.
// Shaping mode: The SEEN family character will expand into two characters using space near
//               the SEEN family character(i.e. the space after the character).
//               If there are no spaces found, an error U_NO_SPACE_AVAILABLE (as defined in utypes.h)
//               will be set in pErrorCode
//
// De-shaping mode: Any Seen character followed by Tail character will be
//                  replaced by one cell Seen and a space will replace the Tail.
// Affects: Seen options
// @stable ICU 4.2
pub const U_SHAPE_SEEN_TWOCELL_NEAR: u32 = 0x200000;

// **
// Bit mask for Seen memory options.
// @stable ICU 4.2
pub const U_SHAPE_SEEN_MASK: u32 = 0x700000;

// ** YehHamza option
// Memory option: the result must have the same length as the source.
// Shaping mode: The YEHHAMZA character will expand into two characters using space near it
//              (i.e. the space after the character
//               If there are no spaces found, an error U_NO_SPACE_AVAILABLE (as defined in utypes.h)
//               will be set in pErrorCode
//
// De-shaping mode: Any Yeh (final or isolated) character followed by Hamza character will be
//                  replaced by one cell YehHamza and space will replace the Hamza.
// Affects: YehHamza options
// @stable ICU 4.2
pub const U_SHAPE_YEHHAMZA_TWOCELL_NEAR: u32 = 0x1000000;

// Bit mask for YehHamza memory options.
// @stable ICU 4.2
pub const U_SHAPE_YEHHAMZA_MASK: u32 = 0x3800000;

// New Tashkeel option
//
// Memory option: the result must have the same length as the source.
// Shaping mode: Tashkeel characters will be replaced by spaces.
//               Spaces will be placed at beginning of the buffer
//
// De-shaping mode: N/A
// Affects: Tashkeel options
// @stable ICU 4.2
pub const U_SHAPE_TASHKEEL_BEGIN: u32 = 0x40000;

// Memory option: the result must have the same length as the source.
// Shaping mode: Tashkeel characters will be replaced by spaces.
//               Spaces will be placed at end of the buffer
//
// De-shaping mode: N/A
// Affects: Tashkeel options
// @stable ICU 4.2
pub const U_SHAPE_TASHKEEL_END: u32 = 0x60000;

// Memory option: allow the result to have a different length than the source.
// Shaping mode: Tashkeel characters will be removed, buffer length will shrink.
// De-shaping mode: N/A
//
// Affect: Tashkeel options
// @stable ICU 4.2
pub const U_SHAPE_TASHKEEL_RESIZE: u32 = 0x80000;

// Memory option: the result must have the same length as the source.
// Shaping mode: Tashkeel characters will be replaced by Tatweel if it is connected to adjacent
//               characters (i.e. shaped on Tatweel) or replaced by space if it is not connected.
//
// De-shaping mode: N/A
// Affects: YehHamza options
// @stable ICU 4.2
pub const U_SHAPE_TASHKEEL_REPLACE_BY_TATWEEL: u32 = 0xC0000;

// Bit mask for Tashkeel replacement with Space or Tatweel memory options.
// @stable ICU 4.2
pub const U_SHAPE_TASHKEEL_MASK: u32 = 0xE0000;

// Space location Control option
//
// This option affect the meaning of BEGIN and END options. if this option is not used the default
// for BEGIN and END will be as following:
// The Default (for both Visual LTR, Visual RTL and Logical Text)
//           1. BEGIN always refers to the start address of physical memory.
//           2. END always refers to the end address of physical memory.
//
// If this option is used it will swap the meaning of BEGIN and END only for Visual LTR text.
//
// The effect on BEGIN and END Memory Options will be as following:
//    A. BEGIN For Visual LTR text: This will be the beginning (right side) of the visual text(
//       corresponding to the physical memory address end for Visual LTR text, Same as END in
//       default behavior)
//    B. BEGIN For Logical text: Same as BEGIN in default behavior.
//    C. END For Visual LTR text: This will be the end (left side) of the visual text (corresponding
//       to the physical memory address beginning for Visual LTR text, Same as BEGIN in default behavior.
//    D. END For Logical text: Same as END in default behavior).
// Affects: All LamAlef BEGIN, END and AUTO options.
// @stable ICU 4.2
pub const U_SHAPE_SPACES_RELATIVE_TO_TEXT_BEGIN_END: u32 = 0x4000000;

// Bit mask for swapping BEGIN and END for Visual LTR text
// @stable ICU 4.2
pub const U_SHAPE_SPACES_RELATIVE_TO_TEXT_MASK: u32 = 0x4000000;

// If this option is used, shaping will use the new Unicode code point for TAIL (i.e. 0xFE73).
// If this option is not specified (Default), old unofficial Unicode TAIL code point is used (i.e. 0x200B)
// De-shaping will not use this option as it will always search for both the new Unicode code point for the
// TAIL (i.e. 0xFE73) or the old unofficial Unicode TAIL code point (i.e. 0x200B) and de-shape the
// Seen-Family letter accordingly.
//
// Shaping Mode: Only shaping.
// De-shaping Mode: N/A.
// Affects: All Seen options
// @stable ICU 4.8
pub const U_SHAPE_TAIL_NEW_UNICODE: u32 = 0x8000000;

// Bit mask for new Unicode Tail option
// @stable ICU 4.8
pub const U_SHAPE_TAIL_TYPE_MASK: u32 = 0x8000000;

pub static YEH_HAMZA_TO_YEH: [u16; 2] = [
    0xFEEF, //* isolated*/
    0xFEF0, //* final   */
];

pub static CONVERT_LAM_ALEF: [u16; 8] = [
    0x0622, //*FEF5*/
    0x0622, //*FEF6*/
    0x0623, //*FEF7*/
    0x0623, //*FEF8*/
    0x0625, //*FEF9*/
    0x0625, //*FEFA*/
    0x0627, //*FEFB*/
    0x0627, //*FEFC*/
];

pub static TASHKEEL_MEDIAL: [u8; 16] = [
    0, // FE70
    1, // FE71
    0, // FE72
    0, // FE73
    0, // FE74
    0, // FE75
    0, // FE76
    1, // FE77
    0, // FE78
    1, // FE79
    0, // FE7A
    1, // FE7B
    0, // FE7C
    1, // FE7D
    0, // FE7E
    1, // FE7F
];

pub static TAIL_FAMILY_ISOLATED_FINAL: [u8; 14] = [
    1, //* FEB1 */
    1, //* FEB2 */
    0, //* FEB3 */
    0, //* FEB4 */
    1, //* FEB5 */
    1, //* FEB6 */
    0, //* FEB7 */
    0, //* FEB8 */
    1, //* FEB9 */
    1, //* FEBA */
    0, //* FEBB */
    0, //* FEBC */
    1, //* FEBD */
    1, //* FEBE */
];

pub static ARA_LINK: [u16; 178] = [
    1 + 32 + 256 * 0x11, // 0x0622*/
    1 + 32 + 256 * 0x13, // 0x0623*/
    1 + 256 * 0x15,      // 0x0624*/
    1 + 32 + 256 * 0x17, // 0x0625*/
    1 + 2 + 256 * 0x19,  // 0x0626*/
    1 + 32 + 256 * 0x1D, // 0x0627*/
    1 + 2 + 256 * 0x1F,  // 0x0628*/
    1 + 256 * 0x23,      // 0x0629*/
    1 + 2 + 256 * 0x25,  // 0x062A*/
    1 + 2 + 256 * 0x29,  // 0x062B*/
    1 + 2 + 256 * 0x2D,  // 0x062C*/
    1 + 2 + 256 * 0x31,  // 0x062D*/
    1 + 2 + 256 * 0x35,  // 0x062E*/
    1 + 256 * 0x39,      // 0x062F*/
    1 + 256 * 0x3B,      // 0x0630*/
    1 + 256 * 0x3D,      // 0x0631*/
    1 + 256 * 0x3F,      // 0x0632*/
    1 + 2 + 256 * 0x41,  // 0x0633*/
    1 + 2 + 256 * 0x45,  // 0x0634*/
    1 + 2 + 256 * 0x49,  // 0x0635*/
    1 + 2 + 256 * 0x4D,  // 0x0636*/
    1 + 2 + 256 * 0x51,  // 0x0637*/
    1 + 2 + 256 * 0x55,  // 0x0638*/
    1 + 2 + 256 * 0x59,  // 0x0639*/
    1 + 2 + 256 * 0x5D,  // 0x063A*/
    0,
    0,
    0,
    0,
    0,                       // 0x063B-0x063F*/
    1 + 2,                   // 0x0640*/
    1 + 2 + 256 * 0x61,      // 0x0641*/
    1 + 2 + 256 * 0x65,      // 0x0642*/
    1 + 2 + 256 * 0x69,      // 0x0643*/
    1 + 2 + 16 + 256 * 0x6D, // 0x0644*/
    1 + 2 + 256 * 0x71,      // 0x0645*/
    1 + 2 + 256 * 0x75,      // 0x0646*/
    1 + 2 + 256 * 0x79,      // 0x0647*/
    1 + 256 * 0x7D,          // 0x0648*/
    1 + 256 * 0x7F,          // 0x0649*/
    1 + 2 + 256 * 0x81,      // 0x064A*/
    4 + 256,                 // 0x064B*/ (4 + 256 * 1)
    4 + 128 + 256,           // 0x064C*/ ( 4 + 128 + 256 * 1)
    4 + 128 + 256,           // 0x064D*/ ( 4 + 128 + 256 * 1)
    4 + 128 + 256,           // 0x064E*/ ( 4 + 128 + 256 * 1)
    4 + 128 + 256,           // 0x064F*/ ( 4 + 128 + 256 * 1)
    4 + 128 + 256,           // 0x0650*/ ( 4 + 128 + 256 * 1)
    4 + 64 + 256 * 3,        // 0x0651*/
    4 + 256,                 // 0x0652*/ (4 + 256 * 1)
    4 + 256 * 7,             // 0x0653*/
    4 + 256 * 8,             // 0x0654*/
    4 + 256 * 8,             // 0x0655*/
    4 + 256,                 // 0x0656*/ (4 + 256 * 1)
    0,
    0,
    0,
    0,
    0,              // 0x0657-0x065B*/
    1 + 256 * 0x85, // 0x065C*/
    1 + 256 * 0x87, // 0x065D*/
    1 + 256 * 0x89, // 0x065E*/
    1 + 256 * 0x8B, // 0x065F*/
    0,
    0,
    0,
    0,
    0, // 0x0660-0x0664*/
    0,
    0,
    0,
    0,
    0, // 0x0665-0x0669*/
    0,
    0,
    0,
    0,
    0,
    0,           // 0x066A-0x066F*/
    4 + 256 * 6, // 0x0670*/
    1 + 8,       // 0x0671*/ (1 + 8 + 256 * 0x00)
    1 + 32,      // 0x0672*/
    1 + 32,      // 0x0673*/
    0,           // 0x0674*/
    1 + 32,      // 0x0675*/
    1,
    1,                      // 0x0676-0x0677*/
    1 + 2,                  // 0x0678*/
    1 + 2 + 8 + 256 * 0x16, // 0x0679*/
    1 + 2 + 8 + 256 * 0x0E, // 0x067A*/
    1 + 2 + 8 + 256 * 0x02, // 0x067B*/
    1 + 2,
    1 + 2, // 0x67C-0x067D*/
    1 + 2 + 8 + 256 * 0x06,
    1 + 2,
    1 + 2,
    1 + 2,
    1 + 2,
    1 + 2, // 0x067E-0x0683*/
    1 + 2,
    1 + 2,
    1 + 2 + 8 + 256 * 0x2A,
    1 + 2,              // 0x0684-0x0687*/
    1 + 8 + 256 * 0x38, // 0x0688*/
    1,
    1,
    1,                  // 0x0689-0x068B*/
    1 + 8 + 256 * 0x34, // 0x068C*/
    1 + 8 + 256 * 0x32, // 0x068D*/
    1 + 8 + 256 * 0x36, // 0x068E*/
    1,
    1,                  // 0x068F-0x0690*/
    1 + 8 + 256 * 0x3C, // 0x0691*/
    1,
    1,
    1,
    1,
    1,
    1,
    1 + 8 + 256 * 0x3A,
    1, // 0x0692-0x0699*/
    1 + 2,
    1 + 2,
    1 + 2,
    1 + 2,
    1 + 2,
    1 + 2, // 0x069A-0x06A3*/
    1 + 2,
    1 + 2,
    1 + 2,
    1 + 2, // 0x069A-0x06A3*/
    1 + 2,
    1 + 2,
    1 + 2,
    1 + 2,
    1 + 2,
    1 + 2 + 8 + 256 * 0x3E, // 0x06A4-0x06AD*/
    1 + 2,
    1 + 2,
    1 + 2,
    1 + 2, // 0x06A4-0x06AD*/
    1 + 2,
    1 + 2 + 8 + 256 * 0x42,
    1 + 2,
    1 + 2,
    1 + 2,
    1 + 2, // 0x06AE-0x06B7*/
    1 + 2,
    1 + 2,
    1 + 2,
    1 + 2, // 0x06AE-0x06B7*/
    1 + 2,
    1 + 2,                  // 0x06B8-0x06B9*/
    1 + 8 + 256 * 0x4E,     // 0x06BA*/
    1 + 2 + 8 + 256 * 0x50, // 0x06BB*/
    1 + 2,
    1 + 2,                  // 0x06BC-0x06BD*/
    1 + 2 + 8 + 256 * 0x5A, // 0x06BE*/
    1 + 2,                  // 0x06BF*/
    1 + 8 + 256 * 0x54,     // 0x06C0*/
    1 + 2 + 8 + 256 * 0x56, // 0x06C1*/
    1,
    1,
    1,                      // 0x06C2-0x06C4*/
    1 + 8 + 256 * 0x90,     // 0x06C5*/
    1 + 8 + 256 * 0x89,     // 0x06C6*/
    1 + 8 + 256 * 0x87,     // 0x06C7*/
    1 + 8 + 256 * 0x8B,     // 0x06C8*/
    1 + 8 + 256 * 0x92,     // 0x06C9*/
    1,                      // 0x06CA*/
    1 + 8 + 256 * 0x8E,     // 0x06CB*/
    1 + 2 + 8 + 256 * 0xAC, // 0x06CC*/
    1,                      // 0x06CD*/
    1 + 2,
    1 + 2,                  // 0x06CE-0x06CF*/
    1 + 2 + 8 + 256 * 0x94, // 0x06D0*/
    1 + 2,                  // 0x06D1*/
    1 + 8 + 256 * 0x5E,     // 0x06D2*/
    1 + 8 + 256 * 0x60,     // 0x06D3*/
];

pub static PRES_ALINK: [u8; 275] = [
    //**0*****1*****2*****3*****4*****5*****6*****7*****8*****9*****A*****B*****C*****D*****E*****F*/
    0,
    1,
    0,
    0,
    0,
    0,
    0,
    1,
    2,
    1 + 2,
    0,
    0,
    0,
    0,
    0,
    0, //*FB5*/
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0, //*FB6*/
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1,
    2,
    1 + 2,
    0,
    0, //*FB7*/
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1,
    0,
    0,
    0,
    1, //*FB8*/
    2,
    1 + 2,
    0,
    1,
    2,
    1 + 2,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0, //*FB9*/
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0, //*FBA*/
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0, //*FBB*/
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0, //*FBC*/
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0, //*FBD*/
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0, //*FBE*/
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1,
    2,
    1 + 2, //*FBF*/
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0, //*FC0*/
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0, //*FC1*/
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0, //*FC2*/
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0, //*FC3*/
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0, //*FC4*/
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    4,
    4, //*FC5*/
    4,
    4,
    4, //*FC6*/
];

pub static PRES_BLINK: [u8; 144] = [
    //******0*****1*****2*****3*****4*****5*****6*****7*****8*****9*****A*****B*****C*****D*****E*****F*/
    1 + 2,
    1 + 2,
    1 + 2,
    0,
    1 + 2,
    0,
    1 + 2,
    1 + 2,
    1 + 2,
    1 + 2,
    1 + 2,
    1 + 2,
    1 + 2,
    1 + 2,
    1 + 2,
    1 + 2, //*FE7*/
    0,
    0,
    1,
    0,
    1,
    0,
    1,
    0,
    1,
    0,
    1,
    2,
    1 + 2,
    0,
    1,
    0, //*FE8*/
    1,
    2,
    1 + 2,
    0,
    1,
    0,
    1,
    2,
    1 + 2,
    0,
    1,
    2,
    1 + 2,
    0,
    1,
    2, //*FE9*/
    1 + 2,
    0,
    1,
    2,
    1 + 2,
    0,
    1,
    2,
    1 + 2,
    0,
    1,
    0,
    1,
    0,
    1,
    0, //*FEA*/
    1,
    0,
    1,
    2,
    1 + 2,
    0,
    1,
    2,
    1 + 2,
    0,
    1,
    2,
    1 + 2,
    0,
    1,
    2, //*FEB*/
    1 + 2,
    0,
    1,
    2,
    1 + 2,
    0,
    1,
    2,
    1 + 2,
    0,
    1,
    2,
    1 + 2,
    0,
    1,
    2, //*FEC*/
    1 + 2,
    0,
    1,
    2,
    1 + 2,
    0,
    1,
    2,
    1 + 2,
    0,
    1,
    2,
    1 + 2,
    0,
    1,
    2, //*FED*/
    1 + 2,
    0,
    1,
    2,
    1 + 2,
    0,
    1,
    2,
    1 + 2,
    0,
    1,
    2,
    1 + 2,
    0,
    1,
    0, //*FEE*/
    1,
    0,
    1,
    2,
    1 + 2,
    0,
    1,
    0,
    1,
    0,
    1,
    0,
    1,
    0,
    0,
    0, //*FEF*/
];

pub static CONVERT_FBTO06: [u16; 176] = [
    //******0******1******2******3******4******5******6******7******8******9******A******B******C******D******E******F***/
    0x671, 0x671, 0x67B, 0x67B, 0x67B, 0x67B, 0x67E, 0x67E, 0x67E, 0x67E, 0, 0, 0, 0, 0x67A,
    0x67A, //*FB5*/
    0x67A, 0x67A, 0, 0, 0, 0, 0x679, 0x679, 0x679, 0x679, 0, 0, 0, 0, 0, 0, //*FB6*/
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x686, 0x686, 0x686, 0x686, 0, 0, //*FB7*/
    0, 0, 0x68D, 0x68D, 0x68C, 0x68C, 0x68E, 0x68E, 0x688, 0x688, 0x698, 0x698, 0x691, 0x691,
    0x6A9, 0x6A9, //*FB8*/
    0x6A9, 0x6A9, 0x6AF, 0x6AF, 0x6AF, 0x6AF, 0, 0, 0, 0, 0, 0, 0, 0, 0x6BA, 0x6BA, //*FB9*/
    0x6BB, 0x6BB, 0x6BB, 0x6BB, 0x6C0, 0x6C0, 0x6C1, 0x6C1, 0x6C1, 0x6C1, 0x6BE, 0x6BE, 0x6BE,
    0x6BE, 0x6d2, 0x6D2, //*FBA*/
    0x6D3, 0x6D3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //*FBB*/
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //*FBC*/
    0, 0, 0, 0, 0, 0, 0, 0x6C7, 0x6C7, 0x6C6, 0x6C6, 0x6C8, 0x6C8, 0, 0x6CB, 0x6CB, //*FBD*/
    0x6C5, 0x6C5, 0x6C9, 0x6C9, 0x6D0, 0x6D0, 0x6D0, 0x6D0, 0, 0, 0, 0, 0, 0, 0, 0, //*FBE*/
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x6CC, 0x6CC, 0x6CC, 0x6CC, //*FBF*/
];

pub static CONVERT_FETO06: [u16; 141] = [
    //******0******1******2******3******4******5******6******7******8******9******A******B******C******D******E******F***/
    0x64B, 0x64B, 0x64C, 0x64C, 0x64D, 0x64D, 0x64E, 0x64E, 0x64F, 0x64F, 0x650, 0x650, 0x651,
    0x651, 0x652, 0x652, //*FE7*/
    0x621, 0x622, 0x622, 0x623, 0x623, 0x624, 0x624, 0x625, 0x625, 0x626, 0x626, 0x626, 0x626,
    0x627, 0x627, 0x628, //*FE8*/
    0x628, 0x628, 0x628, 0x629, 0x629, 0x62A, 0x62A, 0x62A, 0x62A, 0x62B, 0x62B, 0x62B, 0x62B,
    0x62C, 0x62C, 0x62C, //*FE9*/
    0x62C, 0x62D, 0x62D, 0x62D, 0x62D, 0x62E, 0x62E, 0x62E, 0x62E, 0x62F, 0x62F, 0x630, 0x630,
    0x631, 0x631, 0x632, //*FEA*/
    0x632, 0x633, 0x633, 0x633, 0x633, 0x634, 0x634, 0x634, 0x634, 0x635, 0x635, 0x635, 0x635,
    0x636, 0x636, 0x636, //*FEB*/
    0x636, 0x637, 0x637, 0x637, 0x637, 0x638, 0x638, 0x638, 0x638, 0x639, 0x639, 0x639, 0x639,
    0x63A, 0x63A, 0x63A, //*FEC*/
    0x63A, 0x641, 0x641, 0x641, 0x641, 0x642, 0x642, 0x642, 0x642, 0x643, 0x643, 0x643, 0x643,
    0x644, 0x644, 0x644, //*FED*/
    0x644, 0x645, 0x645, 0x645, 0x645, 0x646, 0x646, 0x646, 0x646, 0x647, 0x647, 0x647, 0x647,
    0x648, 0x648, 0x649, //*FEE*/
    0x649, 0x64A, 0x64A, 0x64A, 0x64A, 0x65C, 0x65C, 0x65D, 0x65D, 0x65E, 0x65E, 0x65F,
    0x65F, //*FEF*/
];

pub static IRRELEVANT_POS: [u8; 8] = [0x0, 0x2, 0x4, 0x6, 0x8, 0xA, 0xC, 0xE];

pub static SHAPE_TABLE: [[[u8; 4]; 4]; 4] = [
    [[0, 0, 0, 0], [0, 0, 0, 0], [0, 1, 0, 3], [0, 1, 0, 1]],
    [[0, 0, 2, 2], [0, 0, 1, 2], [0, 1, 1, 2], [0, 1, 1, 3]],
    [[0, 0, 0, 0], [0, 0, 0, 0], [0, 1, 0, 3], [0, 1, 0, 3]],
    [[0, 0, 1, 2], [0, 0, 1, 2], [0, 1, 1, 2], [0, 1, 1, 3]],
];

pub const TAMIL_VOWELS: [u16; 11] =
    [3006, 3007, 3008, 3009, 3010, 3011, 3012, 3013, 3014, 3015, 3016];

pub const KHMER_SIGN_COENG: u16 = 0x17D2; // 6098

pub const KHMER_DEPENDENT_VOWELS: [u16; 16] = [
    // dependent vowels
    0x17B6, 0x17B7, 0x17B8, 0x17B9, 0x17BA, 0x17BB, 0x17BC, 0x17BD, 0x17BE, 0x17BF, 0x17C0, 0x17C1,
    0x17C2, 0x17C3, 0x17C4, 0x17C5,
    // various signs
];
