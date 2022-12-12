use std::{
    fmt::{Display, Write},
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub const ZERO: Self = Pos { x: 0, y: 0 };

    pub fn dist(&self, other: &Pos) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
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
