pub fn run(input: &str) -> (usize, usize) {
    let res1 = task(input, 4);
    let res2 = task(input, 14);
    (res1, res2)
}

/// Bitset solution that skips work by jumping past windows that we already know aren't markers.
fn task(input: &str, length: usize) -> usize {
    let input = input.as_bytes();
    let mut i = 0;
    'outer: while i + length < input.len() {
        let window = &input[i..i + length];
        let mut set = 0u32;
        // Traverse the window in reverse,
        // finding the 'first' byte that is a duplicate of a previous.
        for (j, b) in window.iter().rev().enumerate() {
            let bit = 1 << (b - b'a');
            if (set & bit) == 0 {
                // If it's not a duplicate, add it to the set
                set |= bit;
            } else {
                // Otherwise move the window forward until it stands right after the duplicate.
                i += length - j;
                continue 'outer;
            }
        }
        return i + length;
    }
    panic!("no packet marker detected!");
}
