use itertools::Itertools;

pub fn run(input: &str) -> (u32, u32) {
    let mut res1 = 0;
    let mut res2 = 0;
    for line in input.lines() {
        let ranges = line.split(['-', ',']).map(|n| n.parse::<u32>().unwrap());
        let (a, b, c, d) = ranges.collect_tuple().unwrap();

        if a <= c && d <= b || c <= a && b <= d {
            res1 += 1;
        }
        if a <= d && c <= b {
            res2 += 1;
        }
    }
    (res1, res2)
}
