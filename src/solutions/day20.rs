use std::{fmt::Debug, hash::Hash, ops::Index};

use fxhash::FxHashMap;

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
fn task<S: Seq<u16>>(numbers: &[i64], rounds: usize) -> i64 {
    let len = numbers.len();
    let mut state = S::from(0..len as u16);

    // shuffle
    for _ in 0..rounds {
        for (n, rotation) in numbers.iter().enumerate() {
            let offset = rotation.rem_euclid((numbers.len() - 1) as i64) as usize;
            state.shift(n as u16, offset);
        }
    }

    // compute result
    let zero_correspond = numbers.iter().position(|x| x == &0).unwrap() as u16;
    let zero_idx = state.find(&zero_correspond).unwrap();
    let a = numbers[state[(zero_idx + 1000) % len] as usize];
    let b = numbers[state[(zero_idx + 2000) % len] as usize];
    let c = numbers[state[(zero_idx + 3000) % len] as usize];
    a + b + c
}

trait Seq<T: Clone + Eq + Debug>: Index<usize, Output = T> + Debug {
    fn from<I: Iterator<Item = T>>(iter: I) -> Self;
    fn shift(&mut self, value: T, offset: usize);
    fn find(&self, value: &T) -> Option<usize>;
}

impl<T: Clone + Eq + Debug> Seq<T> for Vec<T> {
    #[inline(always)]
    fn from<I: Iterator<Item = T>>(iter: I) -> Self {
        iter.collect()
    }

    #[inline(always)]
    fn shift(&mut self, value: T, offset: usize) {
        let idx = self.iter().position(|x| *x == value).unwrap();
        let target = (idx + offset) % (self.len() - 1);
        if target > idx {
            self[idx..=target].rotate_left(1);
        } else {
            self[target..=idx].rotate_right(1);
        }
    }

    #[inline(always)]
    fn find(&self, value: &T) -> Option<usize> {
        self.iter().position(|x| x == value)
    }
}

#[derive(Debug)]
struct BlockSeq<const N: usize, T> {
    idxs: FxHashMap<T, usize>,
    blocks: Vec<Vec<T>>,
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
            if blocks.last().unwrap().len() > N {
                blocks.push(Vec::with_capacity(N * 2));
            }
            blocks.last_mut().unwrap().push(value.clone());
            idxs.insert(value, blocks.len() - 1);
        }
        Self { idxs, blocks }
    }

    #[inline(always)]
    fn shift(&mut self, value: T, mut offset: usize) {
        let block_idx = self.idxs[&value];
        let value_idx = self.blocks[block_idx]
            .iter()
            .position(|x| x == &value)
            .unwrap();
        self.blocks[block_idx].remove(value_idx);
        offset += value_idx;
        let mut current_block = block_idx;
        while offset > self.blocks[current_block].len() {
            offset -= self.blocks[current_block].len();
            current_block = (current_block + 1) % self.blocks.len();
        }
        *self.idxs.get_mut(&value).unwrap() = current_block;
        self.blocks[current_block].insert(offset, value);
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
