use std::{
    mem::swap,
    ops::{Add, Sub},
};

use fxhash::FxHashSet;
use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Default)]
struct Currency {
    ore: u8,
    clay: u8,
    obsidian: u8,
    geode: u8,
}

impl Currency {
    fn ore(count: u8) -> Self {
        Self {
            ore: count,
            ..Default::default()
        }
    }
    fn clay(count: u8) -> Self {
        Self {
            clay: count,
            ..Default::default()
        }
    }
    fn obsidian(count: u8) -> Self {
        Self {
            obsidian: count,
            ..Default::default()
        }
    }
    fn geode(count: u8) -> Self {
        Self {
            geode: count,
            ..Default::default()
        }
    }

    // ignores geodes
    fn ge(&self, other: &Self) -> bool {
        self.ore >= other.ore && self.clay >= other.clay && self.obsidian >= other.obsidian
    }

    fn max_merge(self, other: Self) -> Self {
        Self {
            ore: self.ore.max(other.ore),
            clay: self.clay.max(other.clay),
            obsidian: self.obsidian.max(other.obsidian),
            geode: self.geode.max(other.geode),
        }
    }
}

impl std::fmt::Debug for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "({:3}r, {:3}c, {:3}o, {:3}g)",
            self.ore, self.clay, self.obsidian, self.geode
        ))
    }
}

impl Add for Currency {
    type Output = Currency;

    fn add(self, rhs: Self) -> Self::Output {
        Currency {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl Sub for Currency {
    type Output = Currency;

    fn sub(self, rhs: Self) -> Self::Output {
        Currency {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Transaction {
    cost: Currency,
    gain: Currency,
}

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    id: u8,
    ore: Currency,
    clay: Currency,
    obsidian: Currency,
    geode: Currency,
}

impl Blueprint {
    fn parse(line: &str) -> Option<Self> {
        let (id, ore, clay, obsidian_ore, obsidian_clay, geode_ore, geode_obsidian) = line
            .split([' ', ':'])
            .filter_map(|x| x.parse().ok())
            .collect_tuple()?;
        Some(Blueprint {
            id,
            ore: Currency::ore(ore),
            clay: Currency::ore(clay),
            obsidian: Currency::ore(obsidian_ore) + Currency::clay(obsidian_clay),
            geode: Currency::ore(geode_ore) + Currency::obsidian(geode_obsidian),
        })
    }

    fn transactions(&self) -> [Transaction; 4] {
        [
            Transaction {
                cost: self.ore,
                gain: Currency::ore(1),
            },
            Transaction {
                cost: self.clay,
                gain: Currency::clay(1),
            },
            Transaction {
                cost: self.obsidian,
                gain: Currency::obsidian(1),
            },
            Transaction {
                cost: self.geode,
                gain: Currency::geode(1),
            },
        ]
    }

    fn simulate(&mut self, minutes: usize) -> FxHashSet<State> {
        let mut states = FxHashSet::default();
        let mut off_states = FxHashSet::default();
        states.insert(State {
            inventory: Default::default(),
            robots: Currency::ore(1),
        });
        let max_costs: Currency = self
            .transactions()
            .iter()
            .map(|t| t.cost)
            .reduce(Currency::max_merge)
            .unwrap();
        for _ in 1..=minutes {
            for state in &states {
                if state.inventory.ge(&self.geode) {
                    off_states.insert(State {
                        inventory: state.inventory - self.geode + state.robots,
                        robots: state.robots + Currency::geode(1),
                    });
                } else {
                    off_states.insert(State {
                        inventory: state.inventory + state.robots,
                        robots: state.robots,
                    });
                    for transaction in self.transactions() {
                        if state.inventory.ge(&transaction.cost) {
                            off_states.insert(State {
                                inventory: state.inventory - transaction.cost + state.robots,
                                robots: state.robots + transaction.gain,
                            });
                        }
                    }
                }
            }
            swap(&mut states, &mut off_states);
            off_states.clear();
            states.retain(|State { robots, .. }| {
                max_costs.ge(robots)
            });
        }
        states
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct State {
    inventory: Currency,
    robots: Currency,
}

pub fn run(input: &str) -> (u64, u64) {
    let mut blueprints = input
        .lines()
        .map(|line| Blueprint::parse(line).unwrap())
        .collect_vec();

    let mut res1 = 0;
    for blueprint in &mut blueprints {
        let states = blueprint.simulate(24);
        let max_geodes = states
            .iter()
            .map(|state| state.inventory.geode)
            .max()
            .unwrap();
        res1 += max_geodes as u64 * blueprint.id as u64;
    }

    let mut res2 = 1;
    let upper = blueprints.len().min(3);
    for blueprint in &mut blueprints[0..upper] {
        let states = blueprint.simulate(32);
        let max_geodes = states
            .iter()
            .map(|state| state.inventory.geode)
            .max()
            .unwrap();
        res2 *= max_geodes as u64;
    }

    (res1, res2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.\nBlueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
        assert_eq!(super::run(input), (33, 56 * 62));
    }

    #[test]
    fn test_tiny() {
        let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";
        assert_eq!(super::run(input), (9, 56));
    }
}
