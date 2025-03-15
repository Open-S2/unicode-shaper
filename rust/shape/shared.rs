/// Swaps characters in input with characters in comparitor. This is used for multiple languages
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
