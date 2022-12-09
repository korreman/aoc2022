use ascii::{AsciiChar, AsciiStr};

pub fn run(input: &AsciiStr) -> (usize, usize) {
    let (mut rope1, mut rope2) = ([(0, 0); 2], [(0, 0); 10]);
    let (mut trail1, mut trail2) = (vec![(0, 0)], vec![(0, 0)]);
    for line in input.trim_end().split(AsciiChar::LineFeed) {
        let steps = line[2..].as_str().parse::<u32>().unwrap();
        for _ in 0..steps {
            if step_rope(&mut rope1, line[0]) {
                trail1.push(*rope1.last().unwrap());
            }
            if step_rope(&mut rope2, line[0]) {
                trail2.push(*rope2.last().unwrap());
            }
        }
    }
    (count_unique(trail1), count_unique(trail2))
}

fn step_rope(rope: &mut [(i16, i16)], dir: AsciiChar) -> bool {
    match dir {
        AsciiChar::U => rope[0].1 += 1,
        AsciiChar::D => rope[0].1 -= 1,
        AsciiChar::L => rope[0].0 -= 1,
        AsciiChar::R => rope[0].0 += 1,
        other => panic!("unknown direction: {other}"),
    }
    for i in 1..rope.len() {
        let prev = rope[i - 1];
        let curr = &mut rope[i];

        let (x_delta, y_delta) = (prev.0 - curr.0, prev.1 - curr.1);
        if x_delta.abs() >= 2 || y_delta.abs() >= 2 {
            curr.0 += x_delta.signum();
            curr.1 += y_delta.signum();
        } else {
            return false;
        }
    }
    return true;
}

fn count_unique<T: Copy + Default + Ord>(mut data: Vec<T>) -> usize {
    data.sort_unstable();
    let (res, _) = data.iter().fold((0, T::default()), |(count, x), y| {
        if x == *y {
            (count, x)
        } else {
            (count + 1, *y)
        }
    });
    res
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = AsciiStr::from_ascii("R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20").unwrap();
        let (_, res2) = run(input);
        assert_eq!(res2, 36);
    }
}
