use itertools::Itertools;
use std::collections::VecDeque;

pub fn run(input: &str) -> (usize, usize) {
    // parse
    let numbers: Vec<i64> = input
        .lines()
        .map(|word| word.parse::<i64>().unwrap())
        .collect();
    let decrypted_numbers = numbers.iter().map(|&n| n * 811589153).collect_vec();

    let res1 = task(numbers.as_slice(), 1);
    let res2 = task(decrypted_numbers.as_slice(), 10);
    (res1, res2)
}

fn task(numbers: &[i64], times: usize) -> usize {
    let mut state: VecDeque<(usize, usize)> = numbers
        .iter()
        .map(|n| n.rem_euclid((numbers.len() - 1) as i64) as usize)
        .enumerate()
        .collect();
    for _ in 0..times {
        for i in 0..numbers.len() {
            while state[0].0 != i {
                state.rotate_left(1);
            }
            let (id, rotation) = state.pop_front().unwrap();
            state.insert(rotation, (id, rotation));
            //println!(
            //    "{:?}",
            //    state.iter().map(|(idx, _)| numbers[*idx]).collect_vec()
            //);
        }
    }
    while state[0].1 != 0 {
        state.rotate_left(1);
    }
    [1000, 2000, 3000]
        .iter()
        .map(|n| numbers[state[n % state.len()].0])
        .sum::<i64>() as usize
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let input = "1\n2\n-3\n3\n-2\n0\n4";
        assert_eq!(super::run(input), (3, 1623178306));
    }
}
