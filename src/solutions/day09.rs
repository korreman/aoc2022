pub fn run(input: &str) -> (usize, usize) {
    let mut rope = [(0, 0); 10];
    let (mut trail1, mut trail2) = (vec![(0, 0)], vec![(0, 0)]);
    for line in input.lines() {
        let steps = line[2..].parse::<u32>().unwrap();
        for _ in 0..steps {
            let moved_length = step_rope(&mut rope, line.as_bytes()[0]);
            if moved_length > 1 {
                trail1.push(rope[1]);
            }
            if moved_length > 9 {
                trail2.push(rope[9]);
            }
        }
    }
    // We transmute the (i16, i16) elements to u32 instead,
    // allowing the uniqueness counter to save some energy sorting them.
    let trail1: Vec<u32> = unsafe { std::mem::transmute(trail1) };
    let trail2: Vec<u32> = unsafe { std::mem::transmute(trail2) };
    (count_unique(trail1), count_unique(trail2))
}

fn step_rope(rope: &mut [(i16, i16)], dir: u8) -> u8 {
    match dir {
        b'U' => rope[0].1 += 1,
        b'D' => rope[0].1 -= 1,
        b'L' => rope[0].0 -= 1,
        b'R' => rope[0].0 += 1,
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
            return i as u8;
        }
    }
    return rope.len() as u8;
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
    res + 1
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";
        let (_, res2) = run(input);
        assert_eq!(res2, 36);
    }
}
