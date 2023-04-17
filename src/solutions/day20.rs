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

    let res1 = task::<BlockSeq<u16>>(numbers.as_slice(), 1);
    let res2 = task::<BlockSeq<u16>>(decoded_numbers.as_slice(), 10);
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
    //println!("{state:?}");
    for _ in 0..rounds {
        for (n, shift) in normalized_shifts.iter().enumerate() {
            state.shift_right(n as u16, *shift);
            //println!("{state:?}");
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
    fn shift_right(&mut self, value: T, offset: usize);
    fn find(&self, value: &T) -> Option<usize>;
}

struct BlockSeq<T> {
    idxs: FxHashMap<T, usize>,
    groups: Vec<usize>,
    blocks: Vec<Vec<T>>,
}

impl<T> BlockSeq<T> {
    const N: usize = 128;
    const GROUP: usize = 8;
}

impl<T> Debug for BlockSeq<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:?}\n{:?}\n",
            self.blocks.iter().map(|b| b.len()).collect_vec(),
            self.groups
        ))
    }
}

impl<T> Index<usize> for BlockSeq<T> {
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

impl<T: Clone + Eq + Hash + Debug> Seq<T> for BlockSeq<T> {
    #[inline(always)]
    fn from<I: Iterator<Item = T>>(iter: I) -> Self {
        let mut idxs: FxHashMap<T, usize> = Default::default();
        let mut blocks = vec![Vec::with_capacity(Self::N * 2)];
        for value in iter {
            if blocks.last().unwrap().len() >= Self::N {
                blocks.push(Vec::with_capacity(Self::N * 2));
            }
            blocks.last_mut().unwrap().push(value.clone());
            idxs.insert(value, blocks.len() - 1);
        }
        let groups = (0..1 + blocks.len() / Self::GROUP)
            .map(|group| {
                let start = group * Self::GROUP;
                blocks[start..(start + Self::GROUP).min(blocks.len())]
                    .iter()
                    .map(|b| b.len())
                    .sum()
            })
            .collect();
        Self {
            idxs,
            blocks,
            groups,
        }
    }

    #[inline(always)]
    fn shift_right(&mut self, value: T, mut shift: usize) {
        // This will increment the block index with wraparound.
        let num_blocks = self.blocks.len();
        let inc = |curr: usize| (curr + 1) % num_blocks;

        // Find the value.
        let mut curr = self.idxs[&value];
        let offset = loop {
            if let Some(offset) = self.blocks[curr].iter().position(|x| x == &value) {
                break offset;
            } else {
                curr = inc(curr);
            }
        };
        // Remove the value.
        self.blocks[curr].remove(offset);
        self.groups[curr / Self::GROUP] -= 1;
        // Find the new block to place the value in.
        shift += offset;
        while shift > self.blocks[curr].len() {
            if curr % Self::GROUP == 0 && shift >= self.groups[curr / Self::GROUP] {
                shift -= self.groups[curr / Self::GROUP];
                curr += Self::GROUP;
                if curr >= num_blocks {
                    curr = 0;
                }
                continue;
            }
            shift -= self.blocks[curr].len();
            curr = inc(curr);
        }
        // Insert the new value.
        *self.idxs.get_mut(&value).unwrap() = curr;
        self.blocks[curr].insert(shift, value);
        self.groups[curr / Self::GROUP] += 1;

        // Rebalance the sequence if necessary.
        if self.blocks[curr].len() >= Self::N * 2 {
            let mut half = self.blocks[curr].split_off(Self::N);
            self.groups[curr / Self::GROUP] -= Self::N;

            curr = inc(curr);
            while self.blocks[curr].len() + half.len() >= Self::N * 2 {
                self.groups[curr / Self::GROUP] -= self.blocks[curr].len();
                self.groups[curr / Self::GROUP] += half.len();
                std::mem::swap(&mut self.blocks[curr], &mut half);
                curr = inc(curr);
            }

            self.groups[curr / Self::GROUP] += half.len();
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
