use fxhash::FxHashMap;
use itertools::Itertools;

use std::cmp::Reverse;
use std::mem::swap;

// TODO: The choice of abstractions could be better.
// The branch and bound algorithm could be generalized.
// One of our instantiations doesn't even track the bound,
// it's just a depth-first search pruning a static bound.
// We also compute the upper bound twice.
//
// So, maybe we can generalize the depth-first state space search with pruning?
// And then implement branch and bound as a specific form of pruning.
// And implement a specific bound.
//
// This only makes sense if we can use the same abstractions for day 19.

pub fn run(input: &str) -> (u16, u16) {
    let valves = parse(input);
    let graph = preprocess(valves);

    // Part 1
    let mut bound = BestBound::new();
    SearchState::new(&graph, 30, 0).branch_and_bound(&mut bound);
    let res1 = bound.lower_bound();

    // Part 2
    let mut bound1 = BestBound::new();
    SearchState::new(&graph, 26, 0).branch_and_bound(&mut bound1);

    let mut bound2 = BestBound::new();
    SearchState::new(&graph, 26, bound1.visited).branch_and_bound(&mut bound2);

    let mut complement_bound = StaticBound::new(bound2.lower_bound, graph.num_valves);
    SearchState::new(&graph, 26, 0).branch_and_bound(&mut complement_bound);
    let res2 = complement_bound.lower_bound();

    (res1, res2)
}

fn parse(input: &str) -> Vec<Valve> {
    input.lines().map(parse_valve).collect()
}

struct Valve<'a> {
    flow: u16,
    label: &'a str,
    tunnels: Vec<&'a str>,
}

fn parse_valve(line: &str) -> Valve {
    let words = line.split([' ', '=', ',', ';']).filter(|w| !w.is_empty());
    let mut words = words.skip(1);
    let label = words.next().unwrap();
    let mut words = words.skip(3);
    let flow = words.next().unwrap().parse().ok().unwrap();
    let tunnels = words.skip(4).collect_vec();
    Valve { flow, label, tunnels }
}

struct Graph {
    num_valves: usize,
    valves: Vec<u16>,
    dist_matrix: Vec<u16>,
    best_dist: u16,
}

fn preprocess(valves: Vec<Valve>) -> Graph {
    // Collect indices of labels in the sequence.
    let mut idxs = FxHashMap::default();
    // Collect valves that have non-zero flow.
    let mut valves_flow = Vec::new();
    for (idx, valve) in valves.iter().enumerate() {
        idxs.insert(valve.label, idx);
        if valve.flow > 0 {
            valves_flow.push(idx);
        }
    }
    // Rather than track branch priority at a later point,
    // we sort the branches by flow.
    valves_flow.sort_by_key(|idx| Reverse(valves[*idx].flow));
    valves_flow.push(idxs["AA"]);

    // Resolve labels
    let valves = valves
        .iter()
        .map(|valve| {
            let tunnels = valve.tunnels.iter().map(|edge| idxs[edge]).collect_vec();
            (valve.flow, tunnels)
        })
        .collect_vec();

    // Use BFS to compute distances to all nodes from flow nodes.
    let mut dist_matrix = vec![None; valves_flow.len() * valves.len()];
    for (col, &src) in valves_flow.iter().enumerate() {
        let mut handles = vec![src];
        let mut handles_other = Vec::new();
        let mut step = 0u16;
        while !handles.is_empty() {
            step += 1;
            for tgt in &handles {
                let weight = &mut dist_matrix[tgt + col * valves.len()];
                if weight.is_none() {
                    handles_other.extend_from_slice(&valves[*tgt].1);
                    *weight = Some(core::num::NonZeroU16::new(step).unwrap());
                }
            }
            handles.clear();
            swap(&mut handles, &mut handles_other);
            handles.sort();
            handles.dedup();
        }
    }

    // Sort out nodes that aren't flow nodes to generate a distance matrix.
    let dist_matrix = valves_flow
        .iter()
        .cartesian_product(0..valves_flow.len())
        .map(|(row, col)| dist_matrix[row + col * valves.len()].unwrap().get())
        .collect_vec();

    let mut best_dist = u16::MAX;
    for valve in 0..valves_flow.len() {
        for other in 0..valves_flow.len() {
            if valve == other {
                continue;
            }
            best_dist = best_dist.min(dist_matrix[valve + valves_flow.len() * other]);
        }
    }
    Graph {
        num_valves: valves_flow.len(),
        valves: valves_flow.iter().map(|idx| valves[*idx].0).collect(),
        dist_matrix,
        best_dist,
    }
}

struct SearchState<'a> {
    /// The graph of flow valves.
    graph: &'a Graph,
    /// Stack of branches representing the current path.
    stack: Vec<usize>,
    /// A bitset representing which nodes have already been visited.
    visited: u16,
    /// The next branch that we are currently considering visiting.
    next: usize,
    /// The score of the current path.
    score: u16,
    /// Steps left in the current path.
    steps_left: u16,
}

impl<'a> SearchState<'a> {
    fn new(graph: &'a Graph, steps_left: u16, previsited: u16) -> Self {
        Self {
            graph,
            stack: vec![graph.valves.len() - 1],
            visited: previsited,
            next: 0,
            score: 0,
            steps_left,
        }
    }

    #[inline(always)]
    fn branch_and_bound<B: Bound>(&mut self, bound: &mut B) {
        loop {
            if self.next < self.graph.num_valves - 1 {
                let current = *self.stack.last().unwrap();
                let next_cost = self.graph.dist_matrix[self.next + current * self.graph.num_valves];
                if next_cost <= self.steps_left && (1 << self.next) & self.visited == 0 {
                    if !bound.better(self, self.graph, next_cost) {
                        self.next += 1;
                        continue;
                    }
                    self.advance(self.graph, next_cost);
                    // TODO: This should only need to be updated when encountering a leaf,
                    // can we save some time?
                    bound.update(self);
                } else {
                    self.next += 1;
                }
            } else if self.stack.len() > 1 {
                self.backtrack(self.graph);
            } else {
                break;
            }
        }
    }

    #[inline(always)]
    fn advance(&mut self, graph: &Graph, next_cost: u16) {
        self.visited |= 1 << self.next;
        self.steps_left -= next_cost;
        self.score += graph.valves[self.next] * self.steps_left;
        self.stack.push(self.next);
        self.next = 0;
    }

    #[inline(always)]
    fn backtrack(&mut self, graph: &Graph) {
        let prev = self.stack.pop().unwrap();
        self.score -= graph.valves[prev] * self.steps_left;
        let prev_current = *self.stack.last().unwrap();
        self.steps_left += graph.dist_matrix[prev + prev_current * graph.num_valves];
        self.visited ^= 1 << prev;
        self.next = prev + 1;
    }
}

trait Bound {
    /// Update the lower bound based on the current state.
    fn update(&mut self, b: &SearchState);
    /// Check if the upper bound of the considered branch is better than the lower bound.
    fn better(&self, b: &SearchState, g: &Graph, next_cost: u16) -> bool;
    fn lower_bound(&self) -> u16;
}

struct BestBound {
    /// The current lower bound.
    lower_bound: u16,
    /// The set of visited nodes for the lower bound.
    visited: u16,
}

impl BestBound {
    fn new() -> Self {
        Self { lower_bound: 0, visited: 0 }
    }
}

impl Bound for BestBound {
    #[inline(always)]
    fn update(&mut self, b: &SearchState) {
        if self.lower_bound < b.score {
            self.lower_bound = b.score;
            self.visited = b.visited;
        }
    }

    #[inline(always)]
    fn better(&self, b: &SearchState, g: &Graph, next_cost: u16) -> bool {
        let mut bound = b.score + g.valves[b.next] * (b.steps_left - next_cost);
        let mut steps_left_hypo = b.steps_left - next_cost;
        for valve in 0..g.num_valves {
            if (1 << valve) & b.visited != 0 || valve == b.next || steps_left_hypo < g.best_dist {
                continue;
            }
            steps_left_hypo -= g.best_dist;
            bound += g.valves[valve] * steps_left_hypo;
        }
        bound > self.lower_bound
    }

    #[inline(always)]
    fn lower_bound(&self) -> u16 {
        self.lower_bound
    }
}

struct StaticBound {
    /// Tracks the best score for each set of visited valves.
    bests: Vec<u16>,
    /// Lower bound, this is never changed.
    lower_bound: u16,
}

impl StaticBound {
    fn new(best_complement: u16, num_valves: usize) -> Self {
        Self {
            bests: vec![0; usize::pow(2, num_valves as u32 - 1)],
            lower_bound: best_complement,
        }
    }
}

impl Bound for StaticBound {
    #[inline(always)]
    fn update(&mut self, b: &SearchState) {
        let visited = b.visited as usize;
        self.bests[visited] = self.bests[visited].max(b.score);
    }

    #[inline(always)]
    fn better(&self, b: &SearchState, g: &Graph, next_cost: u16) -> bool {
        let mut bound = b.score + g.valves[b.next] * (b.steps_left - next_cost);
        let mut steps_left_hypo = b.steps_left - next_cost;
        for valve in 0..g.num_valves {
            if (1 << valve) & b.visited != 0 || valve == b.next || steps_left_hypo < g.best_dist {
                continue;
            }
            steps_left_hypo -= g.best_dist;
            bound += g.valves[valve] * steps_left_hypo;
        }
        bound >= self.lower_bound
    }

    #[inline(always)]
    fn lower_bound(&self) -> u16 {
        let bests = &self.bests;
        // Collect and sort indices of bests.
        let mut best_paths: Vec<usize> = (0..bests.len()).filter(|&idx| bests[idx] > 0).collect();
        best_paths.sort_unstable_by_key(|&idx| Reverse(bests[idx]));

        // Find the pair with maximum sum that doesn't bitwise overlap.
        let mut best = 0;
        let mut iter = best_paths.iter();
        while let Some(&a) = iter.next() {
            if bests[a] * 2 <= best {
                break;
            }
            for &b in iter.clone().skip(1) {
                let score = bests[a] + bests[b];
                if score <= best {
                    break;
                } else if a & b == 0 {
                    best = score;
                }
            }
        }
        best
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let input = "\
            Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\n\
            Valve BB has flow rate=13; tunnels lead to valves CC, AA\n\
            Valve CC has flow rate=2; tunnels lead to valves DD, BB\n\
            Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE\n\
            Valve EE has flow rate=3; tunnels lead to valves FF, DD\n\
            Valve FF has flow rate=0; tunnels lead to valves EE, GG\n\
            Valve GG has flow rate=0; tunnels lead to valves FF, HH\n\
            Valve HH has flow rate=22; tunnel leads to valve GG\n\
            Valve II has flow rate=0; tunnels lead to valves AA, JJ\n\
            Valve JJ has flow rate=21; tunnel leads to valve II";
        assert_eq!(super::run(input), (1651, 1707));
    }
}
