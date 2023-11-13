use std::{
    fmt::{Display, Write},
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

pub fn run(_input: &str) -> (u32, u32) {
    let mut grid = Grid([Row([0; 3]); 160]);
    for _ in 0..1116 {
        grid.step();
    }
    //println!("{grid}");
    (0, 0)
}

#[derive(Clone)]
struct Grid([Row; 160]);

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0.iter() {
            row.fmt(f)?;
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Grid {
    //fn parse(input: &str) -> Self {
    //}

    fn step(&mut self) {
        // Horizontal and vertical convolutions
        let h = self.left() | self | &self.right();
        let v = self.up() | self | &self.down();

        // Active elves
        let active = h.up() | &h.down() | &v.left() | &v.right() | self;

        // Compute proposals
        let nh = !h.clone();
        let nv = !v.clone();

        let u = active.up() & &nh;
        let r = (!u.down() & &active).right() & &nv;
        let d = (!r.left() & &active).down() & &nh;
        let l = (!d.up() & &active).left() & &nv;

        // Resolve conflicts
        let moved = (u.clone() ^ &d) | &(r.clone() ^ &l);
        let h_conflicts = r & &l;
        let v_conflicts = u & &d;

        // Put together
        let unmoved =
            h_conflicts.left() | &h_conflicts.right() | &v_conflicts.up() | &v_conflicts.down();
        let inactive = self.clone() & &!active;
        *self = unmoved | &inactive | &moved;
    }

    fn left(&self) -> Self {
        let mut res = self.clone();
        for row in res.0.iter_mut() {
            *row = row.left();
        }
        res
    }

    fn right(&self) -> Self {
        let mut res = self.clone();
        for row in res.0.iter_mut() {
            *row = row.right();
        }
        res
    }

    fn up(&self) -> Self {
        let mut res = self.clone();
        for idx in 0..res.0.len() - 1 {
            res.0[idx] = res.0[idx + 1]
        }
        res
    }

    fn down(&self) -> Self {
        let mut res = self.clone();
        for idx in 0..res.0.len() - 1 {
            res.0[idx + 1] = res.0[idx]
        }
        res
    }
}

impl Not for Grid {
    type Output = Self;

    fn not(mut self) -> Self::Output {
        for row in self.0.iter_mut() {
            *row = !*row;
        }
        self
    }
}

impl BitAndAssign<&Self> for Grid {
    fn bitand_assign(&mut self, rhs: &Self) {
        for (l, r) in self.0.iter_mut().zip(rhs.0.iter()) {
            *l &= *r;
        }
    }
}

impl BitAnd<&Self> for Grid {
    type Output = Self;

    fn bitand(mut self, rhs: &Self) -> Self::Output {
        self &= rhs;
        self
    }
}

impl BitOrAssign<&Self> for Grid {
    fn bitor_assign(&mut self, rhs: &Self) {
        for (l, r) in self.0.iter_mut().zip(rhs.0.iter()) {
            *l |= *r;
        }
    }
}

impl BitOr<&Self> for Grid {
    type Output = Self;

    fn bitor(mut self, rhs: &Self) -> Self::Output {
        self |= rhs;
        self
    }
}

impl BitXorAssign<&Self> for Grid {
    fn bitxor_assign(&mut self, rhs: &Self) {
        for (l, r) in self.0.iter_mut().zip(rhs.0.iter()) {
            *l ^= *r;
        }
    }
}

impl BitXor<&Self> for Grid {
    type Output = Self;

    fn bitxor(mut self, rhs: &Self) -> Self::Output {
        self ^= rhs;
        self
    }
}

#[derive(Clone, Copy)]
struct Row([u64; 3]);

impl Row {
    fn left(mut self) -> Self {
        self.0[2] = self.0[2] << 1 | self.0[1] >> 63;
        self.0[1] = self.0[1] << 1 | self.0[0] >> 63;
        self.0[0] <<= 1;
        self
    }

    fn right(mut self) -> Self {
        self.0[0] = self.0[0] >> 1 | self.0[1] << 63;
        self.0[1] = self.0[1] >> 1 | self.0[2] << 63;
        self.0[2] >>= 1;
        self
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut line = String::new();
        for x in &self.0 {
            let mut x = *x;
            for _ in 0..64 {
                if x & 1 == 1 {
                    line.push('#')
                } else {
                    line.push('.')
                }
                x >>= 1;
            }
        }
        line = line.chars().rev().collect();
        f.write_str(&line)?;
        Ok(())
    }
}

impl Not for Row {
    type Output = Self;

    fn not(mut self) -> Self::Output {
        for row in self.0.iter_mut() {
            *row = !*row;
        }
        self
    }
}

impl BitAndAssign for Row {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.0.iter_mut().zip(rhs.0.iter()) {
            *l &= *r;
        }
    }
}

impl BitAnd for Row {
    type Output = Self;

    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}

impl BitOrAssign for Row {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.0.iter_mut().zip(rhs.0.iter()) {
            *l |= *r;
        }
    }
}

impl BitOr for Row {
    type Output = Self;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}

impl BitXorAssign for Row {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.0.iter_mut().zip(rhs.0.iter()) {
            *l ^= *r;
        }
    }
}

impl BitXor for Row {
    type Output = Self;

    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
