use std::{ops::{Index, IndexMut}, mem::swap};

use crate::util::queue::Queue;

pub trait Graph {
    type Handle: Copy;
    type Inner<T>: GraphInner<Self::Handle, T>;
    fn map<T, U, F: FnMut(T) -> U>(graph: &Self::Inner<T>, f: F) -> Self::Inner<U>;
}

pub trait GraphInner<H, T>
where
    Self: Index<H, Output = T> + IndexMut<H, Output = T>,
{
    type Nodes: Iterator<Item = H>;
    fn neighbors(&self, handle: &H) -> Self::Nodes;
}

pub fn bfs<T, G: Graph>(
    graph: &G::Inner<T>,
    start: G::Handle,
    is_target: impl Fn(G::Handle) -> bool,
) -> Option<usize> {
    let mut visited = G::map(graph, |_| false);
    let mut handles = Vec::new();
    let mut off_handles = Vec::new();
    handles.push(start);
    let mut i = 0;
    while !handles.is_empty() {
        i += 1;
        swap(&mut handles, &mut off_handles);
        for h in off_handles.drain(..) {
            if is_target(h) {
                return Some(i);
            }
            for n in graph.neighbors(&h) {
                if !visited[n] {
                    visited[n] = true;
                    handles.push(n);
                }
            }
        }
    }
    None
}

pub fn dijkstra<T, G, Q>(
    graph: &G::Inner<T>,
    cost: impl Fn(G::Handle, G::Handle) -> Option<usize>,
    is_target: impl Fn(G::Handle) -> bool,
    start: G::Handle,
) -> Option<usize>
where
    G: Graph,
    Q: Queue<G::Handle, Priority = usize>,
{
    let mut costs: G::Inner<usize> = G::map(graph, |_| usize::MAX);
    let mut queue = Q::new();

    costs[start] = 0;
    queue.add(0, start);

    while let Some((c, h)) = queue.next() {
        if c != costs[h] {
            continue;
        }
        if is_target(h) {
            return Some(c);
        }
        for n in graph.neighbors(&h) {
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
    graph: &G::Inner<T>,
    cost: impl Fn(G::Handle, G::Handle) -> Option<usize>,
    heuristic: impl Fn(G::Handle) -> usize,
    is_target: impl Fn(G::Handle) -> bool,
    start: G::Handle,
) -> Option<usize>
where
    G: Graph,
    Q: Queue<G::Handle, Priority = usize>,
{
    let mut costs: G::Inner<usize> = G::map(graph, |_| usize::MAX);
    let mut queue = Q::new();

    costs[start] = 0;
    queue.add(heuristic(start), start);

    while let Some((p, h)) = queue.next() {
        let c = p - heuristic(h);
        if c != costs[h] {
            continue;
        }
        if is_target(h) {
            return Some(c);
        }
        for n in graph.neighbors(&h) {
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
