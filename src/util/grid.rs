use std::{
    fmt::{Display, Write},
    ops::{Add, AddAssign, Index, IndexMut},
};

use crate::util::graph::{Graph, GraphImpl};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    pub fn step(self, dir: Dir4) -> Self {
        match dir {
            Dir4::N => pos(self.x, self.y - 1),
            Dir4::E => pos(self.x + 1, self.y),
            Dir4::S => pos(self.x, self.y + 1),
            Dir4::W => pos(self.x - 1, self.y),
        }
    }

    pub fn step_checked(self, dir: Dir4) -> Option<Self> {
        match dir {
            Dir4::N => {
                if self.y == 0 {
                    None
                } else {
                    Some(pos(self.x, self.y - 1))
                }
            }
            Dir4::E => Some(pos(self.x + 1, self.y)),
            Dir4::S => Some(pos(self.x, self.y + 1)),
            Dir4::W => {
                if self.x == 0 {
                    None
                } else {
                    Some(pos(self.x - 1, self.y))
                }
            }
        }
    }

    pub fn step_dir8(self, dir: Dir8) -> Self {
        match dir {
            Dir8::NO => pos(self.x, self.y - 1),
            Dir8::NE => pos(self.x + 1, self.y - 1),
            Dir8::EA => pos(self.x + 1, self.y),
            Dir8::SE => pos(self.x + 1, self.y + 1),
            Dir8::SO => pos(self.x, self.y + 1),
            Dir8::SW => pos(self.x - 1, self.y + 1),
            Dir8::WE => pos(self.x - 1, self.y),
            Dir8::NW => pos(self.x - 1, self.y - 1),
        }
    }

    pub fn swap_xy(self) -> Self {
        Pos {
            x: self.y,
            y: self.x,
        }
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add<Pos> for Pos {
    type Output = Pos;

    fn add(self, rhs: Pos) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Rot {
    L,
    R,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Dir4 {
    N,
    E,
    S,
    W,
}

impl Dir4 {
    pub fn rotate(self, r: Rot) -> Self {
        match r {
            Rot::L => match self {
                Dir4::E => Dir4::N,
                Dir4::S => Dir4::E,
                Dir4::W => Dir4::S,
                Dir4::N => Dir4::W,
            },
            Rot::R => match self {
                Dir4::E => Dir4::S,
                Dir4::S => Dir4::W,
                Dir4::W => Dir4::N,
                Dir4::N => Dir4::E,
            },
        }
    }

    pub fn rotate_relative(mut self, mut r: Self) -> Self {
        while r != Dir4::N {
            self = self.rotate(Rot::R);
            r = r.rotate(Rot::L);
        }
        self
    }

    pub fn flip(self) -> Self {
        match self {
            Dir4::N => Dir4::S,
            Dir4::E => Dir4::W,
            Dir4::S => Dir4::N,
            Dir4::W => Dir4::E,
        }
    }

    pub fn flip_x(self) -> Self {
        match self {
            Dir4::N => Dir4::N,
            Dir4::E => Dir4::W,
            Dir4::S => Dir4::S,
            Dir4::W => Dir4::E,
        }
    }

    /// Establish an index standard for lookup tables
    pub fn to_idx(self) -> usize {
        match self {
            Dir4::N => 0,
            Dir4::E => 1,
            Dir4::S => 2,
            Dir4::W => 3,
        }
    }

    /// Establish an index standard for lookup tables
    pub fn from_idx(idx: usize) -> Option<Self> {
        match idx {
            0 => Some(Dir4::N),
            1 => Some(Dir4::E),
            2 => Some(Dir4::S),
            3 => Some(Dir4::W),
            _ => None,
        }
    }
}

impl std::fmt::Display for Dir4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Dir4::N => '↑',
            Dir4::E => '→',
            Dir4::S => '↓',
            Dir4::W => '←',
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Dir8 {
    NO,
    NE,
    EA,
    SE,
    SO,
    SW,
    WE,
    NW,
}

impl Dir8 {
    pub fn rotate(self, r: Rot) -> Self {
        match r {
            Rot::L => match self {
                Dir8::NO => Dir8::NW,
                Dir8::NE => Dir8::NO,
                Dir8::EA => Dir8::NE,
                Dir8::SE => Dir8::EA,
                Dir8::SO => Dir8::SE,
                Dir8::SW => Dir8::SO,
                Dir8::WE => Dir8::SW,
                Dir8::NW => Dir8::WE,
            },
            Rot::R => match self {
                Dir8::NO => Dir8::NE,
                Dir8::NE => Dir8::EA,
                Dir8::EA => Dir8::SE,
                Dir8::SE => Dir8::SO,
                Dir8::SO => Dir8::SW,
                Dir8::SW => Dir8::WE,
                Dir8::WE => Dir8::NW,
                Dir8::NW => Dir8::NO,
            },
        }
    }

    pub fn flip(self) -> Self {
        match self {
            Dir8::NO => Dir8::SO,
            Dir8::NE => Dir8::SW,
            Dir8::EA => Dir8::WE,
            Dir8::SE => Dir8::NW,
            Dir8::SO => Dir8::NO,
            Dir8::SW => Dir8::NE,
            Dir8::WE => Dir8::EA,
            Dir8::NW => Dir8::SE,
        }
    }

    /// Establish an index standard for lookup tables
    pub fn to_idx(self) -> usize {
        match self {
            Dir8::NO => 0,
            Dir8::EA => 1,
            Dir8::SO => 2,
            Dir8::WE => 3,
            Dir8::NE => 4,
            Dir8::SE => 5,
            Dir8::SW => 6,
            Dir8::NW => 7,
        }
    }
}

impl std::fmt::Display for Dir8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Dir8::NO => '↑',
            Dir8::NE => '↗',
            Dir8::EA => '→',
            Dir8::SE => '↘',
            Dir8::SO => '↓',
            Dir8::SW => '↙',
            Dir8::WE => '←',
            Dir8::NW => '↖',
        })
    }
}

impl From<Dir4> for Dir8 {
    fn from(val: Dir4) -> Self {
        match val {
            Dir4::N => Dir8::NO,
            Dir4::E => Dir8::EA,
            Dir4::S => Dir8::SO,
            Dir4::W => Dir8::WE,
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

    pub fn pad(&self, amount: usize, value: T) -> Self {
        let mut result = Self::new_filled(self.width + amount * 2, self.height + amount * 2, value);
        for x in 0..self.width {
            for y in 0..self.height {
                result[pos(x + amount, y + amount)] = self[pos(x, y)].clone();
            }
        }
        result
    }

    pub fn crop(&self, amount: usize, dummy: T) -> Self {
        let mut result = Self::new_filled(self.width - amount * 2, self.height - amount * 2, dummy);
        for x in 0..result.width {
            for y in 0..result.height {
                result[pos(x, y)] = self[pos(x + amount, y + amount)].clone();
            }
        }
        result
    }

    pub fn parse_default<P>(input: &str, default: T, mut p: P) -> Self
    where
        P: FnMut(Pos, char) -> T,
    {
        let height = input.lines().count();
        let width = input
            .lines()
            .map(|line| line.chars().count())
            .max()
            .unwrap();
        let mut grid = Grid::new_filled(width, height, default);
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let dst = pos(x, y);
                grid[dst] = p(dst, c);
            }
        }
        grid
    }
}

impl<T: Default + Clone> Grid<T> {
    pub fn crop_area(&self, start: Pos, end: Pos) -> Self {
        let width = end.x - start.x;
        let height = end.y - start.y;
        let mut result = Self::new_filled(width, height, T::default());
        for x in 0..width {
            for y in 0..height {
                result[pos(x, y)] = self[pos(x + start.x, y + start.y)].clone();
            }
        }
        result
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

impl<T> Graph<T> for Grid<T> {}
impl<T> GraphImpl<T> for Grid<T> {
    type Node = Pos;

    fn neighbors(&self, node: Pos) -> Self::Neighbors {
        Neighbors {
            center: node,
            width: self.width,
            height: self.height,
            state: 0,
        }
    }
    type Neighbors = Neighbors;

    fn nodes(&self) -> Self::AllNodes {
        let mut res = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                res.push(pos(x, y));
            }
        }
        res.into_iter()
    }
    type AllNodes = std::vec::IntoIter<Pos>;

    fn map<U, F: FnMut(&T) -> U>(&self, mut f: F) -> Self::Map<U> {
        let mut data = Vec::with_capacity(self.width * self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                data.push(f(&self[pos(x, y)]));
            }
        }
        Grid {
            data,
            width: self.width,
            height: self.height,
        }
    }
    type Map<U> = Grid<U>;
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
            } else if *x > 0 {
                *x -= 1;
                true
            } else {
                false
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
