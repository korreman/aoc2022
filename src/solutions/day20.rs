pub fn run(input: &str) -> (i64, i64) {
    // parse
    let numbers: Vec<i64> = input
        .lines()
        .map(|word| word.parse::<i64>().unwrap())
        .collect();

    let decoded_numbers: Vec<i64> = numbers.iter().map(|n| n * 811_589_153).collect();

    let res1 = task(numbers.as_slice(), 1);
    let res2 = task(decoded_numbers.as_slice(), 10);
    (res1, res2)
}

fn task(numbers: &[i64], rounds: usize) -> i64 {
    let len = numbers.len();
    let mut state: Vec<u16> = (0..len as u16).collect();

    // shuffle
    for _ in 0..rounds {
        for (n, rotation) in numbers.iter().enumerate() {
            let idx = state.iter().position(|&x| x as usize == n).unwrap();
            let target = (idx as i64 + rotation).rem_euclid((len - 1) as i64) as usize;
            if target > idx {
                state[idx..=target].rotate_left(1);
            } else {
                state[target..=idx].rotate_right(1);
            }
        }
    }

    // compute result
    let zero_idx = state.iter().position(|&n| numbers[n as usize] == 0).unwrap();
    let a = numbers[state[(zero_idx + 1000) % len] as usize];
    let b = numbers[state[(zero_idx + 2000) % len] as usize];
    let c = numbers[state[(zero_idx + 3000) % len] as usize];
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
