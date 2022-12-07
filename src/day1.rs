pub fn run(input: &str) -> (u32, u32) {
    let mut elf_calories = 0;
    let mut elfs = Vec::new();
    for line in input.as_bytes().split(|&c| c == b'\n') {
        if line.is_empty() {
            elfs.push(elf_calories);
            elf_calories = 0;
        } else {
            let calories = unsafe { std::str::from_utf8_unchecked(line) }
                .parse::<u32>()
                .unwrap();
            elf_calories += calories;
        }
    }
    elfs.sort_unstable();

    let a = *elfs.last().unwrap();
    let b = elfs.pop().unwrap() + elfs.pop().unwrap() + elfs.pop().unwrap();

    (a, b)
}
