use std::{
    fmt::{Display, Write},
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

pub fn pos(x: usize, y: usize) -> Pos {
    Pos { x, y }
}

impl Pos {
    pub const ZERO: Self = Pos { x: 0, y: 0 };

    // Manhattan distance to other position.
    pub fn dist(&self, other: &Pos) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    /// Generates a sequence of positions in a line from self to other (inclusive).
    pub fn line(self, other: &Self) -> Option<Line> {
        if self.x != other.x && self.y != other.y {
            None
        } else {
            Some(Line {
                current: self,
                target: other,
                done: false,
            })
        }
    }
}

pub struct Line<'a> {
    current: Pos,
    target: &'a Pos,
    done: bool,
}

impl<'a> Iterator for Line<'a> {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.current;
        if self.done {
            return None;
        } else if self.current.x < self.target.x {
            self.current.x += 1;
        } else if self.current.x > self.target.x {
            self.current.x -= 1;
        } else if self.current.y < self.target.y {
            self.current.y += 1;
        } else if self.current.y > self.target.y {
            self.current.y -= 1;
        } else {
            self.done = true;
        }
        Some(res)
    }
}

#[derive(Clone)]
pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Clone> Grid<T> {
    pub fn new_filled(width: usize, height: usize, x: T) -> Self {
        let data: Vec<T> = (0..width * height).map(|_| x.clone()).collect();
        Self {
            data,
            width,
            height,
        }
    }
}

impl<T> Grid<T> {
    pub fn parse<P>(input: &str, mut p: P) -> Self
    where
        P: FnMut(Pos, char) -> T,
    {
        let mut data = Vec::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let v = p(Pos { x, y }, c);
                data.push(v);
            }
        }
        let width = input.lines().next().unwrap().chars().count();
        let height = data.len() / width;

        Grid {
            data,
            width,
            height,
        }
    }

    pub fn get(&self, p: Pos) -> Option<&T> {
        if p.x < self.width && p.y < self.height {
            Some(&self.data[p.x + p.y * self.width])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, p: Pos) -> Option<&mut T> {
        if p.x < self.width && p.y < self.height {
            Some(&mut self.data[p.x + p.y * self.width])
        } else {
            None
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn contains(&self, p: Pos) -> bool {
        p.x < self.width && p.y < self.height
    }

    pub fn neighbors(&self, p: Pos) -> Neighbors {
        Neighbors {
            center: p,
            width: self.width,
            height: self.height,
            state: 0,
        }
    }

    pub fn find_pos<P>(&self, p: P) -> Option<Pos>
    where
        P: Copy + Fn(&T) -> bool,
    {
        for x in 0..self.width {
            for y in 0..self.height {
                let pos = Pos { x, y };
                if self.get(pos).map_or(false, p) {
                    return Some(pos);
                }
            }
        }
        None
    }

    pub fn for_each<F>(&mut self, mut f: F)
    where
        F: FnMut(Pos, &mut T),
    {
        for y in 0..self.height {
            for x in 0..self.width {
                let p = Pos { x, y };
                f(p, &mut self[p])
            }
        }
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                self.data[x + y * self.width].fmt(f)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl<T> Index<Pos> for Grid<T> {
    type Output = T;

    fn index(&self, index: Pos) -> &Self::Output {
        self.get(index).expect("index out of bounds")
    }
}

impl<T> IndexMut<Pos> for Grid<T> {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        self.get_mut(index).expect("index out of bounds")
    }
}

pub struct Neighbors {
    center: Pos,
    width: usize,
    height: usize,
    state: u8,
}

impl Iterator for Neighbors {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        let mut pos = self.center;
        let f = |x: &mut usize, length: usize, inc: bool| -> bool {
            if inc {
                if *x < length - 1 {
                    *x += 1;
                    true
                } else {
                    false
                }
            } else {
                if *x > 0 {
                    *x -= 1;
                    true
                } else {
                    false
                }
            }
        };
        self.state += 1;
        let b = match self.state {
            1 => f(&mut pos.x, self.width, true),
            2 => f(&mut pos.y, self.height, true),
            3 => f(&mut pos.x, self.width, false),
            4 => f(&mut pos.y, self.height, false),
            _ => return None,
        };
        if b {
            Some(pos)
        } else {
            self.next()
        }
    }
}

#[derive(Clone)]
pub struct BitGrid {
    data: Vec<u32>,
    width: usize,
    height: usize,
}

impl BitGrid {
    pub fn new(width: usize, height: usize, init: bool) -> Self {
        let data: Vec<u32> = (0..(width * height) / 32 + 1)
            .map(|_| if init { u32::MAX } else { 0 })
            .collect();
        Self {
            data,
            width,
            height,
        }
    }

    fn get_idxs(&self, p: Pos) -> Option<(usize, u32)> {
        if p.x < self.width && p.y < self.height {
            let idx = p.x + p.y * self.width;
            let chunk_idx = idx >> 5;
            let bit = 1 << (idx & 0x1F);
            Some((chunk_idx, bit))
        } else {
            None
        }
    }

    pub fn get(&self, p: Pos) -> Option<bool> {
        let (chunk_idx, bit) = self.get_idxs(p)?;
        Some((self.data[chunk_idx] & bit) != 0)
    }

    pub fn set(&mut self, p: Pos, value: bool) {
        if let Some((chunk_idx, bit)) = self.get_idxs(p) {
            if value {
                self.data[chunk_idx] |= bit;
            } else {
                self.data[chunk_idx] &= !bit;
            }
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl Display for BitGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(Pos { x, y }).unwrap() {
                    f.write_char('.')?
                } else {
                    f.write_char('#')?
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}
