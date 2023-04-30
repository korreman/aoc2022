use std::fmt::Display;

use itertools::Itertools;

pub fn run(input: &str) -> (u32, u32) {
    // Parse
    let blueprints = input.lines().map(Blueprint::parse).collect_vec();

    // Part 1
    let res1 = {
        let mut quality_sum = 0u32;
        for (id, blueprint) in blueprints.iter().enumerate() {
            let mut state = State::new(blueprint, 24);
            state.run();
            quality_sum += (id as u32 + 1) * state.best as u32;
        }
        quality_sum
    };

    // Part 2
    let res2 = {
        let mut best_product = 1u32;
        let upper = blueprints.len().min(3);
        for blueprint in &blueprints[0..upper] {
            let mut state = State::new(blueprint, 32);
            state.run();
            best_product *= state.best as u32;
        }
        best_product
    };
    (res1, res2)
}

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    oreo: u8,
    clay: u8,
    obsi: u8,
    obsi_clay: u8,
    geod: u8,
    geod_obsi: u8,
}

impl Blueprint {
    fn parse(line: &str) -> Self {
        let (_, oreo, clay, obsi, obsi_clay, geod, geod_obsi) = line
            .split([' ', ':'])
            .filter_map(|x| x.parse().ok())
            .collect_tuple()
            .unwrap();
        Blueprint { oreo, clay, obsi, obsi_clay, geod, geod_obsi }
    }
}

struct Res {
    oreo: u8,
    clay: u8,
    obsi: u8,
}

impl Display for Res {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "o{:2} c{:2} b{:2}",
            self.oreo, self.clay, self.obsi
        ))
    }
}

struct State<'a> {
    blueprint: &'a Blueprint,
    stack: Vec<(u8, u8)>,
    next: u8,
    steps_left: u8,
    score: u16,
    best: u16,
    robots: Res,
    res: Res,
}

impl<'a> Display for State<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:?} | {}\n{}\n{} / {}\nRob: {}\nRes: {}\n",
            self.stack, self.next, self.steps_left, self.score, self.best, self.robots, self.res
        ))
    }
}

impl<'a> State<'a> {
    fn new(blueprint: &'a Blueprint, steps: u8) -> Self {
        Self {
            blueprint,
            stack: vec![],
            next: 0,
            steps_left: steps,

            score: 0,
            best: 0,

            robots: Res {
                oreo: 1, // we start with one oreo robot
                clay: 0,
                obsi: 0,
            },
            res: Res { oreo: 0, clay: 0, obsi: 0 },
        }
    }

    #[inline(always)]
    fn step(&mut self, steps: u8) -> bool {
        if self.steps_left < steps {
            return false;
        }
        self.res.oreo += steps * self.robots.oreo;
        self.res.clay += steps * self.robots.clay;
        self.res.obsi += steps * self.robots.obsi;
        self.steps_left -= steps;
        true
    }

    #[inline(always)]
    fn unstep(&mut self, steps: u8) {
        self.res.oreo -= steps * self.robots.oreo;
        self.res.clay -= steps * self.robots.clay;
        self.res.obsi -= steps * self.robots.obsi;
        self.steps_left += steps;
    }

    #[inline(always)]
    fn upper_bound(&self) -> u16 {
        // the current projection of total geodes,
        // plus the projection if we buy a geode robot in every remaining step
        let s = self.steps_left as u16;
        self.score + (s * (s + 1)) / 2
    }

    #[inline(always)]
    fn advance(&mut self) -> Option<()> {
        let req = |cost: u8, res: u8, robots: u8| {
            (cost.saturating_sub(res) + robots - 1).checked_div(robots).map(|x| x + 1)
        };
        let prev_steps_left = self.steps_left;
        match self.next {
            // ore robot
            3 => {
                let req_steps = req(self.blueprint.oreo, self.res.oreo, self.robots.oreo)?;
                self.step(req_steps).then_some(())?;
                self.res.oreo -= self.blueprint.oreo;
                self.robots.oreo += 1;
            }
            // clay robot
            2 => {
                let req_steps = req(self.blueprint.clay, self.res.oreo, self.robots.oreo)?;
                self.step(req_steps).then_some(())?;
                self.res.oreo -= self.blueprint.clay;
                self.robots.clay += 1;
            }
            // obsidian robot
            1 => {
                let req_steps_o = req(self.blueprint.obsi, self.res.oreo, self.robots.oreo)?;
                let req_steps_c = req(self.blueprint.obsi_clay, self.res.clay, self.robots.clay)?;
                let req_steps = req_steps_o.max(req_steps_c);
                self.step(req_steps).then_some(())?;
                self.res.oreo -= self.blueprint.obsi;
                self.res.clay -= self.blueprint.obsi_clay;
                self.robots.obsi += 1;
            }
            // geode robot
            0 => {
                let req_steps_o = req(self.blueprint.geod, self.res.oreo, self.robots.oreo)?;
                let req_steps_i = req(self.blueprint.geod_obsi, self.res.obsi, self.robots.obsi)?;
                let req_steps = req_steps_o.max(req_steps_i);
                self.step(req_steps).then_some(())?;
                self.res.oreo -= self.blueprint.geod;
                self.res.obsi -= self.blueprint.geod_obsi;
                self.score += self.steps_left as u16;
            }
            _ => panic!(),
        }
        self.stack.push((prev_steps_left, self.next));
        self.next = 0;
        Some(())
    }

    #[inline(always)]
    fn backtrack(&mut self) -> Option<()> {
        let (steps_left, next) = self.stack.pop()?;
        match next {
            // ore robot
            3 => {
                self.res.oreo += self.blueprint.oreo;
                self.robots.oreo -= 1;
            }
            // clay robot
            2 => {
                self.res.oreo += self.blueprint.clay;
                self.robots.clay -= 1;
            }
            // obsidian robot
            1 => {
                self.res.oreo += self.blueprint.obsi;
                self.res.clay += self.blueprint.obsi_clay;
                self.robots.obsi -= 1;
            }
            // geode robot
            0 => {
                self.res.oreo += self.blueprint.geod;
                self.res.obsi += self.blueprint.geod_obsi;
                self.score -= self.steps_left as u16;
            }
            _ => panic!(),
        }
        self.unstep(steps_left - self.steps_left);
        self.next = next + 1;
        Some(())
    }

    fn run(&mut self) {
        //let mut s = String::new();
        //let mut i = 0;
        loop {
            //i += 1;
            //if i % 10_000 == 0 {
            //    println!("{}", self.best);
            //}
            if self.next < 4 {
                if self.advance().is_some() {
                    self.best = self.best.max(self.score);
                    if self.upper_bound() <= self.best {
                        self.backtrack();
                    }
                } else {
                    self.next += 1;
                }
            } else if self.backtrack().is_none() {
                break;
            }
        }
    }
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
