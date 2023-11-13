use std::{cmp::Reverse, collections::BinaryHeap};

pub trait Priority
where
    Self: Ord,
{
    const FIRST: Self;
    const LAST: Self;
}

impl Priority for usize {
    const FIRST: Self = usize::MIN;
    const LAST: Self = usize::MAX;
}

pub trait Queue<T> {
    type Priority: Priority;
    fn new() -> Self;
    fn add(&mut self, priority: Self::Priority, item: T);
    fn next(&mut self) -> Option<(Self::Priority, T)>;
}

pub struct SlidingBucketQueue<const R: usize, T> {
    offset: usize,
    count: usize,
    pub queue: [Vec<T>; R],
}

impl<const R: usize, T: Clone> Queue<T> for SlidingBucketQueue<R, T> {
    type Priority = usize;

    fn new() -> Self {
        Self { offset: 0, count: 0, queue: [(); R].map(|_| vec![]) }
    }

    fn add(&mut self, priority: usize, item: T) {
        if (self.offset..self.offset + R).contains(&priority) {
            self.queue[(priority - self.offset) % R].push(item);
            self.count += 1;
        } else {
            panic!("priority {priority} outside current range");
        }
    }

    fn next(&mut self) -> Option<(usize, T)> {
        if self.count == 0 {
            return None;
        }
        loop {
            if let Some(v) = self.queue[0].pop() {
                self.count -= 1;
                return Some((self.offset, v));
            } else {
                self.offset += 1;
                self.queue.rotate_left(1);
            }
        }
    }
}

pub struct RadixHeap<T> {
    bottom: usize,
    lowest_nonempty: Option<usize>,
    buckets: [Vec<(usize, T)>; usize::BITS as usize + 1],
}

impl<T> RadixHeap<T> {
    fn bucket(&self, key: usize) -> usize {
        (usize::BITS - (key ^ self.bottom).leading_zeros()) as usize
    }
}

impl<T> Queue<T> for RadixHeap<T> {
    type Priority = usize;

    fn new() -> Self {
        Self {
            bottom: 0,
            lowest_nonempty: None,
            buckets: [0; usize::BITS as usize + 1].map(|_| Vec::new()),
        }
    }

    fn add(&mut self, priority: usize, item: T) {
        let bucket = self.bucket(priority);
        self.buckets[bucket].push((priority, item));
        let lowest = self.lowest_nonempty.get_or_insert(bucket);
        *lowest = (*lowest).min(bucket);
    }

    fn next(&mut self) -> Option<(usize, T)> {
        let mut min = (None, usize::MAX);
        let bucket = self.lowest_nonempty?;
        for (idx, (key, _)) in self.buckets[bucket].iter().enumerate().rev() {
            if *key == self.bottom {
                min.0 = Some(idx);
                break;
            }
            if *key < min.1 {
                min = (Some(idx), *key);
            }
        }
        let result = self.buckets[bucket].remove(min.0?);
        if self.buckets[bucket].is_empty() {
            self.lowest_nonempty = self.buckets.iter().position(|bucket| !bucket.is_empty());
        }
        if result.0 > self.bottom {
            self.bottom = result.0;
            for entry in std::mem::take(&mut self.buckets[bucket]) {
                self.add(entry.0, entry.1);
            }
        }
        Some(result)
    }
}

impl<P: Priority, T> Queue<T> for BinaryHeap<Reverse<KVPair<P, T>>> {
    type Priority = P;

    fn new() -> Self {
        BinaryHeap::new()
    }

    fn add(&mut self, priority: P, item: T) {
        self.push(Reverse(KVPair::new(priority, item)));
    }

    fn next(&mut self) -> Option<(P, T)> {
        self.pop().map(|entry| KVPair::extract(entry.0))
    }
}

pub struct KVPair<K, V> {
    key: K,
    value: V,
}

impl<K, V> KVPair<K, V> {
    fn new(key: K, value: V) -> Self {
        Self { key, value }
    }

    fn extract(self) -> (K, V) {
        (self.key, self.value)
    }
}

impl<K: PartialEq, V> PartialEq for KVPair<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(&other.key)
    }
}

impl<K: PartialOrd, V> PartialOrd for KVPair<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

impl<K: Eq, V> Eq for KVPair<K, V> {}

impl<K: Ord, V> Ord for KVPair<K, V> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn radix_heap() {
        let mut queue: RadixHeap<()> = RadixHeap::new();
        queue.add(5, ());
        queue.add(3, ());
        queue.add(7, ());
        assert_eq!(queue.next(), Some((3, ())));
        assert_eq!(queue.next(), Some((5, ())));
        assert_eq!(queue.next(), Some((7, ())));
    }
}
