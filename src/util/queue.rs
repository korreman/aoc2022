use std::collections::VecDeque;

pub trait PriorityQueue<P: PartialOrd, T> {
    fn new() -> Self;
    fn add(&mut self, priority: P, item: T);
    fn next(&mut self) -> Option<(P, T)>;
}

pub struct SlidingBucketQueue<const R: usize, T> {
    offset: usize,
    items: usize,
    pub queue: VecDeque<Vec<T>>,
}

impl<const R: usize, T: Clone> PriorityQueue<usize, T> for SlidingBucketQueue<R, T> {
    fn new() -> Self {
        Self {
            offset: 0,
            items: 0,
            queue: VecDeque::from(vec![Vec::new(); R]),
        }
    }

    fn add(&mut self, priority: usize, item: T) {
        if (self.offset..self.offset + R).contains(&priority) {
            self.queue[priority - self.offset].push(item);
            self.items += 1;
        } else {
            panic!("priority {priority} outside current range");
        }
    }

    fn next(&mut self) -> Option<(usize, T)> {
        if self.items == 0 {
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
}
