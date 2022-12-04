pub fn run(input: &str) -> (u32, u32) {
    let mut res1 = 0;
    let mut res2 = 0;
    for line in input.lines() {
        let mut ranges = line.split(['-', ',']);
        let a = u32::from_str_radix(ranges.next().unwrap(), 10).unwrap();
        let b = u32::from_str_radix(ranges.next().unwrap(), 10).unwrap();
        let c = u32::from_str_radix(ranges.next().unwrap(), 10).unwrap();
        let d = u32::from_str_radix(ranges.next().unwrap(), 10).unwrap();
        if a <= c && b >= d || c <= a && d >= b {
            res1 += 1;
        }
        let elf1 = a..=b;
        let elf2 = c..=d;
        if elf1.contains(&c) || elf1.contains(&d) || elf2.contains(&a) || elf2.contains(&b) {
            res2 += 1;
        }
    }
    (res1, res2)
}
