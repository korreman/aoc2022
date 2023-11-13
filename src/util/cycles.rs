use std::hash::Hash;
use fxhash::FxHashMap;

pub struct CycleFinder<T: Clone + Eq + Hash + std::fmt::Debug> {
    // The sequence of values that we are tracking.
    history: Vec<T>,
    // Previous occurrences of elements in the sequence.
    occurrences: FxHashMap<T, Vec<usize>>,
    // Results of previous tests.
    cache: Vec<(usize, usize)>,
}

impl<T: Clone + Eq + Hash + std::fmt::Debug> CycleFinder<T> {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            occurrences: Default::default(),
            cache: Vec::new(),
        }
    }

    pub fn push(&mut self, value: T) -> Option<&[T]> {
        // Update history and occurrences of current.
        let idx = self.history.len();
        self.history.push(value.clone());
        let occurrences = self.occurrences.entry(value).or_default();
        occurrences.push(idx);
        self.cache.push((0, idx));

        // Iterate through all previous occurrences of element from last to first.
        for &occurrence in occurrences.iter().rev().skip(1) {
            let len = idx - occurrence;
            let (cached_len, cached_idx) = &mut self.cache[len];
            // If the verified cached sequence extends all the way to this one, increment it.
            // Else, the sequence is broken, overwrite with length 1.
            if *cached_idx == idx - 1 {
                *cached_len += 1;
            } else {
                *cached_len = 1;
            }
            *cached_idx = idx;
            if *cached_len == len {
                return Some(&self.history[self.history.len() - len..self.history.len()]);
            }
        }
        None
    }
}

impl<T: Clone + Eq + Hash + std::fmt::Debug> Default for CycleFinder<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test(input: &[u8], output: Option<&[u8]>) {
        let mut finder = CycleFinder::new();
        for x in &input[0..input.len() - 1] {
            finder.push(*x);
        }
        assert_eq!(finder.push(*input.last().unwrap()), output)
    }

    #[test]
    fn test1() {
        test(&[1, 1], Some(&[1]));
    }
    #[test]
    fn test2() {
        test(&[3, 1, 2, 1, 2], Some(&[1, 2]));
    }
    #[test]
    fn test3() {
        test(&[1, 2, 3, 1, 2], None);
    }
    #[test]
    fn test4() {
        test(&[1, 4, 2, 1, 2, 4], None);
    }
    #[test]
    fn test5() {
        test(&[1, 2, 3, 6, 1, 2, 3], None);
    }
    #[test]
    fn test6() {
        test(&[0, 7, 1, 2, 3, 6, 1, 2, 3, 6], Some(&[1, 2, 3, 6]));
    }
}
