use super::{
    grid::{Grid, Pos},
    queue::SlidingBucketQueue,
};

#[derive(Clone)]
pub struct Dijkstra<'a, T> {
    map: &'a Grid<T>,
    costs: Grid<usize>,
    queue: SlidingBucketQueue<Pos>,
}

impl<'a, T> Dijkstra<'a, T> {
    pub fn new(map: &'a Grid<T>, range: usize, start: Pos) -> Self {
        let mut costs = Grid::new_filled(map.width(), map.height(), usize::MAX);
        costs[start] = 0;

        let mut queue = SlidingBucketQueue::new(range);
        queue.add(0, start);
        Self { map, costs, queue }
    }

    pub fn run<C, W>(&mut self, cost_func: C, win: W) -> Option<usize>
    where
        C: Fn(&T, &T) -> Option<usize>,
        W: Fn(Pos) -> bool,
    {
        while let Some((cost, pos)) = self.pop() {
            if win(pos) {
                return Some(cost);
            }
            for neighbor in self.map.neighbors(pos) {
                let x = &self.map[neighbor];
                if let Some(new_cost) = cost_func(&self.map[pos], x) {
                    self.relax(neighbor, cost + new_cost);
                }
            }
        }
        None
    }

    pub fn to_costs(self) -> Grid<usize> {
        self.costs
    }

    fn relax(&mut self, p: Pos, value: usize) {
        if let Some(cell) = self.costs.get_mut(p) {
            if value < *cell {
                *cell = value;
                assert!(self.queue.add(value, p));
            }
        }
    }

    fn pop(&mut self) -> Option<(usize, Pos)> {
        while let Some((cost, pos)) = self.queue.next() {
            if cost >= self.costs[pos] {
                return Some((cost, pos));
            }
        }
        None
    }
}
