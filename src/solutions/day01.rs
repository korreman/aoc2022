pub fn run(input: &str) -> (u32, u32) {
    let mut elfs: Vec<u32> = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<u32>().unwrap()).sum())
        .collect();
    elfs.sort_unstable_by(|a, b| b.cmp(a));
    (elfs[0], elfs[0] + elfs[1] + elfs[2])
}
