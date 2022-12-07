use ascii::AsciiStr;

pub fn run(input: &AsciiStr) -> (u32, u32) {
    let mut elf_calories = 0;
    let mut elfs = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            elfs.push(elf_calories);
            elf_calories = 0;
        } else {
            let calories = line.as_str().parse::<u32>().unwrap();
            elf_calories += calories;
        }
    }
    elfs.sort_unstable();

    let a = *elfs.last().unwrap();
    let b = elfs.pop().unwrap() + elfs.pop().unwrap() + elfs.pop().unwrap();

    (a, b)
}
