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
    let valves = valves
        .iter()
        .map(|(flow, _, edges)| {
            let edges = edges.iter().map(|edge| idxs[edge]).collect_vec();
            (flow, edges)
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

    // 2. Find best paths that are possible within 26 minutes.
    let start = start_flow;
    let weights = weights_flow;
    let mut scores: Vec<u16> = vec![0; usize::pow(2, valves_flow.len() as u32)];

    // 3. Find the pair of non-intersecting paths with the highest combined score.
    let sorted: Vec<u16> = (0..scores.len() as u16)
        .sorted_unstable_by_key(|&idx| scores[idx as usize])
        .collect();
    let mut best = 0;
    for a in (0..sorted.len()).rev() {
        if scores[a] < best / 2 {
            // At this point there is no possibility of a better pair.
            break;
        }
        for b in (0..a).rev() {
            if a ^ b != 0 {
                let score = scores[a] + scores[b];
                if score > best {
                    best = score;
                } else {
                    // We just find the best B path,
                    // the rest will necessarily be worse.
                    break;
                }
            }
        }
    }

    // 4. Find the best path within 30 minutes, potentially reusing step 2 results.

    (0, 0)
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

// Observations:
// - Graph is undirected.
// - There are exactly 15/61 valves with non-zero flow.
//   Sets of all valves can fit in a u64.
//   Sets of flow valves can fit in a u16.
//   All flow sets can be indexed in a size 2^15 array.
// - Valve names are all two uppercase ascii letters.
//   Or 2 bytes.
// - The initial graph is pretty sparse.

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
