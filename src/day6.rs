pub fn run(input: &str) -> (usize, usize) {
    let res1 = task(input, 4);
    let res2 = task(input, 14);
    (res1, res2)
}

fn task(input: &str, length: usize) -> usize {
    input
        .as_bytes()
        .windows(length)
        .enumerate()
        .find(|(_, window)| {
            window
                .iter()
                .fold(0u32, |acc, c| (acc | (1 << (c - b'a'))))
                .count_ones()
                == length as u32
        })
        .expect("no packet marker detected!")
        .0
}
