use std::ops::Index;

use crate::util::grid::{pos, Dir4, Grid, Pos, Rot};

pub fn run(input: &str) -> (usize, usize) {
    let (map, insts) = input.trim_end().split_once("\n\n").unwrap();
    let flat_map = FlatMap::new(map);
    let res1 = task(insts, flat_map);
    (res1, 0)
}

fn task<M: MapRep>(mut insts: &str, map: M) -> usize {
    let mut p = map.start();
    while !insts.is_empty() {
        match insts.as_bytes()[0] {
            b'L' => {
                insts = &insts[1..];
                p = p.rotate(Rot::L)
            }
            b'R' => {
                insts = &insts[1..];
                p = p.rotate(Rot::R)
            }
            _ => {
                let (num, tail) = match insts.find(['L', 'R']) {
                    Some(i) => insts.split_at(i),
                    None => (insts, ""),
                };
                insts = tail;
                let num = num.parse::<u32>().unwrap();
                for _ in 0..num {
                    let new_p = map.step_fwd(p);
                    if map[new_p] {
                        p = new_p;
                    } else {
                        break;
                    }
                }
            }
        }
    }
    let (p, dir) = map.result(p);
    (p.y + 1) * 1000
        + (p.x + 1) * 4
        + match dir {
            Dir4::E => 0,
            Dir4::S => 1,
            Dir4::W => 2,
            Dir4::N => 3,
        }
}

// ----- Part 1 types -----
struct FlatMap {
    grid: Grid<Cell>,
    wrap_bounds: WrapBounds,
}

#[derive(Clone, PartialEq, Eq)]
enum Cell {
    Wall,
    Air,
    Nothing,
}

#[derive(Clone, Copy)]
struct FlatPos {
    pos: Pos,
    dir: Dir4,
}

impl WalkPos for FlatPos {
    fn rotate(mut self, rot: Rot) -> Self {
        self.dir = self.dir.rotate(rot);
        self
    }
}

impl Index<FlatPos> for FlatMap {
    type Output = bool;

    fn index(&self, index: FlatPos) -> &Self::Output {
        match self.grid[index.pos] {
            Cell::Wall => &false,
            Cell::Air => &true,
            Cell::Nothing => panic!(),
        }
    }
}

impl MapRep for FlatMap {
    type Pos = FlatPos;

    fn new(map: &str) -> Self {
        let grid = Grid::parse_default(map, Cell::Nothing, |_, c| match c {
            '#' => Cell::Wall,
            '.' => Cell::Air,
            ' ' => Cell::Nothing,
            _ => panic!("unrecognized cell character"),
        });
        let wrap_bounds: WrapBounds = grid.clone().into();
        Self { grid, wrap_bounds }
    }

    fn start(&self) -> Self::Pos {
        let pos = pos(self.wrap_bounds.x_bounds[0].0, 0);
        FlatPos { pos, dir: Dir4::E }
    }

    fn step_fwd(&self, mut p: Self::Pos) -> Self::Pos {
        match p.dir {
            Dir4::N => {
                if p.pos.y == self.wrap_bounds.y_bounds[p.pos.x].0 {
                    p.pos.y = self.wrap_bounds.y_bounds[p.pos.x].1;
                } else {
                    p.pos.y -= 1;
                }
            }
            Dir4::S => {
                if p.pos.y == self.wrap_bounds.y_bounds[p.pos.x].1 {
                    p.pos.y = self.wrap_bounds.y_bounds[p.pos.x].0;
                } else {
                    p.pos.y += 1;
                }
            }
            Dir4::E => {
                if p.pos.x == self.wrap_bounds.x_bounds[p.pos.y].1 {
                    p.pos.x = self.wrap_bounds.x_bounds[p.pos.y].0;
                } else {
                    p.pos.x += 1;
                }
            }
            Dir4::W => {
                if p.pos.x == self.wrap_bounds.x_bounds[p.pos.y].0 {
                    p.pos.x = self.wrap_bounds.x_bounds[p.pos.y].1;
                } else {
                    p.pos.x -= 1;
                }
            }
        }
        p
    }

    fn result(&self, p: Self::Pos) -> (Pos, Dir4) {
        (p.pos, p.dir)
    }
}

#[derive(Debug)]
struct WrapBounds {
    x_bounds: Vec<(usize, usize)>,
    y_bounds: Vec<(usize, usize)>,
}

impl From<Grid<Cell>> for WrapBounds {
    fn from(grid: Grid<Cell>) -> Self {
        let x_bounds = (0..grid.height())
            .map(|y| {
                let mut start = pos(0, y);
                while grid.get(start) == Some(&Cell::Nothing) {
                    start.x += 1;
                }
                let mut end = pos(grid.width() - 1, y);
                while grid[end] == Cell::Nothing {
                    end.x -= 1;
                }
                (start.x, end.x)
            })
            .collect();
        let y_bounds = (0..grid.width())
            .map(|x| {
                let mut start = pos(x, 0);
                while grid.get(start) == Some(&Cell::Nothing) {
                    start.y += 1;
                }
                let mut end = pos(x, grid.height() - 1);
                while grid[end] == Cell::Nothing {
                    end.y -= 1;
                }
                (start.y, end.y)
            })
            .collect();
        Self { x_bounds, y_bounds }
    }
}

// ----- Part 2 types -----
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct CubePos {
    face: usize,
    pos: Pos,
    dir: Dir4,
}

impl WalkPos for CubePos {
    fn rotate(mut self, rot: Rot) -> Self {
        self.dir = self.dir.rotate(rot);
        self
    }
}

struct Face {
    surface: Grid<bool>,
    sides: [(usize, Rot); 4],
}

struct Cube {
    width: usize,
    faces: [Face; 6],
}

impl Index<CubePos> for Cube {
    type Output = bool;

    fn index(&self, index: CubePos) -> &Self::Output {
        &self.faces[index.face].surface[index.pos]
    }
}

impl MapRep for Cube {
    type Pos = CubePos;

    fn new(map: &str) -> Self {
        todo!()
    }

    fn start(&self) -> Self::Pos {
        todo!()
    }

    fn step_fwd(&self, mut p: CubePos) -> CubePos {
        p.pos += pos(self.width, self.width);
        p.pos = p.pos.step(p.dir);
        p.pos.x %= self.width;
        p.pos.y %= self.width;
        if p.pos.x == self.width || p.pos.x == 0 || p.pos.y == self.width || p.pos.y == 0 {
            // retrieve new face and coordinate
            let (new_face, rotation) = self.faces[p.face].sides[p.dir.flip().to_idx()];
            // switch to new face
            p.face = new_face;
            // rotate to enter new face correctly
            p.dir = p.dir.rotate(rotation);
            match rotation {
                Rot::L => {
                    let temp = p.pos.x;
                    p.pos.x = self.width - p.pos.y;
                    p.pos.y = temp;
                }
                Rot::R => {
                    let temp = p.pos.x;
                    p.pos.x = p.pos.y;
                    p.pos.y = self.width - temp;
                }
            }
        }
        p
    }

    fn result(&self, p: Self::Pos) -> (Pos, Dir4) {
        todo!()
    }
}

trait MapRep: Index<Self::Pos, Output = bool> {
    type Pos: WalkPos;
    fn new(map: &str) -> Self;
    fn start(&self) -> Self::Pos;
    fn step_fwd(&self, p: Self::Pos) -> Self::Pos;
    fn result(&self, p: Self::Pos) -> (Pos, Dir4);
}

trait WalkPos: Clone + Copy {
    fn rotate(self, rot: Rot) -> Self;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let input = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";
        assert_eq!(super::run(input), (6032, 5031));
    }
}
