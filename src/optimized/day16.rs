use std::cmp::Reverse;
use std::mem::swap;

use fxhash::FxHashMap;
use itertools::Itertools;

pub fn run(input: &str) -> (u16, u16) {
    let valves = parse(input);
    let graph = preprocess(valves);
    let res1 = graph.branch_and_bound(30);
    let scores = graph.dfs(26);
    let res2 = best_pair(&graph, &scores);
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
    // Convert to a vector graph representation.
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
    let valves = valves
        .iter()
        .map(|valve| {
            let tunnels = valve.tunnels.iter().map(|edge| idxs[edge]).collect_vec();
            (valve.flow, tunnels)
        })
        .collect_vec();

    let start = idxs["AA"];
    let start_flow = match valves_flow.iter().position(|x| x == &start) {
        Some(idx) => idx,
        None => {
            valves_flow.push(start);
            valves_flow.len() - 1
        }
    };

    let mut weights = vec![None; valves_flow.len() * valves.len()];
    for (col, &src) in valves_flow.iter().enumerate() {
        let mut handles = vec![src];
        let mut handles_other = Vec::new();
        let mut step = 0u16;
        while !handles.is_empty() {
            step += 1;
            for tgt in &handles {
                let weight = &mut weights[tgt + col * valves.len()];
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
    let weights_flow = valves_flow
        .iter()
        .cartesian_product(0..valves_flow.len())
        .map(|(row, col)| weights[row + col * valves.len()].unwrap().get())
        .collect_vec();

    Graph {
        start: start_flow as u16,
        num_valves: valves_flow.len() as u16,
        pressures: valves_flow
            .iter()
            .map(|idx| valves[*idx].0 as u16)
            .collect(),
        dist_matrix: weights_flow,
    }
}

struct Graph {
    start: u16,
    num_valves: u16,
    pressures: Vec<u16>,
    dist_matrix: Vec<u16>,
}

impl Graph {
    fn dfs(&self, mut steps_left: u16) -> Vec<u16> {
        let mut scores: Vec<u16> = vec![0; usize::pow(2, self.num_valves as u32)];
        let mut score = 0;

        let mut visited: u16 = 1 << self.start;
        let mut stack: Vec<u16> = vec![self.start];
        let mut next: u16 = 0;
        loop {
            if next < self.num_valves {
                let current = *stack.last().unwrap();
                let next_cost = self.dist_matrix[(next + current * self.num_valves) as usize];
                if next_cost <= steps_left && (1 << next) & visited == 0 {
                    visited |= 1 << next;
                    steps_left -= next_cost;
                    score += self.pressures[next as usize] * steps_left;
                    stack.push(next);
                    next = 0;
                    scores[visited as usize] = scores[visited as usize].max(score);
                } else {
                    next += 1;
                }
            } else if stack.len() > 1 {
                let prev = stack.pop().unwrap();
                score -= self.pressures[prev as usize] * steps_left;
                let prev_current = *stack.last().unwrap();
                steps_left += self.dist_matrix[(prev + prev_current * self.num_valves) as usize];
                visited &= !(1 << prev);
                next = prev + 1;
            } else {
                break;
            }
        }
        scores
    }

    fn branch_and_bound(&self, mut steps_left: u16) -> u16 {
        let mut best_score: u16 = 0;
        let mut score = 0;

        let mut visited: u16 = 1 << self.start;
        let mut stack: Vec<u16> = vec![self.start];
        let mut next: u16 = 0;
        loop {
            if next < self.num_valves {
                let current = *stack.last().unwrap();
                let next_cost = self.dist_matrix[(next + current * self.num_valves) as usize];
                if next_cost <= steps_left && (1 << next) & visited == 0 {
                    // bound guard
                    let remaining_valves = (0..self.num_valves).filter_map(|valve| {
                        if (1 << valve) & visited == 0 && valve != next {
                            Some(self.pressures[valve as usize])
                        } else {
                            None
                        }
                    });
                    let bound = score
                        + self.pressures[next as usize] * steps_left
                        + remaining_valves
                            .zip((0..steps_left - next_cost).rev())
                            .map(|(a, b)| a * b)
                            .sum::<u16>();
                    if bound <= best_score {
                        next += 1;
                        continue;
                    }
                    // advance
                    visited |= 1 << next;
                    steps_left -= next_cost;
                    score += self.pressures[next as usize] * steps_left;
                    best_score = best_score.max(score);
                    stack.push(next);
                    next = 0;
                } else {
                    next += 1;
                }
            } else if stack.len() > 1 {
                let prev = stack.pop().unwrap();
                score -= self.pressures[prev as usize] * steps_left;
                let prev_current = *stack.last().unwrap();
                steps_left += self.dist_matrix[(prev + prev_current * self.num_valves) as usize];
                visited ^= 1 << prev;
                next = prev + 1;
            } else {
                break;
            }
        }
        best_score
    }
}

fn best_pair(graph: &Graph, scores: &Vec<u16>) -> u16 {
    let mut sorted: Vec<u32> = (0..scores.len() as u32)
        .filter(|&idx| scores[idx as usize] > 0)
        .collect();
    sorted.sort_unstable_by_key(|&idx| scores[idx as usize]);
    let mut best = 0;
    let mut outers = sorted.iter().rev();
    while let Some(a) = outers.next() {
        let score_a = scores[*a as usize];
        if score_a * 2 <= best {
            break;
        }
        for b in outers.clone().skip(1) {
            let score_b = scores[*b as usize];
            let score = score_a + score_b;
            if score <= best {
                break;
            } else if (a & b) & !(1 << graph.start) == 0 {
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
