use std::collections::HashSet;

pub fn run(input: &str) -> (usize, usize) {
    let res1 = task(input, 4);
    let res2 = task(input, 14);
    (res1, res2)
}

/// Sliding window, generate a bitset and check its length.
fn task(input: &str, length: usize) -> usize {
    length + input
        .as_bytes()
        .windows(length)
        .position(|window| HashSet::<&u8>::from_iter(window.iter()).len() == length)
        .expect("no packet marker detected!")
        + length
}
