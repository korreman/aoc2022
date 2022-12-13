use ascii::AsciiStr;
use itertools::Itertools;

pub fn run(input: &AsciiStr) -> (u32, u32) {
    let mut res1 = 0;
    let mut res2 = 0;
    for line in input
        .as_bytes()
        .split(|&b| b == b'\n')
        .take_while(|l| l.len() > 0)
    {
        let ranges = line.split(|&b| b == b'-' || b == b',').map(|n| {
            unsafe { std::str::from_utf8_unchecked(n) }
                .parse::<u32>()
                .unwrap()
        });
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
