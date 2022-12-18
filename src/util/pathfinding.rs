use crate::util::{graph::Graph, queue::Queue};
use std::mem::swap;

/// Breadth-first search on a graph structure.
/// The `is_target` closure is guaranteed to be called at most once per cell.
/// If no solution is found, is_target is guaranteed to be called on all reachable cells.
pub fn bfs<T, G: Graph<T>>(
    graph: &G,
    start: G::Handle,
    valid_neighbor: impl Fn(G::Handle, G::Handle) -> bool,
    mut is_target: impl FnMut(usize, G::Handle) -> bool,
) -> (Option<usize>, G::Map<bool>) {
    let mut visited = graph.map(|_| false);
    let mut handles = Vec::new();

    visited[start] = true;
    handles.push(start);

    let mut off_handles = Vec::new();
    let mut c = 0;
    while !handles.is_empty() {
        swap(&mut handles, &mut off_handles);
        for h in off_handles.drain(..) {
            if is_target(c, h) {
                return (Some(c), visited);
            }
            for n in graph.neighbors(h) {
                if valid_neighbor(h, n) && !visited[n] {
                    visited[n] = true;
                    handles.push(n);
                }
            }
        }
        c += 1;
    }
    (None, visited)
}

pub fn dijkstra<T, G, Q>(
    graph: &G,
    cost: impl Fn(G::Handle, G::Handle) -> Option<usize>,
    mut is_target: impl FnMut(usize, G::Handle) -> bool,
    start: G::Handle,
) -> Option<usize>
where
    G: Graph<T>,
    Q: Queue<G::Handle, Priority = usize>,
{
    let mut costs = graph.map(|_| usize::MAX);
    let mut queue = Q::new();

    costs[start] = 0;
    queue.add(0, start);
    while let Some((c, h)) = queue.next() {
        if c != costs[h] {
            continue;
        }
        if is_target(c, h) {
            return Some(c);
        }
        for n in graph.neighbors(h) {
            if let Some(move_cost) = cost(h, n) {
                let nc = c + move_cost;
                if nc < costs[n] {
                    costs[n] = nc;
                    queue.add(nc, n);
                }
            }
        }
    }
    None
}

/// NOTE: The `heuristic` must be admissible.
/// By adding it to the current cost of a node,
/// you should get an estimate for the total cost of a completed path passing through this node.
/// Calculated as `f(n) = cost(n) + heuristic(n)`.
/// It must never exceed the actual cost of the shortest path,
/// or the optimal path might be forgotten.
pub fn a_star<T, G, Q>(
    graph: &G,
    cost: impl Fn(G::Handle, G::Handle) -> Option<usize>,
    heuristic: impl Fn(G::Handle) -> usize,
    mut is_target: impl FnMut(usize, G::Handle) -> bool,
    start: G::Handle,
) -> Option<usize>
where
    G: Graph<T>,
    Q: Queue<G::Handle, Priority = usize>,
{
    let mut costs = graph.map(|_| usize::MAX);
    let mut queue = Q::new();

    costs[start] = 0;
    queue.add(heuristic(start), start);
    while let Some((p, h)) = queue.next() {
        let c = p - heuristic(h);
        if c != costs[h] {
            continue;
        }
        if is_target(c, h) {
            return Some(c);
        }
        for n in graph.neighbors(h) {
            if let Some(move_cost) = cost(h, n) {
                let nc = c + move_cost;
                if nc < costs[n] {
                    costs[n] = nc;
                    queue.add(nc + heuristic(n), n);
                }
            }
        }
    }
    None
}
