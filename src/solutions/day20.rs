use std::ops::Index;

#[inline(always)]
pub fn run(input: &str) -> (i64, i64) {
    // parse
    let numbers: Vec<i64> = input
        .lines()
        .map(|word| word.parse::<i64>().unwrap())
        .collect();

    let decoded_numbers: Vec<i64> = numbers.iter().map(|n| n * 811_589_153).collect();

    let res1 = task::<Vec<u16>>(numbers.as_slice(), 1);
    let res2 = task::<Vec<u16>>(decoded_numbers.as_slice(), 10);
    (res1, res2)
}

#[inline(always)]
fn task<S: Seq<u16>>(numbers: &[i64], rounds: usize) -> i64 {
    let len = numbers.len();
    let mut state = S::from(0..len as u16);

    // shuffle
    for _ in 0..rounds {
        for (n, rotation) in numbers.iter().enumerate() {
            state.shift(n as u16, *rotation);
        }
    }

    // compute result
    let zero_idx = state
        .as_iter()
        .position(|&n| numbers[n as usize] == 0)
        .unwrap();
    let a = numbers[state[(zero_idx + 1000) % len] as usize];
    let b = numbers[state[(zero_idx + 2000) % len] as usize];
    let c = numbers[state[(zero_idx + 3000) % len] as usize];
    a + b + c
}

trait Seq<T: Clone + Eq>: Index<usize, Output = T> {
    fn from<I: Iterator<Item = T>>(iter: I) -> Self;
    fn shift(&mut self, value: T, offset: i64);
    type Iter<'a>: Iterator<Item = &'a T>
    where
        Self: 'a,
        T: 'a;
    fn as_iter(&self) -> Self::Iter<'_>;
}

impl<T: Clone + Eq> Seq<T> for Vec<T> {
    #[inline(always)]
    fn from<I: Iterator<Item = T>>(iter: I) -> Self {
        iter.collect()
    }

    #[inline(always)]
    fn shift(&mut self, value: T, offset: i64) {
        let idx = self.iter().position(|x| *x == value).unwrap();
        let target = (idx as i64 + offset).rem_euclid((self.len() - 1) as i64) as usize;
        if target > idx {
            self[idx..=target].rotate_left(1);
        } else {
            self[target..=idx].rotate_right(1);
        }
    }

    type Iter<'a> = std::slice::Iter<'a, T> where T: 'a;
    #[inline(always)]
    fn as_iter(&self) -> Self::Iter<'_> {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let input = "1\n2\n-3\n3\n-2\n0\n4";
        assert_eq!(super::run(input), (3, 1623178306));
    }
}
