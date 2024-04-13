extern crate alloc;
use alloc::vec::Vec;
use core::cell::RefCell;


use crate::{
    Type,
    get_type,
    find_dominant_type,
    mirror_adjust_string,
};

struct Line {
    pub start: usize,
    pub end: usize,
}

struct Chunk {
    pub start: RefCell<usize>,
    pub end: RefCell<usize>,
    pub r#type: RefCell<Type>,
}
impl Chunk {
    pub fn new(start: usize, end: usize, r#type: Type) -> Chunk {
        Chunk {
            start: RefCell::new(start),
            end: RefCell::new(end),
            r#type: RefCell::new(r#type)
        }
    }

    pub fn is_type(&self, t: Type) -> bool {
        *self.r#type.borrow() == t
    }

    pub fn set_type (&self, t: Type) {
        *self.r#type.borrow_mut() = t;
    }

    pub fn set_chunk_type(&self, c: &Chunk) {
        *self.r#type.borrow_mut() = *c.r#type.borrow();
    }

    pub fn get_start(&self) -> usize {
        *self.start.borrow()
    }

    pub fn get_end(&self) -> usize {
        *self.end.borrow()
    }
}

/// Process a string of text and return a new string with the correct bidi ordering.
/// Follows https://www.unicode.org/reports/tr9/#Basic_Display_Algorithm as closely as possible.
/// Some things are not implemented, such as:
/// - Explicit embedding levels
/// - Explicit bracket control
pub fn process_bidi_text(input: &[u16]) -> Vec<u16> {
    // print input:
    let mut result = Vec::<u16>::new();
    // lines are storing [start, end] positions
    let mut lines = Vec::<Line>::new();
    // step 1: split by lines
    let mut start: usize = 0;
    let mut end: usize = 0;
    for (idx, c) in input.iter().enumerate() {
        if *c == 0x000A || *c == 0x000D {
            if start == end {
                start = idx + 1;
            } else {
                lines.push(Line{ start, end: idx });
                start = idx + 1;
            }
        }
        end = idx + 1;
    }
    // store the last line
    if start < input.len() { lines.push(Line{ start, end: input.len() }); }
    // step 2: iterate lines
    for (line_idx, line) in lines.iter().enumerate() {
        let line_str = &input[line.start..line.end];
        let mut word_chunks = Vec::<Chunk>::new();
        // s2.1: define dominant type
        let mut cur_type: Type = get_type(&line_str[0]);
        let dominant_rtl: bool = if cur_type != Type::Neutral && cur_type != Type::Weak {
            cur_type == Type::Rtl
        } else {
            find_dominant_type(line_str) == Type::Rtl
        };
        if cur_type == Type::Neutral || cur_type == Type::Weak {
            if dominant_rtl { cur_type = Type::Rtl } else { cur_type = Type::Ltr; }
        }
        // s2.2: group by RTL and LTR chunks
        start = 0;
        for (idx, u_code) in line_str.iter().enumerate() {
            let u_type = get_type(u_code);
            if u_type != cur_type {
                word_chunks.push(Chunk::new(start, idx, cur_type));
                start = idx;
                cur_type = u_type;
            }
        }
        // store the last chunk
        if start != line_str.len() {
            word_chunks.push(Chunk::new(start, line_str.len(), cur_type));
        }
        // update neutral and weak chunks
        let word_chunks_len = word_chunks.len();
        for idx in 0..word_chunks_len {
            let chunk = word_chunks.get(idx).unwrap();
            let prev: Option<&Chunk> = if idx == 0 { None } else { word_chunks.get(idx - 1) };
            let next: Option<&Chunk> = if idx == word_chunks_len - 1 { None } else { word_chunks.get(idx + 1) };
            if chunk.is_type(Type::Neutral) {
                if idx == 0 {
                    chunk.set_chunk_type(next.unwrap());
                } else if idx == word_chunks_len - 1 {
                    continue;
                } else if prev.unwrap().r#type == next.unwrap().r#type {
                    chunk.set_chunk_type(prev.unwrap());
                } else {
                    chunk.set_type(if dominant_rtl { Type::Rtl } else { Type::Ltr });
                }
            } else if
                chunk.is_type(Type::Weak) &&
                idx != 0 &&
                prev.unwrap().is_type(Type::Rtl) &&
                !dominant_rtl
            {
                // swap chunks
                word_chunks.swap(idx, idx - 1);
            }
        }
        // merge chunks that are the same
        let mut i: isize = 0;
        while i < (word_chunks.len() as isize - 1) {
            let idx: usize = i.try_into().unwrap();
            let curr = &word_chunks[idx];
            let next = &word_chunks[idx + 1];
            if curr.r#type == next.r#type {
                *curr.end.borrow_mut() = *next.end.borrow();
                word_chunks.remove(idx + 1);
                i -= 1;
            }
            i += 1
        }
        // s2.3: If RTL dominant, then reverse the whole sentence (all chunks)
        if dominant_rtl {
            word_chunks.reverse();
        }
        // s2.4: Store each part, reversing each chunk as needed
        for chunk in word_chunks {
            let mut chunk_vec = line_str[chunk.get_start()..chunk.get_end()].to_vec();
            let chunk_str = chunk_vec.as_mut_slice();
            if chunk.is_type(Type::Rtl) {
                chunk_str.reverse();
                // run through the chunk_str and check for any mirrored characters (e.g. parentheses)
                mirror_adjust_string(chunk_str);
            }
            result.extend_from_slice(chunk_str)
        }
        // TODO: use the original return (\n or \r) not just \n
        if line_idx != lines.len() - 1 { result.push(0x000A); } // \n
    }

    result.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_string() {
        // Create a Rust string
        let my_string = "Hello, 你好";
        // Encode the string as UTF-16 and obtain a slice of u16 values
        let utf16_slice: Vec<u16> = my_string.encode_utf16().collect();
        // Create a reference to the slice
        let utf16_ref: &[u16] = &utf16_slice;
        let result: &[u16] = &process_bidi_text(utf16_ref);
        assert_eq!(result, utf16_ref);
    }

    #[test]
    fn arabic_string() {
        let input_utf16_ref: &[u16] = &[65203, 65276, 65249, 1779, 1785];
        let expected_utf16_ref: &[u16] = &[1779, 1785, 65249, 65276, 65203];
        let result: &[u16] = &process_bidi_text(input_utf16_ref);
        assert_ne!(result, input_utf16_ref);
        assert_eq!(result, expected_utf16_ref);
    }

    #[test]
    fn hebrew_string() {
        let input_utf16_ref: &[u16] = &[1468, 1489];
        let expected_utf16_ref: &[u16] = &[1489, 1468];
        let result: &[u16] = &process_bidi_text(input_utf16_ref);
        assert_ne!(result, input_utf16_ref);
        assert_eq!(result, expected_utf16_ref);
    }
}
