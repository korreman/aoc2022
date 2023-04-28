use fxhash::FxHashMap;
use itertools::Itertools;

use std::cmp::Reverse;
use std::mem::swap;

pub fn run(input: &str) -> (u16, u16) {
    let valves = parse(input);
    let graph = preprocess(valves);

    let bound1 = BranchState::new(&graph, 30).branch_and_bound::<BestBound>();
    let res1 = bound1.best;

    let bound2 = BranchState::new(&graph, 26).branch_and_bound::<ComplementBound>();
    let res2 = best_pair(&bound2.bests);

    (res1, res2)
}

struct Valve<'a> {
    flow: u16,
    label: &'a str,
    tunnels: Vec<&'a str>,
}

fn parse(input: &str) -> Vec<Valve> {
    input.lines().map(parse_valve).collect_vec()
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

struct BranchState<'a> {
    graph: &'a Graph,
    visited: u16,
    stack: Vec<u16>,
    next: u16,
    score: u16,
    steps_left: u16,
}

impl<'a> BranchState<'a> {
    fn new(graph: &'a Graph, steps_left: u16) -> Self {
        Self {
            graph,
            visited: 0,
            stack: vec![(graph.valves.len() - 1) as u16],
            next: 0,
            score: 0,
            steps_left,
        }
    }

    fn branch_and_bound<B: Bound>(&mut self) -> B {
        let mut bound = B::new(self.graph.num_valves as u32);
        loop {
            if self.next < self.graph.num_valves as u16 - 1 {
                let current = *self.stack.last().unwrap();
                let next_cost = self.graph.dist_matrix
                    [self.next as usize + current as usize * self.graph.num_valves];
                if next_cost <= self.steps_left && (1 << self.next) & self.visited == 0 {
                    if !bound.better(&self, &self.graph, next_cost) {
                        self.next += 1;
                        continue;
                    }
                    self.advance(&self.graph, next_cost);
                    bound.update(&self);
                } else {
                    self.next += 1;
                }
            } else if self.stack.len() > 1 {
                self.backtrack(&self.graph);
            } else {
                break;
            }
        }
        bound
    }

    #[inline(always)]
    fn advance(&mut self, graph: &Graph, next_cost: u16) {
        self.visited |= 1 << self.next;
        self.steps_left -= next_cost;
        self.score += graph.valves[self.next as usize] * self.steps_left;
        self.stack.push(self.next);
        self.next = 0;
    }

    #[inline(always)]
    fn backtrack(&mut self, graph: &Graph) {
        let prev = self.stack.pop().unwrap();
        self.score -= graph.valves[prev as usize] * self.steps_left;
        let prev_current = *self.stack.last().unwrap();
        self.steps_left +=
            graph.dist_matrix[prev as usize + prev_current as usize * graph.num_valves];
        self.visited ^= 1 << prev;
        self.next = prev + 1;
    }
}

trait Bound {
    fn new(num_valves: u32) -> Self;
    fn update(&mut self, b: &BranchState);
    fn better(&self, b: &BranchState, g: &Graph, next_cost: u16) -> bool;
}

struct BestBound {
    best: u16,
}

impl Bound for BestBound {
    #[inline(always)]
    fn new(_: u32) -> Self {
        Self { best: 0 }
    }

    #[inline(always)]
    fn update(&mut self, b: &BranchState) {
        self.best = self.best.max(b.score);
    }

    #[inline(always)]
    fn better(&self, b: &BranchState, g: &Graph, next_cost: u16) -> bool {
        let mut bound = b.score + g.valves[b.next as usize] * (b.steps_left - next_cost);
        let mut steps_left_hypo = b.steps_left - next_cost;
        for valve in 0..g.num_valves as u16 {
            if (1 << valve) & b.visited != 0 || valve == b.next || steps_left_hypo < g.best_dist {
                continue;
            }
            steps_left_hypo -= g.best_dist;
            bound += g.valves[valve as usize] * steps_left_hypo;
        }
        bound > self.best
    }
}

struct ComplementBound {
    bests: Vec<u16>,
}

impl Bound for ComplementBound {
    #[inline(always)]
    fn new(num_valves: u32) -> Self {
        Self { bests: vec![0; usize::pow(2, num_valves - 1)] }
    }

    #[inline(always)]
    fn update(&mut self, b: &BranchState) {
        let visited = b.visited as usize;
        self.bests[visited] = self.bests[visited].max(b.score);
    }

    #[inline(always)]
    fn better(&self, b: &BranchState, g: &Graph, next_cost: u16) -> bool {
        true
    }
}

fn best_pair(scores: &Vec<u16>) -> u16 {
    // Collect and sort indices of scores.
    let mut best_paths: Vec<usize> = (0..scores.len()).filter(|&idx| scores[idx] > 0).collect();
    best_paths.sort_unstable_by_key(|&idx| Reverse(scores[idx]));

    // Find the pair with maximum sum that doesn't bitwise overlap.
    let mut best = 0;
    let mut iter = best_paths.iter();
    while let Some(&a) = iter.next() {
        if scores[a] * 2 <= best {
            break;
        }
        for &b in iter.clone().skip(1) {
            let score = scores[a] + scores[b];
            if score <= best {
                break;
            } else if a & b == 0 {
                best = score;
            }
        }
    }
    best
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
