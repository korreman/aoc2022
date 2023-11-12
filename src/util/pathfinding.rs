use crate::util::{graph::Graph, queue::Queue};
use std::mem::swap;

/// Breadth-first search on a graph structure.
///
/// If no solution is found, is_target is guaranteed to be called on all reachable cells.
///
/// * `graph` - Graph to perform search on.
/// * `start` - The node to start on.
/// * `valid_neighbor` - Allows for filtering neighbors.
/// * `is_target` - checks whether the node is our target.
///   This is guaranteed to be called at most once per node,
///   allowing this function to be used as a breadth-first traversal.
pub fn bfs<T, G: Graph<T>>(
    graph: &G,
    start: G::Node,
    valid_neighbor: impl Fn(G::Node, G::Node) -> bool,
    mut is_target: impl FnMut(usize, G::Node) -> bool,
) -> Option<usize> {
    let mut visited = graph.map(|_| false);
    let mut frontier = Vec::new();
    visited[start] = true;
    frontier.push(start);

    let mut tmp = Vec::new();
    let mut distance = 0;
    while !frontier.is_empty() {
        swap(&mut frontier, &mut tmp);
        for node in tmp.drain(..) {
            if is_target(distance, node) {
                return Some(distance);
            }
            for n in graph.neighbors(node) {
                if valid_neighbor(node, n) && !visited[n] {
                    visited[n] = true;
                    frontier.push(n);
                }
            }
        }
        distance += 1;
    }
    None
}

pub fn dijkstra<T, G, Q>(
    graph: &G,
    get_edge: impl Fn(G::Node, G::Node) -> Option<usize>,
    mut is_target: impl FnMut(usize, G::Node) -> bool,
    start: G::Node,
) -> Option<usize>
where
    G: Graph<T>,
    Q: Queue<G::Node, Priority = usize>,
{
    let mut costs = graph.map(|_| usize::MAX);
    let mut queue = Q::new();

    costs[start] = 0;
    queue.add(0, start);
    while let Some((cost, node)) = queue.next() {
        // Skip node if the cost from the queue is outdated.
        if cost != costs[node] {
            continue;
        }
        if is_target(cost, node) {
            return Some(cost);
        }
        for neighbor in graph.neighbors(node) {
            if let Some(move_cost) = get_edge(node, neighbor) {
                let neighbor_cost = cost + move_cost;
                if neighbor_cost < costs[neighbor] {
                    costs[neighbor] = neighbor_cost;
                    queue.add(neighbor_cost, neighbor);
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
    get_edge: impl Fn(G::Node, G::Node) -> Option<usize>,
    heuristic: impl Fn(G::Node) -> usize,
    mut is_target: impl FnMut(usize, G::Node) -> bool,
    start: G::Node,
) -> Option<usize>
where
    G: Graph<T>,
    Q: Queue<G::Node, Priority = usize>,
{
    let mut costs = graph.map(|_| usize::MAX);
    let mut queue = Q::new();

    costs[start] = 0;
    queue.add(heuristic(start), start);
    while let Some((priority, node)) = queue.next() {
        let cost = priority - heuristic(node);
        if cost != costs[node] {
            continue;
        }
        if is_target(cost, node) {
            return Some(cost);
        }
        for neighbor in graph.neighbors(node) {
            if let Some(move_cost) = get_edge(node, neighbor) {
                let neighbor_cost = cost + move_cost;
                if neighbor_cost < costs[neighbor] {
                    costs[neighbor] = neighbor_cost;
                    queue.add(neighbor_cost + heuristic(neighbor), neighbor);
                }
            }
        }
    }
    None
}
