use std::mem::swap;

use fxhash::FxHashMap;
use itertools::Itertools;

pub fn run(input: &str) -> (u16, u16) {
    let mut idxs = FxHashMap::default();
    let valves = input.lines().map(parse_valve).collect_vec();
    let mut valves_flow = Vec::new();
    for (idx, (flow, name, _)) in valves.iter().enumerate() {
        idxs.insert(*name, idx);
        if *flow > 0 {
            valves_flow.push(idx);
        }
    }
    valves_flow.sort_by_key(|idx| valves[*idx].0);
    valves_flow.reverse();
    let valves = valves
        .iter()
        .map(|(flow, _, edges)| {
            let edges = edges.iter().map(|edge| idxs[edge]).collect_vec();
            (*flow, edges)
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

    // 1. Find all-pairs shortest path between valves with non-zero flow.
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

    // 2. Find best path possible within 30 minutes.
    let graph = Graph {
        start: start_flow as u16,
        num_valves: valves_flow.len() as u16,
        pressures: valves_flow
            .iter()
            .map(|idx| valves[*idx].0 as u16)
            .collect(),
        dist_matrix: weights_flow,
    };

    let res1 = graph.branch_and_bound(30);

    // 3. Find best paths that are possible within 26 minutes.
    let scores = graph.dfs(26);

    // 4. Find the pair of non-intersecting paths with the highest combined score.
    let sorted: Vec<u32> = (0..scores.len() as u32)
        .filter(|&idx| scores[idx as usize] > 0)
        .sorted_unstable_by_key(|&idx| scores[idx as usize])
        .collect();
    let mut best = 0;
    let mut outers = sorted.iter().rev();
    while let Some(a) = outers.next() {
        let score_a = scores[*a as usize];
        if score_a * 2 <= best {
            // At this point there is no possibility of a better total score.
            break;
        }
        for b in outers.clone().skip(1) {
            let score_b = scores[*b as usize];
            let score = score_a + score_b;
            if score <= best {
                break;
            } else if (a & b) & !(1 << start_flow) == 0 {
                best = score;
            }
        }
    }
    let res2 = best;

    (res1, res2)
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
                // TODO: && steps_left > 0
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
            // TODO: && steps_left > 0
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

fn parse_valve(line: &str) -> (u64, &str, Vec<&str>) {
    let words = line.split([' ', '=', ',', ';']).filter(|w| !w.is_empty());
    let mut words = words.skip(1);
    let label = words.next().unwrap();
    let mut words = words.skip(3);
    let flow = words.next().unwrap().parse().ok().unwrap();
    let tunnels = words.skip(4).collect_vec();
    (flow, label, tunnels)
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
