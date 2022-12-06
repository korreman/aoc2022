pub fn run(input: &str) -> (usize, usize) {
    let res1 = task(input, 4);
    let res2 = task(input, 14);
    (res1, res2)
}

fn task(input: &str, window_size: usize) -> usize {
    input
        .as_bytes()
        .windows(window_size)
        .enumerate()
        .find_map(|(offset, bytes)| {
            let p = bytes
                .iter()
                .all(|b| bytes.iter().filter(|b2| b == *b2).count() == 1);
            if p {
                Some(offset)
            } else {
                None
            }
        })
        .expect("no packet marker detected!")
}
