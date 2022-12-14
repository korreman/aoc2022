use itertools::Itertools;

fn priority(c: u8) -> u32 {
    let p = if (b'A'..=b'Z').contains(&c) {
        c - b'A' + 27
    } else if (b'a'..=b'z').contains(&c) {
        c - b'a' + 1
    } else {
        panic!()
    };
    p as u32
}

pub fn run(input: &str) -> (u32, u32) {
    let input = input.as_bytes();

    let mut a = 0;
    for line in input.split(|&b| b == b'\n') {
        let len = line.len();
        for item in &line[0..len / 2] {
            if line[len / 2..len].contains(item) {
                a += priority(*item);
                break;
            }
        }
    }

    let mut b = 0;
    for (elf_a, elf_b, elf_c) in input.split(|&b| b == b'\n').tuples() {
        for item in elf_a {
            if elf_b.contains(item) && elf_c.contains(item) {
                b += priority(*item);
                break;
            }
        }
    }
    (a, b)
}
