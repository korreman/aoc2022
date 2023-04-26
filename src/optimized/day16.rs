use fxhash::FxHashMap;
use itertools::Itertools;

use std::cmp::Reverse;
use std::mem::swap;

pub fn run(input: &str) -> (u16, u16) {
    let valves = parse(input);
    let graph = preprocess(valves);
    let res1 = graph.branch_and_bound(30);
    let scores = graph.dfs(26);
    let res2 = best_pair(&scores);
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
        start: (valves_flow.len() - 1) as u16,
        valves: valves_flow.iter().map(|idx| valves[*idx].0).collect(),
        dist_matrix,
        best_dist,
    }
}

struct Graph {
    start: u16,
    valves: Vec<u16>,
    dist_matrix: Vec<u16>,
    best_dist: u16,
}

impl Graph {
    fn dfs(&self, mut steps_left: u16) -> Vec<u16> {
        let num_valves = self.valves.len();
        let mut scores: Vec<u16> = vec![0; usize::pow(2, num_valves as u32 - 1)];
        let mut score = 0;

        let mut visited: u16 = 0;
        let mut stack: Vec<u16> = vec![self.start];
        let mut next: u16 = 0;
        loop {
            if next < num_valves as u16 - 1 {
                let current = *stack.last().unwrap();
                let next_cost = self.dist_matrix[next as usize + current as usize * num_valves];
                if next_cost <= steps_left && (1 << next) & visited == 0 {
                    visited |= 1 << next;
                    steps_left -= next_cost;
                    score += self.valves[next as usize] * steps_left;
                    stack.push(next);
                    next = 0;
                    scores[visited as usize] = scores[visited as usize].max(score);
                } else {
                    next += 1;
                }
            } else if stack.len() > 1 {
                let prev = stack.pop().unwrap();
                score -= self.valves[prev as usize] * steps_left;
                let prev_current = *stack.last().unwrap();
                steps_left += self.dist_matrix[prev as usize + prev_current as usize * num_valves];
                visited &= !(1 << prev);
                next = prev + 1;
            } else {
                break;
            }
        }
        scores
    }

    fn branch_and_bound(&self, mut steps_left: u16) -> u16 {
        let num_valves = self.valves.len();
        let mut best_score: u16 = 0;
        let mut score = 0;

        let mut visited: u16 = 1 << self.start;
        let mut stack: Vec<u16> = vec![self.start];
        let mut next: u16 = 0;
        loop {
            if next < num_valves as u16 {
                let current = *stack.last().unwrap();
                let next_cost = self.dist_matrix[next as usize + current as usize * num_valves];
                if next_cost <= steps_left && (1 << next) & visited == 0 {
                    // bound guard
                    let mut bound = score + self.valves[next as usize] * (steps_left - next_cost);
                    let mut steps_left_hypo = steps_left - next_cost;
                    for valve in 0..num_valves as u16 {
                        if (1 << valve) & visited != 0
                            || valve == next
                            || steps_left_hypo < self.best_dist
                        {
                            continue;
                        }
                        steps_left_hypo -= self.best_dist;
                        bound += self.valves[valve as usize] * steps_left_hypo;
                    }
                    if bound <= best_score {
                        next += 1;
                        continue;
                    }
                    // advance
                    visited |= 1 << next;
                    steps_left -= next_cost;
                    score += self.valves[next as usize] * steps_left;
                    best_score = best_score.max(score);
                    stack.push(next);
                    next = 0;
                } else {
                    next += 1;
                }
            } else if stack.len() > 1 {
                let prev = stack.pop().unwrap();
                score -= self.valves[prev as usize] * steps_left;
                let prev_current = *stack.last().unwrap();
                steps_left += self.dist_matrix[prev as usize + prev_current as usize * num_valves];
                visited ^= 1 << prev;
                next = prev + 1;
            } else {
                break;
            }
        }
        best_score
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
