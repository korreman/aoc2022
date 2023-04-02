use std::{
    fmt::Write,
    ops::{BitAndAssign, BitOrAssign, Shl, Shr},
};

use itertools::izip;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Row<const N: usize>(u128);

impl<const N: usize> BitAndAssign for Row<N> {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl<const N: usize> BitOrAssign for Row<N> {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl<const N: usize> Shl<usize> for Row<N> {
    type Output = Row<N>;

    fn shl(self, rhs: usize) -> Self::Output {
        Self((self.0 << rhs) & ((1 << N) - 1))
    }
}

impl<const N: usize> Shr<usize> for Row<N> {
    type Output = Row<N>;

    fn shr(self, rhs: usize) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl<const N: usize> Row<N> {
    fn mask(self) -> Self {
        Self(self.0 | !((1 << N) - 1))
    }

    fn rotl_assign(&mut self) {
        self.0 = !self.0;
        self.0 = (self.0 << 1) | (self.0 >> (N - 1));
        *self = Self(!self.0).mask();
    }

    fn rotr_assign(&mut self) {
        self.0 = !self.0;
        self.0 = (self.0 >> 1) | (self.0 << (N - 1));
        self.0 = !self.0;
    }
}

struct Board<const C: usize> {
    rows: Vec<Row<C>>,
}

impl<const R: usize> Board<R> {
    fn propagate(&mut self) {
        let mut new_rows = self.rows.clone();
        for row in &mut new_rows {
            *row |= *row << 1;
            *row |= *row >> 1;
        }
        for i in 0..self.rows.len() - 1 {
            new_rows[i] |= self.rows[i + 1];
            new_rows[i + 1] |= self.rows[i];
        }
        std::mem::swap(&mut self.rows, &mut new_rows);
    }

    fn rotl(&mut self) {
        for row in &mut self.rows {
            row.rotl_assign();
        }
    }

    fn rotr(&mut self) {
        for row in &mut self.rows {
            row.rotr_assign();
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

impl<const C: usize> BitAndAssign<&Board<C>> for Board<C> {
    fn bitand_assign(&mut self, rhs: &Self) {
        for (self_row, rhs_row) in self.rows.iter_mut().zip(rhs.rows.iter()) {
            *self_row &= *rhs_row;
        }
    }
}

struct State<const C: usize> {
    positions: Board<C>,
    blizzard_u: Board<C>,
    blizzard_d: Board<C>,
    blizzard_l: Board<C>,
    blizzard_r: Board<C>,
}

impl<const C: usize> State<C> {
    fn parse(input: &str) -> Self {
        let mut positions = Board { rows: Vec::new() };
        let mut blizzard_u = Board { rows: Vec::new() };
        let mut blizzard_d = Board { rows: Vec::new() };
        let mut blizzard_l = Board { rows: Vec::new() };
        let mut blizzard_r = Board { rows: Vec::new() };
        for line in input.lines() {
            assert!(line.as_bytes().len() == C + 2 || line.is_empty());
            if line.starts_with("##") || line.starts_with("#.#") {
                continue;
            }
            let mut u = 0u128;
            let mut d = 0u128;
            let mut l = 0u128;
            let mut r = 0u128;
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
            positions.rows.push(Row::<C>(0));
            blizzard_u.rows.push(Row::<C>(!u));
            blizzard_d.rows.push(Row::<C>(!d));
            blizzard_l.rows.push(Row::<C>(!l));
            blizzard_r.rows.push(Row::<C>(!r));
        }
        Self {
            positions,
            blizzard_u,
            blizzard_d,
            blizzard_l,
            blizzard_r,
        }
    }

    fn clear_positions(&mut self) {
        for row in &mut self.positions.rows {
            row.0 = 0;
        }
    }

    fn step(&mut self) -> bool {
        // Propagate movement options.
        self.positions.propagate();
        // Add entry move as well.
        // Corresponds to the upper left corner.
        self.positions.rows.first_mut().unwrap().0 |= 1 << (C - 1);
        // Move blizzards.
        self.blizzard_u.rotu();
        self.blizzard_d.rotd();
        self.blizzard_l.rotl();
        self.blizzard_r.rotr();
        // Remove possibilities from board.
        self.positions &= &self.blizzard_u;
        self.positions &= &self.blizzard_d;
        self.positions &= &self.blizzard_l;
        self.positions &= &self.blizzard_r;
        // Indicate whether the target cell has been reached.
        self.positions.rows.last().unwrap().0.trailing_zeros() == 0
    }

    fn step_back(&mut self) -> bool {
        // Propagate movement options.
        self.positions.propagate();
        // Add entry move as well.
        // Corresponds to the upper left corner.
        self.positions.rows.last_mut().unwrap().0 |= 1;
        // Move blizzards.
        self.blizzard_u.rotu();
        self.blizzard_d.rotd();
        self.blizzard_l.rotl();
        self.blizzard_r.rotr();
        // Remove possibilities from board.
        self.positions &= &self.blizzard_u;
        self.positions &= &self.blizzard_d;
        self.positions &= &self.blizzard_l;
        self.positions &= &self.blizzard_r;
        // Indicate whether the target cell has been reached.
        self.positions.rows.first().unwrap().0.leading_zeros() == 128 - C as u32
    }
}

impl<const C: usize> std::fmt::Display for State<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(" {:▁<1$}\n", "", C))?;
        for (row_p, row_u, row_d, row_l, row_r) in izip!(
            &self.positions.rows,
            &self.blizzard_u.rows,
            &self.blizzard_d.rows,
            &self.blizzard_l.rows,
            &self.blizzard_r.rows,
        ) {
            let mut p = row_p.0;
            let mut u = row_u.0;
            let mut d = row_d.0;
            let mut l = row_l.0;
            let mut r = row_r.0;
            let m = 1 << (C - 1);
            f.write_char('▕')?;
            for _ in 0..C {
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
        f.write_fmt(format_args!(" {:▔<1$}", "", C))?;
        Ok(())
    }
}

pub fn run(input: &str) -> (usize, usize) {
    run_helper::<100>(input)
}

pub fn run_helper<const C: usize>(input: &str) -> (usize, usize) {
    let mut state: State<C> = State::parse(input);
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
        assert_eq!(run_helper::<7>(input).0, 103);
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
        assert_eq!(run_helper::<67>(input).0, 103);
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
        assert_eq!(run_helper::<6>(input), (18, 54));
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
        assert_eq!(run_helper::<6>(input), (18, 54));
    }

    #[test]
    fn row() {
        assert_eq!(Row::<4>(0b11011100).mask(), Row::<4>(0b00001100));
        assert_eq!(Row::<5>(0b11011100).mask(), Row::<5>(0b00011100));
        let mut x = Row::<8>(0b11011100);
        x.rotl_assign();
        assert_eq!(x.mask(), Row::<8>(0b10111001));
    }
}
