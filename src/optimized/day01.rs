use std::mem::swap;

pub fn run(input: &str) -> (u32, u32) {
    let mut maxes = [0u32; 3];
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
                for max in &mut maxes {
                    if *max < calories {
                        swap(max, &mut calories);
                    }
                }
                calories = 0;
            }
        }
    }
    (maxes[0], maxes[0] + maxes[1] + maxes[2])
}
