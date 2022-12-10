use ascii::AsciiStr;
use std::fmt::{Debug, Display, Write};

pub fn run(input: &AsciiStr) -> (i32, Res2) {
    let mut reg_x = 1i32;
    let (mut res1, mut res2) = (0i32, [false; 240]);
    for (clock, element) in input.as_str().split_ascii_whitespace().enumerate() {
        if (clock + 1) % 40 == 20 {
            res1 += reg_x * (clock + 1) as i32;
        }
        if reg_x.abs_diff(clock as i32 % 40) <= 1 {
            res2[clock] = true;
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
            addx 15
            addx -11
            addx 6
            addx -3
            addx 5
            addx -1
            addx -8
            addx 13
            addx 4
            noop
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx -35
            addx 1
            addx 24
            addx -19
            addx 1
            addx 16
            addx -11
            noop
            noop
            addx 21
            addx -15
            noop
            noop
            addx -3
            addx 9
            addx 1
            addx -3
            addx 8
            addx 1
            addx 5
            noop
            noop
            noop
            noop
            noop
            addx -36
            noop
            addx 1
            addx 7
            noop
            noop
            noop
            addx 2
            addx 6
            noop
            noop
            noop
            noop
            noop
            addx 1
            noop
            noop
            addx 7
            addx 1
            noop
            addx -13
            addx 13
            addx 7
            noop
            addx 1
            addx -33
            noop
            noop
            noop
            addx 2
            noop
            noop
            noop
            addx 8
            noop
            addx -1
            addx 2
            addx 1
            noop
            addx 17
            addx -9
            addx 1
            addx 1
            addx -3
            addx 11
            noop
            noop
            addx 1
            noop
            addx 1
            noop
            noop
            addx -13
            addx -19
            addx 1
            addx 3
            addx 26
            addx -30
            addx 12
            addx -1
            addx 3
            addx 1
            noop
            noop
            noop
            addx -9
            addx 18
            addx 1
            addx 2
            noop
            noop
            addx 9
            noop
            noop
            noop
            addx -1
            addx 2
            addx -37
            addx 1
            addx 3
            noop
            addx 15
            addx -21
            addx 22
            addx -6
            addx 1
            noop
            addx 2
            addx 1
            noop
            addx -10
            noop
            noop
            addx 20
            addx 1
            addx 2
            addx 2
            addx -6
            addx -11
            noop
            noop
            noop";
        let (res1, _) = run(AsciiStr::from_ascii(input).unwrap());
        assert_eq!(res1, 13140);
    }
}
