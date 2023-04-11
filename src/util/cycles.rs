pub struct CycleFinder<T: PartialEq> {
    sequence: Vec<T>,
    cache: Vec<usize>,
}

impl<T: PartialEq> CycleFinder<T> {
    pub fn new() -> Self {
        Self {
            sequence: Vec::new(),
            cache: Vec::new(),
        }
    }

    pub fn push(&mut self, value: T) -> Option<&[T]> {
        self.sequence.push(value);
        self.cache.push(0);
        let len = self.sequence.len();
        for n in 0..self.sequence.len() {
            if self.sequence[len - 1 - n] == self.sequence[len - 1] {
                self.cache[n] += 1;
                if self.cache[n] == n {
                    return Some(&self.sequence[len - n..len]);
                }
            } else {
                self.cache[n] = 0;
            }
        }
        None
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
    fn tests() {
        test(&[1, 1], Some(&[1]));
        test(&[3, 1, 2, 1, 2], Some(&[1, 2]));
        test(&[1, 2, 3, 1, 2], None);
        test(&[1, 4, 2, 1, 2, 4], None);
        test(&[1, 2, 3, 6, 1, 2, 3], None);
        test(&[0, 0, 1, 2, 3, 6, 1, 2, 3, 6], Some(&[1, 2, 3, 6]));
    }
}
