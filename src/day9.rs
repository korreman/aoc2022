use ascii::{AsciiChar, AsciiStr};
use itertools::Itertools;
use std::{collections::HashSet, mem::swap};

pub fn run(input: &AsciiStr) -> (usize, usize) {
    let mut rope1 = [(0, 0); 2];
    let mut rope2 = [(0, 0); 10];
    let mut trail1 = HashSet::new();
    let mut trail2 = HashSet::new();
    for line in input.lines() {
        let steps = line[2..].as_str().parse::<u32>().unwrap();
        for _ in 0..steps {
            step_rope(&mut rope1, line[0]);
            step_rope(&mut rope2, line[0]);
            trail1.insert(*rope1.last().unwrap());
            trail2.insert(*rope2.last().unwrap());
        }
    }
    (trail1.len(), trail2.len())
}

fn step_rope(rope: &mut [(i32, i32)], dir: AsciiChar) {
    match dir {
        AsciiChar::U => rope[0].1 += 1,
        AsciiChar::D => rope[0].1 -= 1,
        AsciiChar::L => rope[0].0 -= 1,
        AsciiChar::R => rope[0].0 += 1,
        other => panic!("unknown direction: {other}"),
    }
    for i in 1..rope.len() {
        let prev_knot = rope[i - 1];
        let knot = &mut rope[i];

        let x_diff = prev_knot.0 - knot.0;
        let y_diff = prev_knot.1 - knot.1;
        if x_diff.abs() < 2 && y_diff.abs() < 2 {
            break;
        }
        knot.0 += x_diff.signum();
        knot.1 += y_diff.signum();

    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = AsciiStr::from_ascii("R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20").unwrap();
        let (res1, res2) = run(input);
        assert_eq!(res2, 36);
    }
}
