use std::ops::{Index, IndexMut};

use crate::util::graph::{Graph, GraphImpl};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

pub fn pos(x: usize, y: usize, z: usize) -> Pos {
    Pos { x, y, z }
}

impl Pos {
    pub const ZERO: Self = Pos { x: 0, y: 0, z: 0 };

    // Manhattan distance to other position.
    pub fn dist(&self, other: &Pos) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Clone)]
pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
    depth: usize,
}

impl<T: Clone> Grid<T> {
    pub fn new_filled(width: usize, height: usize, depth: usize, x: T) -> Self {
        let data: Vec<T> = (0..width * height * depth).map(|_| x.clone()).collect();
        Self {
            data,
            width,
            height,
            depth,
        }
    }
}

impl<T> Grid<T> {
    pub fn get(&self, p: Pos) -> Option<&T> {
        if p.x < self.width && p.y < self.height && p.z < self.depth {
            Some(&self.data[p.x + self.width * (p.y + self.height * p.z)])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, p: Pos) -> Option<&mut T> {
        if p.x < self.width && p.y < self.height && p.z < self.depth {
            Some(&mut self.data[p.x + self.width * (p.y + self.height * p.z)])
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

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn contains(&self, p: Pos) -> bool {
        p.x < self.width && p.y < self.height
    }

    pub fn find_pos<P>(&self, pred: P) -> Option<Pos>
    where
        P: Copy + Fn(&T) -> bool,
    {
        for z in 0..self.depth {
            for y in 0..self.height {
                for x in 0..self.width {
                    let pos = Pos { x, y, z };
                    if self.get(pos).map_or(false, pred) {
                        return Some(pos);
                    }
                }
            }
        }
        None
    }

    pub fn for_each<F>(&mut self, mut f: F)
    where
        F: FnMut(Pos, &mut T),
    {
        for z in 0..self.depth {
            for y in 0..self.height {
                for x in 0..self.width {
                    let p = Pos { x, y, z };
                    f(p, &mut self[p])
                }
            }
        }
    }
}

impl<T> Graph<T> for Grid<T> {}
impl<T> GraphImpl<T> for Grid<T> {
    type Node = Pos;

    type Neighbors = Neighbors;
    fn neighbors(&self, node: Pos) -> Self::Neighbors {
        Neighbors {
            center: node,
            width: self.width,
            height: self.height,
            depth: self.depth,
            state: 0,
        }
    }

    fn nodes(&self) -> Self::AllNodes {
        let mut res = Vec::new();
        for z in 0..self.depth {
            for y in 0..self.height {
                for x in 0..self.width {
                    res.push(pos(x, y, z));
                }
            }
        }
        res.into_iter()
    }
    type AllNodes = std::vec::IntoIter<Pos>;

    type Map<U> = Grid<U>;
    fn map<U, F: FnMut(&T) -> U>(&self, mut f: F) -> Self::Map<U> {
        let mut data = Vec::with_capacity(self.width * self.height);
        for z in 0..self.depth {
            for y in 0..self.height {
                for x in 0..self.width {
                    data.push(f(&self[pos(x, y, z)]));
                }
            }
        }
        Grid {
            data,
            width: self.width,
            height: self.height,
            depth: self.depth,
        }
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
    depth: usize,
    state: u8,
}

impl Iterator for Neighbors {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        let mut pos = self.center;
        #[rustfmt::skip]
        let f = |x: &mut usize, length: usize, inc: bool| -> bool {
            if inc { if *x < length - 1 { *x += 1; true } else { false } }
            else if *x > 0 { *x -= 1; true } else { false }
        };
        let b = match self.state {
            0 => f(&mut pos.x, self.width, true),
            1 => f(&mut pos.x, self.width, false),
            2 => f(&mut pos.y, self.height, true),
            3 => f(&mut pos.y, self.height, false),
            4 => f(&mut pos.z, self.depth, true),
            5 => f(&mut pos.z, self.depth, false),
            _ => return None,
        };
        self.state += 1;
        if b {
            Some(pos)
        } else {
            self.next()
        }
    }
}
