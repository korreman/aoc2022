use itertools::Itertools;

pub fn run(input: &str) -> (i64, i64) {
    // parse
    let numbers: Vec<i64> = input
        .lines()
        .map(|word| word.parse::<i64>().unwrap())
        .collect();

    let res1 = task(numbers.as_slice(), 1, 1);
    let res2 = task(numbers.as_slice(), 10, 811589153);
    (res1, res2)
}

fn task(numbers: &[i64], rounds: usize, multiplier: i64) -> i64 {
    let len = numbers.len();

    // set up state
    let mut state: Vec<(usize, i64)> = numbers
        .iter()
        .map(|&n| n * multiplier)
        .enumerate()
        .collect();

    // shuffle
    for _ in 0..rounds {
        for n in 0..len {
            let (idx, (_, rotation)) = state.iter().find_position(|(sn, _)| *sn == n).unwrap();
            let target = (idx as i64 + rotation).rem_euclid((len - 1) as i64) as usize;
            if target > idx {
                state[idx..=target].rotate_left(1);
            } else {
                state[target..=idx].rotate_right(1);
            }
        }
    }

    // compute result
    while state[0].1 != 0 {
        state.rotate_left(1);
    }
    let a = state[1000 % len].1;
    let b = state[2000 % len].1;
    let c = state[3000 % len].1;
    a + b + c
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let input = "1\n2\n-3\n3\n-2\n0\n4";
        assert_eq!(super::run(input), (3, 1623178306));
    }
}
