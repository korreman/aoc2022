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
        // Skip if the node has been further relaxed.
        if cost != costs[pos] {
            continue;
        }
        // Finish if the target condition is allowed.
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

// problem: currently the priority is compared against the cost
/// NOTE: The heuristic must be admissible. Google it.
pub fn a_star<Q: PriorityQueue<usize, Pos>, T, C, H, D>(
    grid: &Grid<T>,
    start: Pos,
    cost_func: C,
    heuristic: H,
    is_target: D,
) -> Option<usize>
where
    C: Fn(Pos, Pos) -> Option<usize>,
    H: Fn(Pos) -> usize,
    D: Fn(Pos) -> bool,
{
    let mut costs = Grid::new_filled(grid.width(), grid.height(), usize::MAX);
    let mut queue = Q::new();

    costs[start] = 0;
    queue.add(heuristic(start), start);

    while let Some((priority, pos)) = queue.next() {
        let cost = priority - heuristic(pos);
        if cost != costs[pos] {
            continue;
        }
        if is_target(pos) {
            costs.for_each(|_, c| if *c == usize::MAX {*c = 0} else {*c = 1});
            return Some(cost);
        }
        for nebo in grid.neighbors(pos) {
            if let Some(move_cost) = cost_func(pos, nebo) {
                // Relax
                let nebo_cost = cost + move_cost;
                if nebo_cost < costs[nebo] {
                    costs[nebo] = nebo_cost;
                    queue.add(nebo_cost + heuristic(nebo), nebo);
                }
            }
        }
    }
    None
}
