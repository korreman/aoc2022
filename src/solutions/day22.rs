use std::{
    collections::HashMap,
    fmt::{Debug, Write},
    ops::Index,
};

use itertools::Itertools;

use crate::util::{
    graph::GraphImpl,
    grid::{pos, Dir4, Grid, Pos, Rot},
};

pub fn run(input: &str) -> (usize, usize) {
    run_size::<50>(input)
}

pub fn run_size<const N: usize>(input: &str) -> (usize, usize) {
    let (map, insts) = input.trim_end().split_once("\n\n").unwrap();
    let grid = Grid::parse_default(map, Cell::Nothing, |_, c| match c {
        '#' => Cell::Wall,
        '.' => Cell::Air,
        ' ' => Cell::Nothing,
        _ => panic!("unrecognized cell character"),
    });
    let flat_map = FlatMap::new(&grid);
    let res1 = task(insts, flat_map);
    let cube_map = CubeMap::<N>::new(&grid);
    let res2 = task(insts, cube_map);
    (res1, res2)
}

trait MapRep: Index<Self::Pos, Output = bool> {
    type Pos: WalkPos;
    fn new(map: &Grid<Cell>) -> Self;
    fn start(&self) -> Self::Pos;
    fn step_fwd(&self, p: Self::Pos) -> Self::Pos;
    fn result(&self, p: Self::Pos) -> (Pos, Dir4);
}

trait WalkPos: Clone + Copy + Debug {
    fn rotate(self, rot: Rot) -> Self;
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
    map: Grid<Cell>,
    wrap_bounds: WrapBounds,
}

#[derive(Clone, PartialEq, Eq, Default)]
enum Cell {
    Wall,
    #[default]
    Air,
    Nothing,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Cell::Wall => '#',
            Cell::Air => '.',
            Cell::Nothing => ' ',
        })
    }
}

#[derive(Clone, Copy, Debug)]
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
        match self.map[index.pos] {
            Cell::Wall => &false,
            Cell::Air => &true,
            Cell::Nothing => panic!("index outside flatmap bounds (a Nothing cell)"),
        }
    }
}

impl MapRep for FlatMap {
    type Pos = FlatPos;

    fn new(map: &Grid<Cell>) -> Self {
        let wrap_bounds: WrapBounds = map.clone().into();
        Self {
            map: map.clone(),
            wrap_bounds,
        }
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
    sides: [(usize, Dir4); 4],
    grid_pos: Pos,
}

impl std::fmt::Debug for Face {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("face {:?}", self.grid_pos))
    }
}

struct CubeMap<const N: usize> {
    start: usize,
    faces: [Face; 6],
}

impl<const N: usize> Index<CubePos> for CubeMap<N> {
    type Output = bool;

    fn index(&self, index: CubePos) -> &Self::Output {
        &self.faces[index.face].surface[index.pos]
    }
}

impl<const N: usize> MapRep for CubeMap<N> {
    type Pos = CubePos;

    fn new(map: &Grid<Cell>) -> Self {
        // Collect faces.
        let mut faces = HashMap::new();
        let mut start = None;
        for y in 0..(map.height() / N) {
            for x in 0..(map.width() / N) {
                let corner = pos(x * N, y * N);
                if map[corner] != Cell::Nothing {
                    faces.insert(pos(x, y), map.crop_area(corner, corner + pos(N, N)));
                    if start.is_none() {
                        start = Some(pos(x, y));
                    }
                }
            }
        }

        // Get immediate neighbors.
        let mut neighbor_map = hashbrown::HashMap::new();
        for face in faces.keys() {
            for dir in [Dir4::N, Dir4::E, Dir4::S, Dir4::W] {
                if let Some(neighbor) = face.step_checked(dir) {
                    if faces.contains_key(&neighbor) {
                        neighbor_map
                            .entry(*face)
                            .or_insert([None, None, None, None])[dir.to_idx()] =
                            Some((neighbor, Dir4::N));
                    }
                }
            }
        }

        // Repeatedly form connections until fully saturated.
        while neighbor_map
            .values()
            .flat_map(|a| a.iter())
            .any(|x| x.is_none())
        {
            // Collect new edges that can be formed.
            let mut new_edges = Vec::new();
            for (face, face_neighbors) in &neighbor_map {
                for (dir_idx, neighbor) in face_neighbors.iter().enumerate() {
                    if let Some((neighbor, n_orient)) = neighbor {
                        // Retrieve the direction the neighbor was in.
                        let n_dir = Dir4::from_idx(dir_idx).unwrap();
                        // Then figure out the direction to get to the neighbors' neighbor.
                        let nn_dir = n_dir.rotate(Rot::R).rotate_relative(n_orient.flip_x());
                        // Check if the neighbor has that neighbor.
                        let n_neighbors = neighbor_map[neighbor];
                        if let Some((nn, nn_orient)) = n_neighbors[nn_dir.to_idx()] {
                            // If so, the neighbors' neighbor should be connected to this face.
                            let relative_orient =
                                nn_orient.rotate_relative(*n_orient).rotate(Rot::R);
                            new_edges.push((*face, n_dir.rotate(Rot::R), nn, relative_orient));
                        }
                    }
                }
            }

            // Form new edges.
            for (face, dir, nn, relative_orient) in new_edges.drain(..) {
                let face_entry = neighbor_map.get_mut(&face).unwrap();
                face_entry[dir.to_idx()].get_or_insert((nn, relative_orient));
            }
        }
        // Resolve positions to indices
        let idxs: HashMap<Pos, usize> = faces.keys().enumerate().map(|(p, &i)| (i, p)).collect();
        let start = idxs[&start.unwrap()];
        // Package for output
        let mut faces = faces
            .iter()
            .map(|(grid_pos, grid)| {
                let neighbors = neighbor_map[grid_pos].map(|neighbor| {
                    let (n_pos, orient) = neighbor.unwrap();
                    (idxs[&n_pos], orient)
                });
                (idxs[grid_pos], grid_pos, grid, neighbors)
            })
            .collect_vec();
        faces.sort_unstable_by_key(|x| x.0);
        let faces = faces
            .into_iter()
            .map(|(_, &grid_pos, surface, sides)| Face {
                surface: surface.map(|cell| cell == &Cell::Air),
                sides,
                grid_pos: pos(grid_pos.x * N, grid_pos.y * N),
            })
            .collect_vec();
        let faces = faces.try_into().unwrap();
        Self { faces, start }
    }

    fn start(&self) -> CubePos {
        CubePos {
            face: self.start,
            pos: pos(0, 0),
            dir: Dir4::E,
        }
    }

    fn step_fwd(&self, mut p: CubePos) -> CubePos {
        p.pos += pos(N, N);
        p.pos = p.pos.step(p.dir);
        let overstep = p.pos.x == N - 1 || p.pos.x == N + N || p.pos.y == N - 1 || p.pos.y == N + N;
        p.pos.x %= N;
        p.pos.y %= N;
        if overstep {
            // retrieve new face and coordinate
            let (new_face, new_orient) = self.faces[p.face].sides[p.dir.to_idx()];
            // switch to new face
            p.face = new_face;
            // rotate to enter new face correctly
            // that is, the opposite of the direction that face is facing
            match new_orient {
                Dir4::N => (),
                Dir4::E => {
                    p.dir = p.dir.rotate(Rot::L);
                    p.pos = p.pos.swap_xy();
                    p.pos.y = N - 1 - p.pos.y;
                }
                Dir4::W => {
                    p.dir = p.dir.rotate(Rot::R);
                    p.pos = p.pos.swap_xy();
                    p.pos.x = N - 1 - p.pos.x;
                }
                Dir4::S => {
                    p.dir = p.dir.flip();
                    p.pos.x = N - 1 - p.pos.x;
                    p.pos.y = N - 1 - p.pos.y;
                }
            }
        }
        p
    }

    fn result(&self, p: CubePos) -> (Pos, Dir4) {
        (self.faces[p.face].grid_pos + p.pos, p.dir)
    }
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

10R5L5R10L4R5L5\n\
        ";
        assert_eq!(super::run_size::<4>(input), (6032, 5031));
    }
}
