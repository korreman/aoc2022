//use std::mem::swap;

pub fn run(input: &str) -> (u32, u32) {
    let mut elf_calories = 0;
    let mut elfs = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            elfs.push(elf_calories);
            elf_calories = 0;
        } else {
            let calories = line.parse::<u32>().unwrap();
            elf_calories += calories;
        }
    }
    elfs.sort_unstable();

    let a = *elfs.last().unwrap();
    let b = elfs.pop().unwrap() + elfs.pop().unwrap() + elfs.pop().unwrap();

    (a, b)
}

pub fn run_fast(input: &str) -> (u32, u32) {
    //let mut maxes = [0u32; 3];
    let mut max = 0;

    let mut acc = 0;
    let mut calories = 0;
    for b in input.as_bytes() {
        if *b != b'\n' {
            acc *= 10;
            acc += (b - b'0') as u32;
        } else {
            if acc > 0 {
                calories += acc;
                acc = 0;
            } else {
                max = max.max(calories);
                //for max in &mut maxes {
                //    if *max < calories {
                //        swap(max, &mut calories);
                //    }
                //}
                calories = 0;
            }
        }
    }
    (max, 0)
    //(maxes[0], maxes[0] + maxes[1] + maxes[2])
}
