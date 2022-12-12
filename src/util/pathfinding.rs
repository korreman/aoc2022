use super::{
    grid::{Grid, Pos},
    queue::SlidingBucketQueue,
};

pub fn dijkstra<T, C, D>(
    grid: &Grid<T>,
    range: usize,
    start: Pos,
    cost_func: C,
    is_target: D,
) -> Option<usize>
where
    C: Fn(&T, &T) -> Option<usize>,
    D: Fn(Pos) -> bool,
{
    let mut costs = Grid::new_filled(grid.width(), grid.height(), usize::MAX);
    let mut queue = SlidingBucketQueue::new(range);

    costs[start] = 0;
    queue.add(0, start);

    while let Some((cost, pos)) = queue.next() {
        if cost < costs[pos] {
            continue;
        }
        if is_target(pos) {
            return Some(cost);
        }
        for nb in grid.neighbors(pos) {
            let x = &grid[nb];
            if let Some(move_cost) = cost_func(&grid[pos], x) {
                // Relax
                let nb_cost = move_cost + cost;
                if nb_cost < costs[nb] {
                    costs[nb] = nb_cost;
                    assert!(queue.add(nb_cost, nb));
                }
            }
        }
    }
    None
}
