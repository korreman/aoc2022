use std::collections::VecDeque;

#[derive(Clone)]
pub struct SlidingBucketQueue<T> {
    offset: usize,
    pub queue: VecDeque<Vec<T>>,
}

impl<T: Clone> SlidingBucketQueue<T> {
    pub fn new(range: usize) -> Self {
        Self {
            offset: 0,
            queue: VecDeque::from(vec![Vec::new(); range]),
        }
    }

    pub fn add(&mut self, priority: usize, value: T) -> bool {
        if let Some(bucket) = self.queue.get_mut(priority - self.offset) {
            bucket.push(value);
            true
        } else {
            false
        }
    }

    pub fn next(&mut self) -> Option<(usize, T)> {
        let mut total_rotation = 0;
        let value = loop {
            if let Some(v) = self.queue[0].pop() {
                break v;
            } else if total_rotation < self.queue.len() {
                total_rotation += 1;
                self.offset += 1;
                self.queue.rotate_left(1);
            } else {
                return None;
            }
        };
        Some((self.offset, value))
    }
}
