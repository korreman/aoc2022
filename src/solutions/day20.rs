use itertools::Itertools;

use std::{fmt::Debug, ops::Index};

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

fn task(shifts: &[i64], rounds: usize) -> i64 {
    let len = shifts.len();

    // normalize shifts according to len
    let normalized_shifts = shifts
        .iter()
        .map(|shift| shift.rem_euclid((shifts.len() - 1) as i64) as usize)
        .collect_vec();

    // shuffle
    let mut state = BlockSeq::new(len as u16);
    for _ in 0..rounds {
        for (n, shift) in normalized_shifts.iter().enumerate() {
            state.shift_right(n as u16, *shift);
        }
    }

    // compute result
    let zero_correspond = shifts.iter().position(|x| x == &0).unwrap() as u16;
    let zero_idx = state.find(zero_correspond).unwrap();
    let a = shifts[state[(zero_idx + 1000) % len] as usize];
    let b = shifts[state[(zero_idx + 2000) % len] as usize];
    let c = shifts[state[(zero_idx + 3000) % len] as usize];
    a + b + c
}

struct BlockSeq {
    idxs: Vec<usize>,
    groups: Vec<usize>,
    blocks: Vec<Vec<u16>>,
}

impl BlockSeq {
    const N: usize = 64;
    const GROUP: usize = 4;
}

impl Debug for BlockSeq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:?}\n{:?}\n",
            self.blocks.iter().map(|b| b.len()).collect_vec(),
            self.groups
        ))
    }
}

impl Index<usize> for BlockSeq {
    type Output = u16;

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

impl BlockSeq {
    #[inline(always)]
    fn new(len: u16) -> Self {
        let mut idxs: Vec<usize> = Vec::new();
        let mut blocks = vec![Vec::with_capacity(Self::N * 2)];
        for value in 0..len {
            if blocks.last().unwrap().len() >= Self::N {
                blocks.push(Vec::with_capacity(Self::N * 2));
            }
            blocks.last_mut().unwrap().push(value.clone());
            idxs.push(blocks.len() - 1);
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
    fn shift_right(&mut self, value: u16, mut shift: usize) {
        // Find the value.
        let mut block = self.idxs[value as usize];
        let offset = self.blocks[block].iter().position(|x| x == &value).unwrap();

        // Remove the value.
        self.blocks[block].remove(offset);
        self.groups[block / Self::GROUP] -= 1;

        // Find the new block to place the value in.
        shift += offset;
        while shift > self.blocks[block].len() {
            let group = block / Self::GROUP;
            if block % Self::GROUP == 0 && shift >= self.groups[group] {
                shift -= self.groups[group];
                block += Self::GROUP;
                if block >= self.blocks.len() {
                    block = 0;
                }
                continue;
            }
            shift -= self.blocks[block].len();
            block = (block + 1) % self.blocks.len();
        }

        // Insert the new value.
        self.idxs[value as usize] = block;
        self.blocks[block].insert(shift, value);
        self.groups[block / Self::GROUP] += 1;
    }

    #[inline(always)]
    fn find(&self, value: u16) -> Option<usize> {
        let mut idx = 0;
        for block in &self.blocks {
            if let Some(position) = block.iter().position(|x| x == &value) {
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
