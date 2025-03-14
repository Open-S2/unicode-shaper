// https://www.unicode.org/charts/PDF/U0600.pdf
// https://r12a.github.io/scripts/arab/arb.html
use crate::*;
use alloc::vec::Vec;
use core::cmp::max;

// enum ShapeError {
//     UOutOfRange,
//     UNoSpaceAvailable,
// }

// Converts the Alef characters into an equivalent
// LamAlef location in the 0x06xx Range, this is an
// intermediate stage in the operation of the program
// later it'll be converted into the 0xFExx LamAlefs
// in the shaping function.
fn change_lam_alef(ch: &u16) -> u16 {
    match ch {
        0x0622 => 0x065C,
        0x0623 => 0x065D,
        0x0625 => 0x065E,
        0x0627 => 0x065F,
        _ => 0,
    }
}

// Returns true for Tashkeel characters in 06 range else return false
fn is_tashkeel_char(ch: &u16) -> bool {
    *ch >= 0x064B && *ch <= 0x0652
}

// Returns 1 for Tashkeel characters in FE range else return 0
fn is_tashkeel_char_fe(ch: &u16) -> bool {
    *ch >= 0xFE70 && *ch <= 0xFE7F
}

// Returns 1 for Alef characters else return 0
fn is_alef_char(ch: &u16) -> bool {
    *ch == 0x0622 || *ch == 0x0623 || *ch == 0x0625 || *ch == 0x0627
}

// Returns 1 for LamAlef characters else return 0
fn is_lam_alef_char(ch: &u16) -> bool {
    *ch >= 0xFEF5 && *ch <= 0xFEFC
}

// returns 1 if the character is a seen family character in the Unicode
// 06 range otherwise returns 0
// fn is_seen_family_char(ch: &u16) -> bool {
//     *ch >= 0x633 && *ch <= 0x636
// }

// returns 1 if the character is a seen family isolated character
// in the FE range otherwise returns 0
// fn is_seen_tail_family_char(ch: &u16) -> u16 {
//     if *ch >= 0xfeb1 && *ch < 0xfebf {
//         let diff: usize = (*ch - 0xFEB1).into();
//         return TAIL_FAMILY_ISOLATED_FINAL[diff].into();
//     }
//     0
// }

// returns 1 if the character is a yehHamza isolated or yehhamza
// final is found otherwise returns 0
// fn is_yeh_hamza_char(ch: &u32) -> bool {
//     *ch == 0xFE89 || *ch == 0xFE8A
// }

// Resolves the link between the characters as
// Arabic characters have four forms :
// Isolated, Initial, Middle and Final Form
fn get_link(ch: &u16) -> u16 {
    if *ch >= 0x0622 && *ch <= 0x06D3 {
        let ind: usize = (*ch - 0x0622).into();
        return ARA_LINK[ind];
    } else if *ch == 0x200D {
        return 3;
    } else if *ch >= 0x206D && *ch <= 0x206F {
        return 4;
    } else if *ch >= 0xFB50 && *ch <= 0xFC62 {
        let ind: usize = (*ch - 0xFB50).into();
        return PRES_ALINK[ind].into();
    } else if *ch >= 0xFE70 && *ch <= 0xFEFC {
        let ind: usize = (*ch - 0xFE70).into();
        return PRES_BLINK[ind].into();
    }
    0
}

// Checks if the Tashkeel Character is on Tatweel or not,if the
// Tashkeel on tatweel (FE range), it returns 1 else if the
// Tashkeel with shadda on tatweel (FC range)return 2 otherwise
// returns 0
fn is_tashkeel_on_tatweel_char(ch: &u16) -> i32 {
    if *ch >= 0xfe70
        && *ch <= 0xfe7f
        && *ch != NEW_TAIL_CHAR
        && *ch != 0xFE75
        && *ch != SHADDA_TATWEEL_CHAR
    {
        let ind: usize = (*ch - 0xFE70).into();
        return TASHKEEL_MEDIAL[ind].into();
    } else if (*ch >= 0xfcf2 && *ch <= 0xfcf4) || *ch == SHADDA_TATWEEL_CHAR {
        return 2;
    }

    0
}

// Checks if the Tashkeel Character is in the isolated form
// (i.e. Unicode FE range) returns 1 else if the Tashkeel
// with shadda is in the isolated form (i.e. Unicode FC range)
// returns 2 otherwise returns 0
fn is_isolated_tashkeel_char(ch: &u16) -> i32 {
    if *ch >= 0xfe70 && *ch <= 0xfe7f && *ch != NEW_TAIL_CHAR && *ch != 0xFE75 {
        let ind: usize = (*ch - 0xFE70).into();
        return (1 - TASHKEEL_MEDIAL[ind]).into();
    } else if *ch >= 0xfc5e && *ch <= 0xfc63 {
        return 1;
    }

    0
}

// Replaces Tashkeel as following:
// Case 1: if the Tashkeel on tatweel, replace it with Tatweel.
// Case 2: if the Tashkeel aggregated with Shadda on Tatweel, replace
//         it with Shadda on Tatweel.
// Case 3: if the Tashkeel is isolated replace it with Space.
fn handle_tashkeel_with_tatweel(dest: &mut [u16]) {
    let mut i: usize = 0;
    while i < dest.len() {
        if is_tashkeel_on_tatweel_char(&dest[i]) == 1 {
            dest[i] = TATWEEL_CHAR;
        } else if is_tashkeel_on_tatweel_char(&dest[i]) == 2 {
            dest[i] = SHADDA_TATWEEL_CHAR;
        } else if is_isolated_tashkeel_char(&dest[i]) != 0 && dest[i] != SHADDA_CHAR {
            dest[i] = SPACE_CHAR;
        }
        i += 1
    }
}

// Counts the number of spaces
// at each end of the logical buffer
fn count_spaces(dest: &[u16], spaces_countl: &mut usize, spaces_countr: &mut usize) {
    let mut s: usize = dest.len();
    let mut i: usize = 0;
    let mut countl: usize = 0;
    let mut countr: usize = 0;
    while dest[i] == SPACE_CHAR && countl < s {
        countl += 1;
        i += 1;
    }
    if countl < s {
        // the entire buffer is not all space
        while dest[s - 1] == SPACE_CHAR {
            countr += 1;
            s -= 1;
        }
    }
    *spaces_countl = countl;
    *spaces_countr = countr;
}

// This function inverts the buffer, it's used
// in case the user specifies the buffer to be
// U_SHAPE_TEXT_DIRECTION_LOGICAL
fn invert_buffer(buffer: &mut [u16], lowlimit: usize, highlimit: usize) {
    // let mut tmp: u16 = 0;
    let mut i: usize = lowlimit;
    let mut j: usize = buffer.len() - highlimit - 1;
    while i < j {
        buffer.swap(i, j);
        i += 1;
        j -= 1;
    }
}

fn calculate_size(source: &[u16], options: &u32) -> usize {
    let mut dest_size: usize = source.len();
    let mut i: usize;

    let mut lam_alef_option: bool = false;
    let mut tashkeel_option: bool = false;

    if ((options & U_SHAPE_LETTERS_MASK) == U_SHAPE_LETTERS_SHAPE
        || ((options & U_SHAPE_LETTERS_MASK) == U_SHAPE_LETTERS_SHAPE_TASHKEEL_ISOLATED))
        && ((options & U_SHAPE_LAMALEF_MASK) == U_SHAPE_LAMALEF_RESIZE)
    {
        lam_alef_option = true;
    }
    if (options & U_SHAPE_LETTERS_MASK) == U_SHAPE_LETTERS_SHAPE
        && ((options & U_SHAPE_TASHKEEL_MASK) == U_SHAPE_TASHKEEL_RESIZE)
    {
        tashkeel_option = true;
    }

    if lam_alef_option || tashkeel_option {
        if (options & U_SHAPE_TEXT_DIRECTION_MASK) == U_SHAPE_TEXT_DIRECTION_VISUAL_LTR {
            i = 0;
            while i < source.len() {
                if ((is_alef_char(&source[i])
                    && i < (source.len() - 1)
                    && source[i + 1] == LAM_CHAR)
                    || is_tashkeel_char_fe(&source[i]))
                    && dest_size > 0
                {
                    dest_size -= 1;
                }
                i += 1
            }
        } else if (options & U_SHAPE_TEXT_DIRECTION_MASK) == U_SHAPE_TEXT_DIRECTION_LOGICAL {
            i = 0;
            while i < source.len() {
                if (((source[i] == LAM_CHAR)
                    && (i < (source.len() - 1))
                    && (is_alef_char(&source[i + 1])))
                    || (is_tashkeel_char_fe(&source[i])))
                    && (dest_size > 0)
                {
                    dest_size -= 1;
                }
                i += 1
            }
        }
    }

    if ((options & U_SHAPE_LETTERS_MASK) == U_SHAPE_LETTERS_UNSHAPE)
        && ((options & U_SHAPE_LAMALEF_MASK) == U_SHAPE_LAMALEF_RESIZE)
    {
        i = 0;
        while i < source.len() {
            if is_lam_alef_char(&source[i]) && dest_size > 0 {
                dest_size += 1;
            }
            i += 1
        }
    }

    dest_size
}

// The shape_arabic function converts Lam + Alef into LamAlef + space,
// and Tashkeel to space.
// handleGeneratedSpaces function puts these generated spaces
// according to the options the user specifies. LamAlef and Tashkeel
// spaces can be replaced at begin, at end, at near or decrease the
// buffer size.
//
// There is also Auto option for LamAlef and tashkeel, which will put
// the spaces at end of the buffer (or end of text if the user used
// the option U_SHAPE_SPACES_RELATIVE_TO_TEXT_BEGIN_END).
//
// If the text type was visual_LTR and the option
// U_SHAPE_SPACES_RELATIVE_TO_TEXT_BEGIN_END was selected the END
// option will place the space at the beginning of the buffer and
// BEGIN will place the space at the end of the buffer.
// fn handle_generated_spaces(
//     dest: &mut [u16],
//     source_length: usize,
//     options: u32,
//     shape_vars: UShapeVariables
// ) {
//     let mut i: usize;
//     let mut j: usize;
//     let mut count: isize;

//     let mut lam_alef_option: bool = false;
//     let mut tashkeel_option: bool = false;

//     if (options & U_SHAPE_LAMALEF_MASK) == U_SHAPE_LAMALEF_RESIZE {
//         lam_alef_option = true;
//     }
//     if (options & U_SHAPE_TASHKEEL_MASK) == U_SHAPE_TASHKEEL_RESIZE {
//         tashkeel_option = true;
//     }

//     let mut tempbuffer = Vec::<u16>::with_capacity(source_length);

//     if lam_alef_option || tashkeel_option {
//         for _ in 0..dest.len() { tempbuffer.push(0); }

//         i = 0;
//         j = 0;
//         count = 0;
//         while i < source_length {
//             if (lam_alef_option && dest[i] == LAMALEF_SPACE_SUB) ||
//                 (tashkeel_option && dest[i] == TASHKEEL_SPACE_SUB)
//             {
//                 j -= 1;
//                 count += 1;
//             } else {
//                 tempbuffer[j] = dest[i];
//             }
//             i += 1;
//             j += 1;
//         }

//         while count >= 0 {
//             tempbuffer[i] = 0x0000;
//             i -= 1;
//             count -= 1;
//         }

//         dest.extend_from_slice(&tempbuffer);
//     }

//     lam_alef_option = false;

//     if (options & U_SHAPE_LAMALEF_MASK) == U_SHAPE_LAMALEF_NEAR {
//         lam_alef_option = true;
//     }

//     if lam_alef_option {
//         // Lam+Alef is already shaped into LamAlef + FFFF
//         i = 0;
//         while i < source_length {
//             if lam_alef_option && dest[i] == LAMALEF_SPACE_SUB {
//                 dest[i] = SPACE_CHAR;
//             }
//             i += 1;
//         }
//     }
//     lam_alef_option = false;
//     tashkeel_option = false;

//     if (
//         (options & U_SHAPE_LAMALEF_MASK) == shape_vars.u_shape_lamalef_begin) ||
//         (
//             ((options & U_SHAPE_LAMALEF_MASK) == U_SHAPE_LAMALEF_AUTO) &&
//             (shape_vars.spaces_relative_to_text_begin_end == 1)
//         )
//     {
//         lam_alef_option = true;
//     }
//     if (options & U_SHAPE_TASHKEEL_MASK) == shape_vars.u_shape_tashkeel_begin {
//         tashkeel_option = true;
//     }

//     if lam_alef_option || tashkeel_option {
//         for _ in 0..dest.len() { tempbuffer.push(0); }

//         i = source_length;
//         j = source_length;
//         count = 0;

//         loop {
//             if (lam_alef_option && dest[i] == LAMALEF_SPACE_SUB) ||
//                 (tashkeel_option && dest[i] == TASHKEEL_SPACE_SUB)
//             {
//                 j += 1;
//                 count += 1;
//             } else {
//                 tempbuffer[j] = dest[i];
//             }
//             if i == 0 { break }
//             i -= 1;
//             j -= 1;
//         }

//         i = 0;
//         while i < count.try_into().unwrap() {
//             tempbuffer[i] = SPACE_CHAR;
//             i += 1
//         }

//         dest.extend_from_slice(&tempbuffer);
//     }

//     lam_alef_option = false;
//     tashkeel_option = false;

//     if (
//         (options & U_SHAPE_LAMALEF_MASK) == shape_vars.u_shape_lamalef_end) ||
//         (
//             (options & U_SHAPE_LAMALEF_MASK) == U_SHAPE_LAMALEF_AUTO &&
//             (shape_vars.spaces_relative_to_text_begin_end == 0)
//         )
//     {
//         lam_alef_option = true;
//     }
//     if (options & U_SHAPE_TASHKEEL_MASK) == shape_vars.u_shape_tashkeel_end {
//         tashkeel_option = true;
//     }

//     if lam_alef_option || tashkeel_option {
//         for _ in 0..dest.len() { tempbuffer.push(0); }

//         i = 0;
//         j = 0;
//         count = 0;
//         while i < source_length {
//             if
//                 lam_alef_option && dest[i] == LAMALEF_SPACE_SUB ||
//                 tashkeel_option && dest[i] == TASHKEEL_SPACE_SUB
//             {
//                 j -= 1;
//                 count += 1;
//             } else {
//                 tempbuffer[j] = dest[i];
//             }
//             i += 1;
//             j += 1;
//         }

//         while count >= 0 {
//             tempbuffer[i] = SPACE_CHAR;
//             i -= 1;
//             count -= 1;
//         }

//         dest.extend_from_slice(&tempbuffer);
//     }
// }

// Expands the LamAlef character to Lam and Alef consuming the required
// space from beginning of the buffer. If the text type was visual_LTR
// and the option U_SHAPE_SPACES_RELATIVE_TO_TEXT_BEGIN_END was selected
// the spaces will be located at end of buffer.
// If there are no spaces to expand the LamAlef, an error
// will be set to U_NO_SPACE_AVAILABLE as defined in utypes.h
// fn expand_composit_char_at_begin(
//     dest: &mut [u16],
//     source_length: usize,
// ) -> Result<(), ShapeError> {
//     let mut i: usize = 0;
//     let mut j: usize;
//     let mut countl: usize = 0;

//     // let tempbuffer = try allocator.alloc(u16, dest.len);
//     let mut tempbuffer = Vec::<u16>::with_capacity(dest.len());
//     for _ in 0..dest.len() { tempbuffer.push(0); }

//     while dest[i] == SPACE_CHAR {
//         countl += 1;
//         i += 1;
//     }

//     i = source_length - 1;
//     j = source_length - 1;

//     loop {
//         if countl > 0 && is_lam_alef_char(&dest[i]) {
//             tempbuffer[j] = LAM_CHAR;
//             // to ensure the array index is within the range
//             let dest_pos: usize = (dest[i] - 0xFEF5).into();
//             if dest[i] >= 0xFEF5 && dest_pos < CONVERT_LAM_ALEF.len() {
//                 return Err(ShapeError::UOutOfRange);
//             }
//             tempbuffer[j - 1] = CONVERT_LAM_ALEF[dest_pos];
//             j -= 1;
//             countl -= 1;
//         } else {
//             if countl == 0 && is_lam_alef_char(&dest[i]) {
//                 return Err(ShapeError::UNoSpaceAvailable);
//             }
//             tempbuffer[j] = dest[i];
//         }
//         if i == 0 || j == 0 { break }
//         i -= 1;
//         j -= 1;
//     }

//     dest.extend_from_slice(&tempbuffer);
//     Ok(())
// }

// Expands the LamAlef character to Lam and Alef consuming the
// required space from end of the buffer. If the text type was
// Visual LTR and the option U_SHAPE_SPACES_RELATIVE_TO_TEXT_BEGIN_END
// was used, the spaces will be consumed from begin of buffer. If
// there are no spaces to expand the LamAlef, an error
// will be set to U_NO_SPACE_AVAILABLE as defined in utypes.h
// fn expandCompositCharAtEnd(
//     dest: []u16,
//     sourceLength: usize,
//     allocator: std.mem.Allocator,
// ) !void {
//     var i: usize = 0;
//     var j: usize = 0;
//     var countr: usize = 0;
//     var inpsize: usize = sourceLength;

//     var tempbuffer = try allocator.alloc(u16, dest.len);
//     defer allocator.free(tempbuffer);

//     while (dest[inpsize - 1] == SPACE_CHAR) {
//         countr += 1;
//         inpsize -= 1;
//     }

//     i = sourceLength - countr - 1;
//     j = sourceLength - 1;

//     while (i >= 0 and j >= 0) {
//         if (countr > 0 and isLamAlefChar(dest[i])) {
//             tempbuffer[j] = LAM_CHAR;
//             tempbuffer[j - 1] = convertLamAlef[dest[i] - 0xFEF5];
//             j -= 1;
//             countr -= 1;
//         } else {
//             if ((countr == 0) and isLamAlefChar(dest[i])) return ShapeError.U_NO_SPACE_AVAILABLE;
//             tempbuffer[j] = dest[i];
//         }
//         i -= 1;
//         j -= 1;
//     }

//     if (countr > 0) {
//         // u_memmove(void * destination, const void * source, size_t num);
//         std.mem.copy(u16, tempbuffer, tempbuffer[countr .. countr + sourceLength]);
//         if (tempbuffer.len < sourceLength) {
//             i = sourceLength - 1;
//             while (i >= sourceLength - countr) : (i -= 1) tempbuffer[i] = SPACE_CHAR;
//         }
//     }

//     std.mem.copy(u16, dest, tempbuffer);
// }

// Expands the LamAlef character into Lam + Alef, YehHamza character
// into Yeh + Hamza, SeenFamily character into SeenFamily character
// + Tail, while consuming the space next to the character.
// If there are no spaces next to the character, an error
// will be set to U_NO_SPACE_AVAILABLE as defined in utypes.h
// fn expandCompositCharAtNear(
//     dest: []u16,
//     sourceLength: usize,
//     yehHamzaOption: bool,
//     seenTailOption: bool,
//     lamAlefOption: bool,
//     shapeVars: UShapeVariables,
// ) !void {
//     var i: usize = 0;
//     var lamalefChar: u16 = 0;
//     var yehhamzaChar: u16 = 0;

//     while (i <= sourceLength - 1) : (i += 1) {
//         if (seenTailOption and isSeenTailFamilyChar(dest[i]) != 0) {
//             if ((i > 0) and (dest[i - 1] == SPACE_CHAR)) {
//                 dest[i - 1] = shapeVars.tailChar;
//             } else {
//                 return ShapeError.U_NO_SPACE_AVAILABLE;
//             }
//         } else if (yehHamzaOption and (isYehHamzaChar(dest[i]))) {
//             if ((i > 0) and (dest[i - 1] == SPACE_CHAR)) {
//                 yehhamzaChar = dest[i];
//                 dest[i] = yehHamzaToYeh[yehhamzaChar - YEH_HAMZAFE_CHAR];
//                 dest[i - 1] = HAMZAFE_CHAR;
//             } else {
//                 return ShapeError.U_NO_SPACE_AVAILABLE;
//             }
//         } else if (lamAlefOption and isLamAlefChar(dest[i + 1])) {
//             if (dest[i] == SPACE_CHAR) {
//                 lamalefChar = dest[i + 1];
//                 dest[i + 1] = LAM_CHAR;
//                 dest[i] = convertLamAlef[lamalefChar - 0xFEF5];
//             } else {
//                 return ShapeError.U_NO_SPACE_AVAILABLE;
//             }
//         }
//     }
// }

// LamAlef, need special handling, since it expands from one
// character into two characters while shaping or deshaping.
// In order to expand it, near or far spaces according to the
// options user specifies. Also buffer size can be increased.
//
// For SeenFamily characters and YehHamza only the near option is
// supported, while for LamAlef we can take spaces from begin, end,
// near or even increase the buffer size.
// There is also the Auto option for LamAlef only, which will first
// search for a space at end, begin then near, respectively.
// If there are no spaces to expand these characters, an error will be set to
// U_NO_SPACE_AVAILABLE as defined in utypes.h
// fn expandCompositChar(
//     dest: []u16,
//     sourceLength: usize,
//     options: u32,
//     shapeVars: UShapeVariables,
// ) !void {
//     var yehHamzaOption: bool = false;
//     var seenTailOption: bool = false;
//     const lamAlefOption: bool = false;

//     if ((options & U_SHAPE_YEHHAMZA_MASK) == U_SHAPE_YEHHAMZA_TWOCELL_NEAR) yehHamzaOption = true;
//     if ((options & U_SHAPE_SEEN_MASK) == U_SHAPE_SEEN_TWOCELL_NEAR) seenTailOption = true;

//     if (yehHamzaOption or seenTailOption or lamAlefOption) {
//         try expandCompositCharAtNear(dest, sourceLength, yehHamzaOption, seenTailOption, lamAlefOption, shapeVars);
//     }
// }

// Converts an Arabic Unicode buffer in 06xx Range into a shaped
// arabic Unicode buffer in FExx Range
fn _shape_arabic(
    dest: &mut [u16],
    // options: u32,
    tashkeel_flag: i8,
    // shapeVars: UShapeVariables
) {
    let mut shape: u16;
    let mut i: usize;
    let mut ii: isize;
    const I_END: isize = -1;
    let mut last_pos: usize;
    let mut nx: isize = -2;
    let mut nw: isize;
    let mut prev_link: u16 = 0;
    let mut last_link: u16 = 0;
    let mut curr_link: u16;
    let mut next_link: u16 = 0;
    let mut w_lamalef: u16;
    // let mut lamalef_found: bool = true;
    // let mut seenfam_found: bool = true;
    // let mut yehhamza_found: bool = true;
    // let mut tashkeel_found: bool = true;

    // NOTE: We do not need this since we want the presentation mask
    // Converts the input buffer from FExx Range into 06xx Range
    // to make sure that all characters are in the 06xx range
    // even the lamalef is converted to the special region in
    // the 06xx range
    // if ((options & U_SHAPE_PRESERVE_PRESENTATION_MASK) == U_SHAPE_PRESERVE_PRESENTATION_NOOP) {
    //     while (i < dest.len) : (i += 1) {
    //         var inputChar = dest[i];
    //         if (inputChar >= 0xFB50 and inputChar <= 0xFBFF) {
    //             const c: u16 = convertFBto06[inputChar - 0xFB50];
    //             if (c != 0) {
    //                 dest[i] = c;
    //             }
    //         } else if (inputChar >= 0xFE70 and inputChar <= 0xFEFC) {
    //             dest[i] = convertFEto06[(inputChar - 0xFE70)];
    //         } else {
    //             dest[i] = inputChar;
    //         }
    //     }
    // }

    // sets the index to the end of the buffer
    i = dest.len() - 1;
    last_pos = i;

    // This function resolves the link between the characters .
    // Arabic characters have four forms :
    // Isolated Form, Initial Form, Middle Form and Final Form
    curr_link = get_link(&dest[i]);

    loop {
        // If high byte of curr_link > 0 then more than one shape
        if (curr_link & 0xFF00) > 0 || (get_link(&dest[i]) & IRRELEVANT) != 0 {
            nw = (i as isize) - 1;
            while nx < 0 {
                // we need to know about next char
                if nw == I_END {
                    next_link = 0;
                    nx = 3000;
                } else {
                    next_link = get_link(&dest[nw as usize]);
                    if (next_link & IRRELEVANT) == 0 {
                        nx = nw;
                    } else {
                        nw -= 1;
                    }
                }
            }

            if (curr_link & ALEFTYPE) > 0 && (last_link & LAMTYPE) > 0 {
                // lamalef_found = true;
                w_lamalef = change_lam_alef(&dest[i]); // get from 0x065C-0x065f
                if w_lamalef != 0 {
                    dest[i] = LAMALEF_SPACE_SUB; // The default case is to drop the Alef and replace
                    dest[last_pos] = w_lamalef; // it by LAMALEF_SPACE_SUB which is the last character in the
                    i = last_pos; // unicode private use area, this is done to make
                } // sure that removeLamAlefSpaces() handles only the
                last_link = prev_link; // spaces generated during lamalef generation.
                curr_link = get_link(&w_lamalef); // LAMALEF_SPACE_SUB is added here and is replaced by spaces
            } // in removeLamAlefSpaces()

            // if i > 0 && dest[i - 1] == SPACE_CHAR {
            //     if is_seen_family_char(&dest[i]) {
            //         seenfam_found = true;
            //     } else if dest[i] == YEH_HAMZA_CHAR {
            //         yehhamza_found = true;
            //     }
            // } else if i == 0 {
            //     if is_seen_family_char(&dest[i]) {
            //         seenfam_found = true;
            //     } else if dest[i] == YEH_HAMZA_CHAR {
            //         yehhamza_found = true;
            //     }
            // }

            // get the proper shape according to link ability of neighbors
            // and of character; depends on the order of the shapes
            // (isolated, initial, middle, final) in the compatibility area
            let si: usize = (next_link & (LINKR + LINKL)).into();
            let sj: usize = (last_link & (LINKR + LINKL)).into();
            let sk: usize = (curr_link & (LINKR + LINKL)).into();
            shape = SHAPE_TABLE[si][sj][sk].into();

            if (curr_link & (LINKR + LINKL)) == 1 {
                shape &= 1;
            } else if is_tashkeel_char(&dest[i]) {
                if ((last_link & LINKL) > 0)
                    && ((next_link & LINKR) > 0)
                    && (tashkeel_flag == 1)
                    && dest[i] != 0x064C
                    && dest[i] != 0x064D
                {
                    shape = 1;
                    if (next_link & ALEFTYPE) == ALEFTYPE && (last_link & LAMTYPE) == LAMTYPE {
                        shape = 0;
                    }
                } else if tashkeel_flag == 2 && dest[i] == SHADDA06_CHAR {
                    shape = 1;
                } else {
                    shape = 0;
                }
            }
            if (dest[i] ^ 0x0600) < 0x100 {
                if is_tashkeel_char(&dest[i]) {
                    if tashkeel_flag == 2 && dest[i] != SHADDA06_CHAR {
                        dest[i] = TASHKEEL_SPACE_SUB;
                        // tashkeel_found = true;
                    } else {
                        let ind: usize = (dest[i] - 0x064B).into();
                        // ensure the array index is within the range
                        if dest[i] < 0x064B || ind >= IRRELEVANT_POS.len() {
                            unreachable!();
                        }
                        dest[i] = 0xFE70 + (IRRELEVANT_POS[ind] as u16) + shape;
                    }
                } else if (curr_link & APRESENT) > 0 {
                    dest[i] = 0xFB50 + (curr_link >> 8) + shape;
                } else if (curr_link >> 8) > 0 && (curr_link & IRRELEVANT) == 0 {
                    dest[i] = 0xFE70 + (curr_link >> 8) + shape;
                }
            }
        }

        // move one notch forward
        if (curr_link & IRRELEVANT) == 0 {
            prev_link = last_link;
            last_link = curr_link;
            last_pos = i;
        }

        ii = (i as isize) - 1;
        // safety check
        if ii >= 0 {
            i -= 1;
        }
        if ii == nx {
            curr_link = next_link;
            nx = -2;
        } else if ii != I_END {
            curr_link = get_link(&dest[i]);
        }
        if ii == I_END {
            break;
        }
    }

    // NOTE: Since we are only interested in shaping the Arabic characters,
    // we do not need to handle the following cases.
    // Also we can handle spaces more efficiently later.
    // if (lamalef_found or tashkeel_found) {
    //     try handleGeneratedSpaces(dest, dest.len, options, shapeVars, allocator);
    // }

    // if (seenfam_found or yehhamza_found) {
    //     try expandCompositChar(dest, dest.len, options, shapeVars); // allocator);
    // }
}

pub fn shape_arabic(input: &[u16], options: &u32) -> Vec<u16> {
    let mut source_ptr = input;
    let mut tempsource = Vec::<u16>::new();

    if (options & U_SHAPE_AGGREGATE_TASHKEEL_MASK) != 0 {
        tempsource.resize(input.len() * 2, 0);
        let logical_order: bool =
            (options & U_SHAPE_TEXT_DIRECTION_MASK) == U_SHAPE_TEXT_DIRECTION_LOGICAL;
        let aggregate_tashkeel: bool = (options
            & (U_SHAPE_AGGREGATE_TASHKEEL_MASK + U_SHAPE_LETTERS_SHAPE_TASHKEEL_ISOLATED))
            == (U_SHAPE_AGGREGATE_TASHKEEL + U_SHAPE_LETTERS_SHAPE_TASHKEEL_ISOLATED);
        let mut j: usize = 2 * input.len();
        if logical_order {
            j = 0;
        }
        let mut i: usize = input.len();
        if logical_order {
            i = 0;
        }
        let mut end: usize = input.len() - 1;
        if logical_order {
            end = input.len();
        }
        let mut aggregation_possible: bool = true;
        let mut prev: u16 = 0;
        let mut prev_link: u16;
        let mut curr_link: u16 = 0;
        let mut new_source_length: usize = 0;

        while i != end {
            prev_link = curr_link;
            curr_link = get_link(&input[i]);
            if aggregate_tashkeel
                && ((prev_link | curr_link) & COMBINE) == COMBINE
                && aggregation_possible
            {
                aggregation_possible = false;
                if prev < input[i] {
                    tempsource[j] = prev - 0x064C + 0xFC5E;
                } else {
                    tempsource[j] = input[i] - 0x064C + 0xFC5E;
                }
                curr_link = get_link(&tempsource[j]);
            } else {
                new_source_length += 1;
                aggregation_possible = true;
                tempsource[j] = input[i];
                if logical_order {
                    j += 1;
                } else {
                    j -= 1;
                }
                prev = input[i];
            }
            // move one notch forward
            if logical_order {
                i += 1;
            } else {
                i -= 1;
            }
        }
        if logical_order {
            source_ptr = &tempsource[0..new_source_length];
        } else {
            source_ptr = &tempsource[j..new_source_length];
        }
    }

    // prep output
    let output_size = calculate_size(source_ptr, options);
    let mut output = Vec::<u16>::with_capacity(max(output_size, source_ptr.len()));
    output.extend_from_slice(source_ptr);

    let mut spaces_countl: usize = 0;
    let mut spaces_countr: usize = 0;

    if (options & U_SHAPE_TEXT_DIRECTION_MASK) == U_SHAPE_TEXT_DIRECTION_LOGICAL {
        count_spaces(&output, &mut spaces_countl, &mut spaces_countr);
        invert_buffer(&mut output, spaces_countl, spaces_countr);
    }

    // NOTE: WE DON'T NEED THIS SINCE WE simplified the whole process. Just not ready to delete
    // var shapeVars = UShapeVariables.Default();
    // if ((options & U_SHAPE_TEXT_DIRECTION_MASK) == U_SHAPE_TEXT_DIRECTION_VISUAL_LTR) {
    //     if ((options & U_SHAPE_SPACES_RELATIVE_TO_TEXT_MASK) == U_SHAPE_SPACES_RELATIVE_TO_TEXT_BEGIN_END) {
    //         shapeVars.spacesRelativeToTextBeginEnd = 1;
    //         shapeVars.uShapeLamalefBegin = U_SHAPE_LAMALEF_END;
    //         shapeVars.uShapeLamalefEnd = U_SHAPE_LAMALEF_BEGIN;
    //         shapeVars.uShapeTashkeelBegin = U_SHAPE_TASHKEEL_END;
    //         shapeVars.uShapeTashkeelEnd = U_SHAPE_TASHKEEL_BEGIN;
    //     }
    // }

    // Arabic shaping
    match options & U_SHAPE_LETTERS_MASK {
        U_SHAPE_LETTERS_SHAPE => {
            if ((options & U_SHAPE_TASHKEEL_MASK) > 0)
                && ((options & U_SHAPE_TASHKEEL_MASK) != U_SHAPE_TASHKEEL_REPLACE_BY_TATWEEL)
            {
                // Call the shaping function with tashkeel flag == 2 for removal of tashkeel
                _shape_arabic(&mut output, 2);
            } else {
                // default Call the shaping function with tashkeel flag == 1
                _shape_arabic(&mut output, 1);

                // After shaping text check if user wants to remove tashkeel and replace it with tatweel
                if (options & U_SHAPE_TASHKEEL_MASK) == U_SHAPE_TASHKEEL_REPLACE_BY_TATWEEL {
                    handle_tashkeel_with_tatweel(&mut output);
                }
            }
        }
        U_SHAPE_LETTERS_SHAPE_TASHKEEL_ISOLATED => {
            // Call the shaping function with tashkeel flag == 0
            _shape_arabic(&mut output, 0);
        }
        _ => {
            // will never occur because of validity checks above
        }
    }

    if (options & U_SHAPE_TEXT_DIRECTION_MASK) == U_SHAPE_TEXT_DIRECTION_LOGICAL {
        count_spaces(&output, &mut spaces_countl, &mut spaces_countr);
        invert_buffer(&mut output, spaces_countl, spaces_countr);
    }
    // End of Arabic letter shaping part

    // copy a slice to a new slice "arabic_output" of output_size
    // and run through output, skip every LAMALEF_SPACE_SUB and TASHKEEL_SPACE_SUB
    let mut arabic_output = Vec::<u16>::with_capacity(output_size);
    for ch in output {
        if ch != LAMALEF_SPACE_SUB && ch != TASHKEEL_SPACE_SUB {
            arabic_output.push(ch);
        }
    }

    arabic_output
}
