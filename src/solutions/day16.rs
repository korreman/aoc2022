use itertools::Itertools;
use std::ops::Range;

use crate::util::dfs::Dfs;
use crate::util::graph::{GraphImpl, HashGraph, VecGraph};
use crate::util::grid::{pos, Grid};
use crate::util::pathfinding::bfs;

fn parse_valve<'a>(line: &str) -> Option<(&str, std::vec::IntoIter<&str>, u64)> {
    let words = line.split([' ', '=', ',', ';']).filter(|word| *word != "");
    let mut words = words.skip(1);
    let label = words.next()?;
    let mut words = words.skip(3);
    let flow = words.next()?.parse().ok()?;
    let tunnels = words.skip(4).collect_vec();
    Some((label, tunnels.into_iter(), flow))
}

struct DfsState<'a> {
    valves: &'a Vec<u64>,
    costs: &'a Grid<u64>,

    start: usize,
    node: usize,
    steps_left: u64,
    score: u64,
}

impl<'a> DfsState<'a> {
    fn new(valves: &'a Vec<u64>, costs: &'a Grid<u64>, node: usize) -> Self {
        Self {
            valves,
            costs,
            start: node,
            node,
            steps_left: 30,
            score: 0,
        }
    }

    fn score_increase(&self) -> u64 {
        self.steps_left * self.valves[self.node]
    }

    fn cost(&self, target: usize) -> u64 {
        self.costs[pos(self.node, target)] + 1
    }
}

impl<'a> Dfs for DfsState<'a> {
    type Action = usize;
    type Actions = Range<usize>;
    type Score = u64;

    fn actions(&self) -> Self::Actions {
        0..self.costs.width()
    }

    fn score(&self) -> Self::Score {
        self.score
    }

    fn perform(&mut self, history: &Vec<usize>, action: &usize) -> bool {
        // The cost of moving to a node and then opening it.
        let cost = self.cost(*action);
        if self.steps_left >= cost && !history.contains(action) {
            self.steps_left -= cost;
            self.node = *action;
            self.score += self.score_increase();
            true
        } else {
            false
        }
    }

    fn backtrack(&mut self, history: &Vec<usize>, action: &Self::Action) {
        self.score -= self.score_increase();
        self.node = *history.last().unwrap_or(&self.start);
        self.steps_left += self.cost(*action);
    }
}

pub fn run(input: &str) -> (u64, usize) {
    // Parse into a graph
    let mut valves: HashGraph<&str, u64> = input.lines().map(|l| parse_valve(l).unwrap()).collect();

    // Convert to VecGraph.
    // We're assuming from the ambiguous text that "AA" is the start node,
    // not the first line in the input.
    valves.start = "AA";
    let valves: VecGraph<u64> = valves.into();

    // Collect the valves that can release pressure
    let mut relevant_valves = valves.handles().filter(|&h| valves[h] != 0).collect_vec();
    // Add the starting valve regardless of pressure release.
    if !relevant_valves.contains(&valves.start) {
        relevant_valves.push(valves.start);
    }
    // Compute the distances between all relevant nodes.
    let mut cost_grid: Grid<u64> =
        Grid::new_filled(relevant_valves.len(), relevant_valves.len(), 0);
    for (idx, valve) in relevant_valves.iter().enumerate() {
        bfs(
            &valves,
            *valve,
            |_, _| true,
            |tcost, tnode| {
                if let Some(tidx) = relevant_valves.iter().position(|&x| x == tnode) {
                    cost_grid[pos(idx, tidx)] = tcost as u64;
                }
                false
            },
        );
    }
    // Generate a new graph containing only the relevant valves, with new indices.
    let new_valves: Vec<u64> = relevant_valves.iter().map(|&valve| valves[valve]).collect();
    let dfs_start = relevant_valves
        .iter()
        .position(|&x| x == valves.start)
        .unwrap();

    let mut dfs_state = DfsState::new(&new_valves, &cost_grid, dfs_start);
    println!("{cost_grid:3}");
    println!("{}, {:?}", dfs_start, new_valves,);
    let res1 = dfs_state.dfs();
    (res1, 0)
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
        assert_eq!(super::run(input).0, 1651);
    }
}
