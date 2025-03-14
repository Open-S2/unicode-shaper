pub mod arabic;
pub mod buginese;
mod internal;
pub mod javanese;
pub mod khmer;
pub mod myanmar;
pub mod tibetan;

use crate::*;
use alloc::vec::Vec;
pub use arabic::*;
pub use buginese::*;
pub use internal::*;
pub use javanese::*;
pub use khmer::*;
pub use myanmar::*;
pub use tibetan::*;

pub fn shared_shaper(input: &mut [u16], comparitor: &[u16]) {
    for i in 0..input.len() {
        if i == 0 {
            continue;
        }
        if comparitor.contains(&input[i]) {
            input.swap(i - 1, i);
        }
    }
}

pub fn shape_tamil(input: &mut [u16]) {
    shared_shaper(input, &TAMIL_VOWELS);
}

// Converts an Arabic Unicode buffer in 06xx Range into a shaped
// arabic Unicode buffer in FExx Range
pub fn shape_unicode(source: &[u16], options: &u32) -> Vec<u16> {
    let mut output = source.to_vec();

    // all other shaping
    if options & U_SHAPE_LETTERS_MASK != 0 {
        // arabic shaping
        output = shape_arabic(&output, options);
        // Buginese shaping
        shape_buginese(&mut output);
        // Javanese shaping
        shape_javanese(&mut output);
        // Myanmar shaping
        shape_myanmar(&mut output);
        // Tamil shaping
        shape_tamil(&mut output);
        // Tibetan shaping
        shape_tibetan(&mut output);
        // khmer
        shape_khmer(&mut output);
    }

    // if option to process bidirectional text is set, then reorder the output
    if (options & U_SHAPE_DIRECTION_OUTPUT_BIDI) != 0 {
        return process_bidi_text(&output);
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{string::String, vec::Vec};

    const DEFAULT_OPTIONS: u32 = (U_SHAPE_LETTERS_SHAPE & U_SHAPE_LETTERS_MASK)
        | (U_SHAPE_TEXT_DIRECTION_LOGICAL & U_SHAPE_TEXT_DIRECTION_MASK)
        | U_SHAPE_DIRECTION_OUTPUT_BIDI;

    fn reverse_string(input: &str) -> String {
        let mut chars: Vec<char> = input.chars().collect();
        chars.reverse();
        chars.into_iter().collect()
    }

    #[test]
    fn basic_string() {
        // Create a Rust string
        let my_string = "normal latin text";
        // Encode the string as UTF-16 and obtain a slice of u16 values
        let utf16_slice: Vec<u16> = my_string.encode_utf16().collect();
        // Create a reference to the slice
        let utf16_ref: &[u16] = &utf16_slice;
        // Print the original UTF-16 representation
        let result: &[u16] = &shape_unicode(utf16_ref, &DEFAULT_OPTIONS);
        // Print the result of shape_unicode
        assert_eq!(result, utf16_ref);
    }

    #[test]
    fn arabic_string() {
        // Create a Rust string
        let input = "سلام۳۹";
        let expected = "۳۹ﻡﻼﺳ";
        // Encode the string as UTF-16 and obtain a slice of u16 values
        let input_utf16_slice: Vec<u16> = input.encode_utf16().collect();
        let expected_utf16_slice: Vec<u16> = expected.encode_utf16().collect();
        // Create a reference to the slice
        let input_utf16_ref: &[u16] = &input_utf16_slice;
        let expected_utf16_ref: &[u16] = &expected_utf16_slice;
        let result: &[u16] = &shape_unicode(input_utf16_ref, &DEFAULT_OPTIONS);
        assert_ne!(result, input_utf16_ref);
        assert_eq!(result, expected_utf16_ref);
    }

    #[test]
    fn hebrew_string() {
        // Create a Rust string
        let input = "ישראל"; // 1500, 1488, 1512, 1513, 1497
        let expected = reverse_string(input); // 1497, 1513, 1512, 1488, 1500
                                              // Encode the string as UTF-16 and obtain a slice of u16 values
        let input_utf16_slice: Vec<u16> = input.encode_utf16().collect();
        let expected_utf16_slice: Vec<u16> = expected.encode_utf16().collect();
        // Create a reference to the slice
        let input_utf16_ref: &[u16] = &input_utf16_slice;
        let expected_utf16_ref: &[u16] = &expected_utf16_slice;
        let result: &[u16] = &shape_unicode(input_utf16_ref, &DEFAULT_OPTIONS);
        assert_eq!(result, expected_utf16_ref);
    }

    // #[test]
    // fn khmer_test() {
    //     // Create a Rust string
    //     let input = "ព្រ"; // 6038, 6098, 6042
    //     let expected: &[u16] = &[6098, 6042, 6038];
    //     // Encode the string as UTF-16 and obtain a slice of u16 values
    //     let input_utf16_slice: Vec<u16> = input.encode_utf16().collect();
    //     // Create a reference to the slice
    //     let input_utf16_ref: &[u16] = &input_utf16_slice;
    //     let result: &[u16] = &shape_unicode(input_utf16_ref, &DEFAULT_OPTIONS);
    //     assert_eq!(result, expected);
    // }

    #[test]
    fn hebrew_degesh_test() {
        // Create a Rust string
        let input = "בּ"; // 1468, 1489
        let expected: &[u16] = &[1468, 1489];
        // Encode the string as UTF-16 and obtain a slice of u16 values
        let input_utf16_slice: Vec<u16> = input.encode_utf16().collect();
        // Create a reference to the slice
        let input_utf16_ref: &[u16] = &input_utf16_slice;
        let result: &[u16] = &shape_unicode(input_utf16_ref, &DEFAULT_OPTIONS);
        assert_eq!(result, expected);
    }

    #[test]
    fn myanmar_test() {
        // Create a Rust string
        // DEC: 4100, 4154, 4153, 4096, 4153, 4096, 4155, 4156, 4157, 4158, 4145, 4141, 4143, 4151, 4154, 4140, 4158, 4142, 4151, 4196, 4146, 4150, 4151, 4152, 4237
        // HEX: 1004, 103A, 1039, 1000, 1039, 1000, 103B, 103C, 103D, 1031, 1031, 102D, 102F, 1036, 102C, 1036
        let input = "င်္က္ကျြွှေို့်ာှီ့ၤဲံ့းႍ";
        let expected: &[u16] = &[
            4145, 4156, 4096, 4100, 4154, 4153, 4153, 4096, 4155, 4157, 4158, 4141, 4143, 4151,
            4154, 4140, 4158, 4142, 4151, 4196, 4146, 4150, 4151, 4152, 4237,
        ];
        // Encode the string as UTF-16 and obtain a slice of u16 values
        let input_utf16_slice: Vec<u16> = input.encode_utf16().collect();
        // Create a reference to the slice
        let input_utf16_ref: &[u16] = &input_utf16_slice;
        let result: &[u16] = &shape_unicode(input_utf16_ref, &DEFAULT_OPTIONS);
        assert_eq!(result, expected);
    }

    #[test]
    fn myanmar_complex_2_test() {
        let input: &[u16] = &[
            0x1004, 0x103A, 0x1039, 0x1000, 0x1039, 0x1000, 0x103B, 0x103C, 0x103D, 0x1031, 0x1031,
            0x102D, 0x102F, 0x1036, 0x102C, 0x1036,
        ];
        let expected: &[u16] = &[
            0x1031, 0x1031, 0x103C, 0x1000, 0x1004, 0x103A, 0x1039, 0x1039, 0x1000, 0x103B, 0x103D,
            0x102D, 0x1036, 0x102F, 0x102C, 0x1036,
        ];
        let result: &[u16] = &shape_unicode(input, &DEFAULT_OPTIONS);
        assert_eq!(result, expected);
    }

    #[test]
    fn tibetan_test() {
        let input = "བོད་རང་སྐྱོང་ལྗོངས།";
        let expected: &[u16] = &[
            3964, 3926, 3921, 3851, 3938, 3908, 3851, 3964, 3942, 3984, 4017, 3908, 3851, 3964,
            3939, 3991, 3908, 3942, 3853,
        ];
        // Encode the string as UTF-16 and obtain a slice of u16 values
        let input_utf16_slice: Vec<u16> = input.encode_utf16().collect();
        let result: &[u16] = &shape_unicode(&input_utf16_slice, &DEFAULT_OPTIONS);
        assert_eq!(result, expected);
    }

    #[test]
    fn buginese_test() {
        let input = "ᨑᨗ ᨍᨍᨗᨕᨂᨗ";
        let expected: &[u16] = &[6673, 6679, 32, 6669, 6669, 6679, 6677, 6658, 6679];
        // Encode the string as UTF-16 and obtain a slice of u16 values
        let input_utf16_slice: Vec<u16> = input.encode_utf16().collect();
        // Create a reference to the slice
        let input_utf16_ref: &[u16] = &input_utf16_slice;

        let result: &[u16] = &shape_unicode(input_utf16_ref, &DEFAULT_OPTIONS);
        assert_eq!(result, expected);
    }

    #[test]
    fn javanese_test() {
        let input = "ꦧꦺꦲꦏ꧀ꦠꦸꦩꦿꦥ꧀ꦲ";
        let expected: &[u16] =
            &[43450, 43431, 43442, 43407, 43456, 43424, 43448, 43433, 43455, 43429, 43456, 43442];
        // Encode the string as UTF-16 and obtain a slice of u16 values
        let input_utf16_slice: Vec<u16> = input.encode_utf16().collect();
        // Create a reference to the slice
        let input_utf16_ref: &[u16] = &input_utf16_slice;

        let result: &[u16] = &shape_unicode(input_utf16_ref, &DEFAULT_OPTIONS);
        assert_eq!(result, expected);
    }
}
