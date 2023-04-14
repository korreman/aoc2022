use std::collections::VecDeque;

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
    // set up state
    let mut state: VecDeque<(usize, i64)> = numbers
        .iter()
        .cloned()
        .map(|n| n * multiplier)
        .enumerate()
        .collect();

    // shuffle
    for _ in 0..rounds {
        for i in 0..numbers.len() {
            while state[0].0 != i {
                state.rotate_left(1);
            }
            let (id, rotation) = state.pop_front().unwrap();
            let offset = rotation.rem_euclid(state.len() as i64) as usize;
            state.insert(offset, (id, rotation));
        }
    }

    // compute result
    while state[0].1 != 0 {
        state.rotate_left(1);
    }
    let a = state[1000 % state.len()].1;
    let b = state[2000 % state.len()].1;
    let c = state[3000 % state.len()].1;
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
