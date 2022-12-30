use std::fmt::{Display, Write};

use crate::util::{
    graph::GraphImpl,
    grid::{pos, Grid, Pos},
};

#[derive(Clone, Copy)]
enum Dir {
    E,
    S,
    W,
    N,
}

impl Dir {
    fn to_char(self) -> char {
        match self {
            Dir::E => '>',
            Dir::S => 'v',
            Dir::W => '<',
            Dir::N => '^',
        }
    }
    fn turn_left(self) -> Self {
        match self {
            Dir::E => Dir::N,
            Dir::S => Dir::E,
            Dir::W => Dir::S,
            Dir::N => Dir::W,
        }
    }
    fn turn_right(self) -> Self {
        match self {
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
            Dir::N => Dir::E,
        }
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Dir::E => '>',
            Dir::S => 'v',
            Dir::W => '<',
            Dir::N => '^',
        };
        f.write_char(c)
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Cell {
    Wall,
    Air,
    Nothing,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Cell::Wall => '#',
            Cell::Air => '.',
            Cell::Nothing => ' ',
        };
        f.write_char(c)
    }
}

#[derive(Debug)]
struct WrapBounds {
    x_bounds: Vec<(usize, usize)>,
    y_bounds: Vec<(usize, usize)>,
}

impl WrapBounds {
    fn walk_wrapped(&self, mut p: Pos, dir: Dir) -> Pos {
        match dir {
            Dir::N => {
                if p.y == self.y_bounds[p.x].0 {
                    p.y = self.y_bounds[p.x].1;
                } else {
                    p.y -= 1;
                }
            }
            Dir::S => {
                if p.y == self.y_bounds[p.x].1 {
                    p.y = self.y_bounds[p.x].0;
                } else {
                    p.y += 1;
                }
            }
            Dir::E => {
                if p.x == self.x_bounds[p.y].1 {
                    p.x = self.x_bounds[p.y].0;
                } else {
                    p.x += 1;
                }
            }
            Dir::W => {
                if p.x == self.x_bounds[p.y].0 {
                    p.x = self.x_bounds[p.y].1;
                } else {
                    p.x -= 1;
                }
            }
        }
        p
    }
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

pub fn run(input: &str) -> (usize, usize) {
    let (a, mut b) = input.trim_end().split_once("\n\n").unwrap();
    let grid = Grid::parse_default(a, Cell::Nothing, |_, c| match c {
        '#' => Cell::Wall,
        '.' => Cell::Air,
        ' ' => Cell::Nothing,
        _ => panic!("unrecognized cell character"),
    });
    let mut drawn_path = grid.map(|cell| match cell {
        Cell::Wall => '#',
        Cell::Air => '.',
        Cell::Nothing => ' ',
    });
    let bounds: WrapBounds = grid.clone().into();
    let mut p = pos(bounds.x_bounds[0].0, 0);
    let mut dir = Dir::E;
    while !b.is_empty() {
        match b.as_bytes()[0] {
            b'L' => {
                b = &b[1..];
                dir = dir.turn_left()
            }
            b'R' => {
                b = &b[1..];
                dir = dir.turn_right()
            }
            _ => {
                let (num, tail) = match b.find(['L', 'R']) {
                    Some(i) => b.split_at(i),
                    None => (b, ""),
                };
                b = tail;
                let num = num.parse::<u32>().unwrap();
                for _ in 0..num {
                    let new_p = bounds.walk_wrapped(p, dir);
                    if grid[new_p] != Cell::Wall {
                        drawn_path[p] = dir.to_char();
                        p = new_p;
                    } else {
                        break;
                    }
                }
            }
        }
    }
    drawn_path[p] = 'x';
    //println!("{grid}");
    //println!("{drawn_path}");
    let res1 = (p.y + 1) * 1000
        + (p.x + 1) * 4
        + match dir {
            Dir::E => 0,
            Dir::S => 1,
            Dir::W => 2,
            Dir::N => 3,
        };
    (res1, 0)
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
