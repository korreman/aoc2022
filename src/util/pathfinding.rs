use super::{
    grid::{Grid, Pos},
    queue::PriorityQueue,
};

pub fn dijkstra<Q: PriorityQueue<usize, Pos>, T, C, D>(
    grid: &Grid<T>,
    start: Pos,
    cost_func: C,
    is_target: D,
) -> Option<usize>
where
    C: Fn(Pos, Pos) -> Option<usize>,
    D: Fn(Pos) -> bool,
{
    let mut costs = Grid::new_filled(grid.width(), grid.height(), usize::MAX);
    let mut queue = Q::new();

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
            if let Some(move_cost) = cost_func(pos, nb) {
                // Relax
                let nb_cost = cost + move_cost;
                if nb_cost < costs[nb] {
                    costs[nb] = nb_cost;
                    queue.add(nb_cost, nb);
                }
            }
        }
    }
    None
}
