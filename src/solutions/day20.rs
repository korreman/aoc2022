use std::collections::VecDeque;

pub fn run(input: &str) -> (i32, i64) {
    // parse
    let numbers: Vec<i32> = input
        .lines()
        .map(|word| word.parse::<i32>().unwrap())
        .collect();

    // part 1
    let mut state: VecDeque<(usize, i32)> = numbers.iter().cloned().enumerate().collect();
    for i in 0..numbers.len() {
        while state[0].0 != i {
            state.rotate_left(1);
        }
        let (id, rotation) = state.pop_front().unwrap();
        let offset = rotation.rem_euclid(state.len() as i32) as usize;
        state.rotate_left(offset);
        state.push_front((id, rotation));
    }
    while state[0].1 != 0 {
        state.rotate_left(1);
    }
    let res1 = [1000, 2000, 3000]
        .iter()
        .map(|n| state[(n) % state.len()].1)
        .sum();

    // part 2
    let mut state: VecDeque<(usize, i64)> = numbers
        .iter()
        .cloned()
        .map(|n| (n as i64) * 811589153)
        .enumerate()
        .collect();
    for _ in 0..10 {
        for i in 0..numbers.len() {
            while state[0].0 != i {
                state.rotate_left(1);
            }
            let (id, rotation) = state.pop_front().unwrap();
            let offset = rotation.rem_euclid(state.len() as i64) as usize;
            state.rotate_left(offset);
            state.push_front((id, rotation));
        }
    }
    while state[0].1 != 0 {
        state.rotate_left(1);
    }
    let res2 = [1000, 2000, 3000]
        .iter()
        .map(|n| state[(n) % state.len()].1)
        .sum();

    (res1, res2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let input = "1\n2\n-3\n3\n-2\n0\n4";
        assert_eq!(super::run(input), (3, 1623178306));
    }
}
