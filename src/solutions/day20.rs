use std::{fmt::Debug, hash::Hash, ops::Index};

use fxhash::FxHashMap;
use itertools::Itertools;

#[inline(always)]
pub fn run(input: &str) -> (i64, i64) {
    // parse
    let numbers: Vec<i64> = input
        .lines()
        .map(|word| word.parse::<i64>().unwrap())
        .collect();

    let decoded_numbers: Vec<i64> = numbers.iter().map(|n| n * 811_589_153).collect();

    let res1 = task::<BlockSeq<256, u16>>(numbers.as_slice(), 1);
    let res2 = task::<BlockSeq<256, u16>>(decoded_numbers.as_slice(), 10);
    (res1, res2)
}

#[inline(always)]
fn task<S: Seq<u16>>(shifts: &[i64], rounds: usize) -> i64 {
    let len = shifts.len();

    // normalize shifts according to len
    let normalized_shifts = shifts
        .iter()
        .map(|shift| shift.rem_euclid((shifts.len() - 1) as i64) as usize)
        .collect_vec();

    // shuffle
    let mut state = S::from(0..len as u16);
    println!("{state:?}");
    for _ in 0..rounds {
        for (n, shift) in normalized_shifts.iter().enumerate() {
            state.shift(n as u16, *shift);
            println!("{state:?}");
        }
    }

    // compute result
    let zero_correspond = shifts.iter().position(|x| x == &0).unwrap() as u16;
    let zero_idx = state.find(&zero_correspond).unwrap();
    let a = shifts[state[(zero_idx + 1000) % len] as usize];
    let b = shifts[state[(zero_idx + 2000) % len] as usize];
    let c = shifts[state[(zero_idx + 3000) % len] as usize];
    a + b + c
}

trait Seq<T: Clone + Eq + Debug>: Index<usize, Output = T> + Debug {
    fn from<I: Iterator<Item = T>>(iter: I) -> Self;
    fn shift(&mut self, value: T, offset: usize);
    fn find(&self, value: &T) -> Option<usize>;
}

struct BlockSeq<const N: usize, T> {
    idxs: FxHashMap<T, usize>,
    blocks: Vec<Vec<T>>,
}

impl<const N: usize, T> Debug for BlockSeq<N, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:?}",
            self.blocks.iter().map(|b| b.len()).collect_vec()
        ))
    }
}

impl<const N: usize, T> Index<usize> for BlockSeq<N, T> {
    type Output = T;

    fn index(&self, mut index: usize) -> &Self::Output {
        for block in &self.blocks {
            if index < block.len() {
                return &block[index];
            }
            index -= block.len();
        }
        panic!("index outside bounds");
    }
}

impl<const N: usize, T: Clone + Eq + Hash + Debug> Seq<T> for BlockSeq<N, T> {
    #[inline(always)]
    fn from<I: Iterator<Item = T>>(iter: I) -> Self {
        let mut idxs: FxHashMap<T, usize> = Default::default();
        let mut blocks = vec![Vec::with_capacity(N * 2)];
        for value in iter {
            if blocks.last().unwrap().len() >= N {
                blocks.push(Vec::with_capacity(N * 2));
            }
            blocks.last_mut().unwrap().push(value.clone());
            idxs.insert(value, blocks.len() - 1);
        }
        Self { idxs, blocks }
    }

    #[inline(always)]
    fn shift(&mut self, value: T, mut amount: usize) {
        // This will increment the block index with wraparound.
        let len = self.blocks.len();
        let inc = |curr: usize| (curr + 1) % len;

        // Find the value.
        let mut curr = self.idxs[&value];
        let offset = loop {
            if let Some(offset) = self.blocks[curr].iter().position(|x| x == &value) {
                break offset;
            } else {
                curr = inc(curr);
            }
        };
        // Remove The value.
        self.blocks[curr].remove(offset);
        // Find the new block to place the value in.
        amount += offset;
        while amount > self.blocks[curr].len() {
            amount -= self.blocks[curr].len();
            curr = inc(curr);
        }
        // Insert the new value.
        *self.idxs.get_mut(&value).unwrap() = curr;
        self.blocks[curr].insert(amount, value);

        // Rebalance the sequence if necessary.
        if self.blocks[curr].len() >= N * 2 {
            let mut half = self.blocks[curr].split_off(N);
            curr = inc(curr);
            while self.blocks[curr].len() >= N {
                std::mem::swap(&mut self.blocks[curr], &mut half);
                curr = inc(curr);
            }
            half.append(&mut self.blocks[curr]);
            std::mem::swap(&mut self.blocks[curr], &mut half);
        }
    }

    #[inline(always)]
    fn find(&self, value: &T) -> Option<usize> {
        let mut idx = 0;
        for block in &self.blocks {
            if let Some(position) = block.iter().position(|x| x == value) {
                return Some(idx + position);
            } else {
                idx += block.len();
            }
        }
        None
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
