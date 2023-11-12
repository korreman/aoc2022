use std::{fmt::Write, mem::swap, ops::BitAndAssign};

use itertools::izip;

pub fn run(input: &str) -> (usize, usize) {
    run_helper(input)
}

pub fn run_helper(input: &str) -> (usize, usize) {
    let mut state: State = State::parse(input);
    let mut minutes = 0;

    // Part 1
    while !state.step() {
        minutes += 1;
    }
    state.step(); // extra step into the goal
    minutes += 2; // one minute not counted, one extra
    let res1 = minutes;

    // Part 2
    // Going back
    state.clear_positions();
    while !state.step_back() {
        minutes += 1;
    }
    state.step_back(); // extra step into the start
    minutes += 2;

    // Going forward again
    state.clear_positions();
    while !state.step() {
        minutes += 1;
    }
    minutes += 2;

    (res1, minutes)
}

struct State {
    width: usize,
    positions: Board,
    blizzard_u: Board,
    blizzard_d: Board,
    blizzard_l: Board,
    blizzard_r: Board,
}

impl State {
    fn parse(input: &str) -> Self {
        let mut positions = Board { rows: Vec::new() };
        let mut blizzard_u = Board { rows: Vec::new() };
        let mut blizzard_d = Board { rows: Vec::new() };
        let mut blizzard_l = Board { rows: Vec::new() };
        let mut blizzard_r = Board { rows: Vec::new() };
        let mut width = 0;
        for line in input.lines() {
            width = line.as_bytes().len() - 2;
            assert!(width <= 128 || line.is_empty());
            if line.starts_with("##") || line.starts_with("#.#") {
                continue;
            }
            let mut u = 0;
            let mut d = 0;
            let mut l = 0;
            let mut r = 0;
            for c in line.chars() {
                if c == '#' {
                    continue;
                }
                u <<= 1;
                d <<= 1;
                l <<= 1;
                r <<= 1;
                match c {
                    '^' => u |= 1,
                    'v' => d |= 1,
                    '<' => l |= 1,
                    '>' => r |= 1,
                    '.' => (),
                    _ => panic!(),
                }
            }
            positions.rows.push(0);
            blizzard_u.rows.push(!u);
            blizzard_d.rows.push(!d);
            blizzard_l.rows.push(!l);
            blizzard_r.rows.push(!r);
        }
        Self { width, positions, blizzard_u, blizzard_d, blizzard_l, blizzard_r }
    }

    fn clear_positions(&mut self) {
        for row in &mut self.positions.rows {
            *row = 0;
        }
    }

    fn run_blizzards(&mut self) {
        // Move blizzards.
        self.blizzard_u.rotu();
        self.blizzard_d.rotd();
        self.blizzard_l.rotl(self.width);
        self.blizzard_r.rotr(self.width);
        // Remove possibilities from board.
        self.positions &= &self.blizzard_u;
        self.positions &= &self.blizzard_d;
        self.positions &= &self.blizzard_l;
        self.positions &= &self.blizzard_r;
    }

    fn step(&mut self) -> bool {
        // Propagate movement options.
        self.positions.propagate(self.width);
        // Add entry move as well.
        // Corresponds to the upper left corner.
        *self.positions.rows.first_mut().unwrap() |= 1 << (self.width - 1);
        // Simulate blizzards
        self.run_blizzards();
        // Advance the board state
        // Indicate whether the target cell has been reached.
        self.positions.rows.last().unwrap().trailing_zeros() == 0
    }

    fn step_back(&mut self) -> bool {
        // Same as step, but start and goal are inverted.
        self.positions.propagate(self.width);
        *self.positions.rows.last_mut().unwrap() |= 1;
        self.run_blizzards();
        self.positions.rows.first().unwrap().leading_zeros() == 128 - self.width as u32
    }
}

struct Board {
    rows: Vec<u128>,
}

impl Board {
    fn propagate(&mut self, width: usize) {
        let mut new_rows = self.rows.clone();
        for row in &mut new_rows {
            *row |= *row << 1;
            *row |= *row >> 1;
            *row = mask(*row, width);
        }
        for i in 0..self.rows.len() - 1 {
            new_rows[i] |= self.rows[i + 1];
            new_rows[i + 1] |= self.rows[i];
        }
        swap(&mut self.rows, &mut new_rows);
    }

    fn rotl(&mut self, width: usize) {
        for row in &mut self.rows {
            *row = rotl_assign(*row, width);
        }
    }

    fn rotr(&mut self, width: usize) {
        for row in &mut self.rows {
            *row = rotr_assign(*row, width);
        }
    }

    fn rotd(&mut self) {
        let row = self.rows.pop().unwrap();
        self.rows.insert(0, row);
    }

    fn rotu(&mut self) {
        let row = self.rows.remove(0);
        self.rows.push(row);
    }
}

impl BitAndAssign<&Board> for Board {
    fn bitand_assign(&mut self, rhs: &Self) {
        for (self_row, rhs_row) in self.rows.iter_mut().zip(rhs.rows.iter()) {
            *self_row &= *rhs_row;
        }
    }
}

fn mask(x: u128, width: usize) -> u128 {
    x & ((1 << width) - 1)
}

fn rotl_assign(mut x: u128, width: usize) -> u128 {
    x = !x;
    x = (x << 1) | (x >> (width - 1));
    x = !x;
    x
}

fn rotr_assign(mut x: u128, width: usize) -> u128 {
    x = !x;
    x = (x >> 1) | (x << (width - 1));
    x = !x;
    x
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(" {:▁<1$}\n", "", self.width))?;
        for (row_p, row_u, row_d, row_l, row_r) in izip!(
            &self.positions.rows,
            &self.blizzard_u.rows,
            &self.blizzard_d.rows,
            &self.blizzard_l.rows,
            &self.blizzard_r.rows,
        ) {
            let mut p = *row_p;
            let mut u = *row_u;
            let mut d = *row_d;
            let mut l = *row_l;
            let mut r = *row_r;
            let m = 1 << (self.width - 1);
            f.write_char('▕')?;
            for _ in 0..self.width {
                let mut c = ' ';
                if (p & m) != 0 {
                    c = 'O';
                } else if (u & m) == 0 {
                    c = '↑';
                } else if (d & m) == 0 {
                    c = '↓';
                } else if (l & m) == 0 {
                    c = '←';
                } else if (r & m) == 0 {
                    c = '→';
                }
                f.write_char(c)?;
                p <<= 1;
                u <<= 1;
                d <<= 1;
                l <<= 1;
                r <<= 1;
            }
            f.write_str("▏\n")?;
        }
        f.write_fmt(format_args!(" {:▔<1$}", "", self.width))?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_blizzards() {
        let input = "\
#.#######
#.......#
#.......#
#.......#
#.......#
#######.#";
        assert_eq!(run_helper(input).0, 103);
    }

    #[test]
    fn some_blizzards() {
        let input = "\
######################################################################################################
#.....>..>.>>>...........v...........................................................................#
#......................................<<<..<.<..............^.......................................#
#........................v...................................^.......................................#
#....................................................................................................#
######################################################################################################\
";
        assert_eq!(run(input).0, 103);
    }

    #[test]
    fn diagonal() {
        let input = "\
#.####################################################################################################
#vvv.vvvv.vvvv.v.....................................................................................#
#vv.vvvv.vvvv.vv................<<<..................................................................#
#v.vvvv.vvvv.vvv.....................>>>........^....................................................#
#.vvvv.vvvv.vvvv................................^....................................................#
#vvvv.vvvv.vvvv......................................................................................#
####################################################################################################.#\
";
        assert_eq!(run(input).0, 103);
    }

    #[test]
    fn wraparound() {
        let input = "\
#.###################################################################
#...................................................................#
#<<<<<<<<<<<<<<<<<<<<<<<<<..........................................#
#....................................................>>>>..>>>.>>.>.#
#...................................................................#
#...................................................................#
###################################################################.#\
";
        assert_eq!(run_helper(input).0, 103);
    }

    // TODO: Which end of the bitstring are the relevant bits actually located in?
    // Make sure all functions agree on this.
    #[test]
    fn example() {
        let input = "\
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#\
";
        assert_eq!(run_helper(input), (18, 54));
    }

    // TODO: Which end of the bitstring are the relevant bits actually located in?
    // Make sure all functions agree on this.
    #[test]
    fn wall() {
        let input = "\
#.######
#<.....#
#<.....#
#<.....#
#<.....#
######.#\
";
        assert_eq!(run_helper(input), (18, 54));
    }
}
