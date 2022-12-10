use ascii::AsciiStr;
use std::fmt::{Debug, Display, Write};

pub fn run(input: &AsciiStr) -> (i32, Res2) {
    let input = input.as_bytes();
    let mut clock: i32 = 0;
    let mut reg_x: i32 = 1;
    let (mut res1, mut res2) = (0i32, [false; 240]);
    let mut idx = 0;

    let mut tick = |x: i32| {
        if x.abs_diff(clock % 40) <= 1 {
            res2[clock as usize] = true;
        }
        clock += 1;
        if clock % 40 == 20 {
            res1 += x * clock;
        }
        clock < 240
    };

    while idx < input.len() && tick(reg_x) {
        // Virtual machine
        if input[idx] == b'a' {
            tick(reg_x);
            // 1. Jump to space.
            while input[idx] != b' ' {
                idx += 1;
            }
            idx += 1;
            // 2. Check for '-' sign.
            let sign: i32 = if input[idx] == b'-' {
                idx += 1;
                -1
            } else {
                1
            };
            // 3. Parse number into v.
            let mut v = 0i32;
            while input[idx] != b'\n' {
                v *= 10;
                v += (input[idx] - b'0') as i32;
                idx += 1;
            }
            v *= sign;
            // 4. Add or subtract v from reg_x.
            reg_x += v;
        }

        while idx < input.len() && input[idx] != b'\n' {
            idx += 1;
        }
        idx += 1;
    }

    (res1, Res2 { data: res2 })
}

pub fn run_flat(input: &AsciiStr) -> (i32, Res2) {
    let (mut res1, mut res2) = (0i32, [false; 240]);

    let mut v = 0i32;
    let mut reg_x = 1i32;
    let mut neg = false;
    let mut parse = false;
    let mut clock = 0i32;

    let mut tick = |x: i32| {
        if x.abs_diff(clock % 40) <= 1 {
            res2[clock as usize] = true;
        }
        clock += 1;
        if clock % 40 == 20 {
            res1 += x * clock;
        }
    };

    for byte in input.as_bytes().iter() {
        match byte {
            b'\n' => {
                parse = false;
                tick(reg_x);
                if neg {
                    v = -v;
                    neg = false;
                }
                reg_x += v;
                v = 0;
            }
            b' ' => {
                parse = true;
                tick(reg_x)
            }
            b if parse => {
                v *= 10;
                v += (b - b'0') as i32;
            }
            _ => (),
        }
    }
    (res1, Res2 { data: res2 })
}

pub fn run_simple(input: &AsciiStr) -> (i32, Res2) {
    let mut reg_x = 1i32;
    let (mut res1, mut res2) = (0i32, [false; 240]);
    for (clock, element) in input.as_str().split_ascii_whitespace().enumerate() {
        // Result gathering
        if reg_x.abs_diff(clock as i32 % 40) <= 1 {
            res2[clock] = true;
        }
        if (clock + 1) % 40 == 20 {
            res1 += reg_x * (clock + 1) as i32;
        }
        reg_x += element.parse::<i32>().unwrap_or(0);
    }
    (res1, Res2 { data: res2 })
}

pub struct Res2 {
    data: [bool; 240],
}

impl Display for Res2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("see below")
    }
}

impl Debug for Res2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Day 10, part 2:\n")?;
        for y in 0..6 {
            for x in 0..40 {
                let c = if self.data[x + y * 40] { '█' } else { '░' };
                f.write_char(c)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = "\
            addx 15\n\
            addx -11\n\
            addx 6\n\
            addx -3\n\
            addx 5\n\
            addx -1\n\
            addx -8\n\
            addx 13\n\
            addx 4\n\
            noop\n\
            addx -1\n\
            addx 5\n\
            addx -1\n\
            addx 5\n\
            addx -1\n\
            addx 5\n\
            addx -1\n\
            addx 5\n\
            addx -1\n\
            addx -35\n\
            addx 1\n\
            addx 24\n\
            addx -19\n\
            addx 1\n\
            addx 16\n\
            addx -11\n\
            noop\n\
            noop\n\
            addx 21\n\
            addx -15\n\
            noop\n\
            noop\n\
            addx -3\n\
            addx 9\n\
            addx 1\n\
            addx -3\n\
            addx 8\n\
            addx 1\n\
            addx 5\n\
            noop\n\
            noop\n\
            noop\n\
            noop\n\
            noop\n\
            addx -36\n\
            noop\n\
            addx 1\n\
            addx 7\n\
            noop\n\
            noop\n\
            noop\n\
            addx 2\n\
            addx 6\n\
            noop\n\
            noop\n\
            noop\n\
            noop\n\
            noop\n\
            addx 1\n\
            noop\n\
            noop\n\
            addx 7\n\
            addx 1\n\
            noop\n\
            addx -13\n\
            addx 13\n\
            addx 7\n\
            noop\n\
            addx 1\n\
            addx -33\n\
            noop\n\
            noop\n\
            noop\n\
            addx 2\n\
            noop\n\
            noop\n\
            noop\n\
            addx 8\n\
            noop\n\
            addx -1\n\
            addx 2\n\
            addx 1\n\
            noop\n\
            addx 17\n\
            addx -9\n\
            addx 1\n\
            addx 1\n\
            addx -3\n\
            addx 11\n\
            noop\n\
            noop\n\
            addx 1\n\
            noop\n\
            addx 1\n\
            noop\n\
            noop\n\
            addx -13\n\
            addx -19\n\
            addx 1\n\
            addx 3\n\
            addx 26\n\
            addx -30\n\
            addx 12\n\
            addx -1\n\
            addx 3\n\
            addx 1\n\
            noop\n\
            noop\n\
            noop\n\
            addx -9\n\
            addx 18\n\
            addx 1\n\
            addx 2\n\
            noop\n\
            noop\n\
            addx 9\n\
            noop\n\
            noop\n\
            noop\n\
            addx -1\n\
            addx 2\n\
            addx -37\n\
            addx 1\n\
            addx 3\n\
            noop\n\
            addx 15\n\
            addx -21\n\
            addx 22\n\
            addx -6\n\
            addx 1\n\
            noop\n\
            addx 2\n\
            addx 1\n\
            noop\n\
            addx -10\n\
            noop\n\
            noop\n\
            addx 20\n\
            addx 1\n\
            addx 2\n\
            addx 2\n\
            addx -6\n\
            addx -11\n\
            noop\n\
            noop\n\
            noop";
        let (res1, _) = run(AsciiStr::from_ascii(input).unwrap());
        assert_eq!(res1, 13140);
    }
}
