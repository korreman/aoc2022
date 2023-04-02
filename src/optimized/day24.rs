use std::{
    fmt::Write,
    ops::{BitAndAssign, BitOrAssign, Shl, Shr},
};

use itertools::{izip};

#[derive(Clone, Copy, PartialEq, Eq)]
struct Row100(u128);

impl BitAndAssign for Row100 {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOrAssign for Row100 {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl Shl<usize> for Row100 {
    type Output = Row100;

    fn shl(self, rhs: usize) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shr<usize> for Row100 {
    type Output = Row100;

    fn shr(self, rhs: usize) -> Self::Output {
        Self(self.mask().0 >> rhs)
    }
}

impl Row100 {
    fn mask(self) -> Self {
        Self(self.0 & 0xF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF)
    }

    fn rotl(self, cnt: usize) -> Self {
        Self(self.0 << cnt | self.0 >> (100 - cnt))
    }

    fn rotl_assign(&mut self) {
        self.0 = self.0 << 1 | self.0 >> 99
    }

    fn rotr(self, cnt: usize) -> Self {
        Self(self.mask().0 >> cnt | self.0 << (100 - cnt))
    }

    fn rotr_assign(&mut self) {
        self.0 = self.mask().0 >> 1 | self.0 << 99;
    }
}

struct Board {
    rows: Vec<Row100>,
}

impl Board {
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

impl BitAndAssign<&Board> for Board {
    fn bitand_assign(&mut self, rhs: &Self) {
        for (self_row, rhs_row) in self.rows.iter_mut().zip(rhs.rows.iter()) {
            *self_row &= *rhs_row;
        }
    }
}

struct State {
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
        for line in input.lines() {
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
            positions.rows.push(Row100(0));
            blizzard_u.rows.push(Row100(!u));
            blizzard_d.rows.push(Row100(!d));
            blizzard_l.rows.push(Row100(!l));
            blizzard_r.rows.push(Row100(!r));
        }
        Self {
            positions,
            blizzard_u,
            blizzard_d,
            blizzard_l,
            blizzard_r,
        }
    }

    fn step(&mut self) -> bool {
        // Propagate movement options.
        self.positions.propagate();
        // Add entry move as well.
        // Corresponds to the upper left corner.
        self.positions.rows.first_mut().unwrap().0 |= 0x8_0000_0000_0000_0000_0000_0000;
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
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:█<102}\n", ""))?;
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
            let m = 1u128 << 99;
            f.write_char('█')?;
            for _ in 0..100 {
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
            f.write_str("█\n")?;
        }
        f.write_fmt(format_args!("{:█<102}", ""))?;
        Ok(())
    }
}

pub fn run(input: &str) -> (usize, usize) {
    // 1. Generate bit arrays from the grid.
    let mut state: State = State::parse(input);
    //println!("{state}");
    // 2. Generate an empty playing field.
    let mut minutes = 1;
    while !state.step() {
        minutes += 1;
        //let mut s = String::new();
        //drop(std::io::stdin().read_line(&mut s));
        //println!("{state}");
    }
    (minutes, 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_blizzards() {
        let input = "\
######################################################################################################
#....................................................................................................#
#....................................................................................................#
#....................................................................................................#
#....................................................................................................#
######################################################################################################\
";
        assert_eq!(run(input).0, 103);
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
#.####################################################################################################
#.........................................................v..........................................#
#.<.<<.<<<..<<<<..........................................v..........................................#
#.........................................................v...........................>>>>..>>>.>>.>.#
#.........................................................v..........................................#
#.........................................................v..........................................#
####################################################################################################.#\
";
        assert_eq!(run(input).0, 103);
    }

}
