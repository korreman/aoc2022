use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

pub trait PriorityQueue<P, T> {
    fn new() -> Self;
    fn add(&mut self, priority: P, item: T);
    fn next(&mut self) -> Option<(P, T)>;
    fn len(&self) -> usize;
}

pub struct SlidingBucketQueue<const R: usize, T> {
    offset: usize,
    count: usize,
    pub queue: VecDeque<Vec<T>>,
}

impl<const R: usize, T: Clone> PriorityQueue<usize, T> for SlidingBucketQueue<R, T> {
    fn new() -> Self {
        Self {
            offset: 0,
            count: 0,
            queue: VecDeque::from(vec![Vec::new(); R]),
        }
    }

    fn add(&mut self, priority: usize, item: T) {
        if (self.offset..self.offset + R).contains(&priority) {
            self.queue[priority - self.offset].push(item);
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
                return Some((self.offset, v));
            } else {
                self.offset += 1;
                self.queue.rotate_left(1);
            }
        }
    }

    fn len(&self) -> usize {
        self.count
    }
}

impl<T> PriorityQueue<usize, T> for BinaryHeap<Reverse<KVPair<usize, T>>> {
    fn new() -> Self {
        BinaryHeap::new()
    }

    fn add(&mut self, priority: usize, item: T) {
        self.push(Reverse(KVPair::new(priority, item)));
    }

    fn next(&mut self) -> Option<(usize, T)> {
        self.pop().map(|entry| KVPair::extract(entry.0))
    }

    fn len(&self) -> usize {
        self.len()
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
