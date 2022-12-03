use itertools::Itertools;

fn priority(c: u8) -> u32 {
    let p = if (0x41..0x5b).contains(&c) {
        c - 0x41 + 27
    } else if (0x61..0x7b).contains(&c) {
        c - 0x61 + 1
    } else {
        panic!()
    };
    p as u32
}

pub fn run(input: &str) -> (u32, u32) {
    let mut a = 0;
    for line in input.lines() {
        let line = line.as_bytes();
        let len = line.len();
        for item in &line[0..len / 2] {
            if line[len / 2..len].contains(item) {
                a += priority(*item);
                break;
            }
        }
    }

    let mut b = 0;
    for (elf_a, elf_b, elf_c) in input.lines().tuples() {
        for item in elf_a.as_bytes() {
            if elf_b.bytes().contains(item) && elf_c.bytes().contains(item) {
                b += priority(*item);
                break;
            }
        }
    }
    (a, b)
}
