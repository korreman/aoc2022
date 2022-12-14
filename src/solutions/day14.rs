use std::fmt::{Display, Write};

use crate::util::grid::{Grid, Pos};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Air,
    Rock,
    Sand,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Cell::Air => '.',
            Cell::Rock => '#',
            Cell::Sand => 'o',
        };
        f.write_char(c)
    }
}

pub fn run(input: &str) -> (usize, usize) {
    // Parse
    let structures: Vec<Vec<Pos>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|elem| {
                    let (x, y) = elem.split_once(',').unwrap();
                    let x = x.parse().unwrap();
                    let y = y.parse().unwrap();
                    Pos { x, y }
                })
                .collect()
        })
        .collect();
    let spawn = 500;

    let width = 1000;
    let height = structures.iter().flatten().map(|p| p.y).max().unwrap() + 2;

    let mut grid = Grid::new_filled(width, height, Cell::Air);
    for structure in &structures {
        for (a, b) in structure.iter().tuple_windows() {
            let range = if a.x != b.x {
                (usize::min(a.x, b.x)..=usize::max(a.x, b.x))
                    .map(|x| Pos { x, y: a.y })
                    .collect_vec()
            } else {
                (usize::min(a.y, b.y)..=usize::max(a.y, b.y))
                    .map(|y| Pos { x: a.x, y })
                    .collect_vec()
            };

            for pos in range {
                grid[pos] = Cell::Rock;
            }
        }
    }

    // Simulate
    let mut res1 = 0;
    let mut res2 = 0;
    loop {
        let mut p = Pos { x: spawn, y: 0 };
        'falling: loop {
            for &target in &[
                Pos { x: p.x, y: p.y + 1 },
                Pos {
                    x: p.x - 1,
                    y: p.y + 1,
                },
                Pos {
                    x: p.x + 1,
                    y: p.y + 1,
                },
            ] {
                if p.y == grid.height() - 1 {
                    if res1 == 0 {
                        res1 = res2;
                    }
                    break 'falling;
                }
                if grid[target] == Cell::Air {
                    p = target;
                    continue 'falling;
                }
            }
            break 'falling;
        }
        if grid[p] == Cell::Air {
            grid[p] = Cell::Sand;
        } else {
            break;
        }
        res2 += 1;
    }
    (res1, res2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";
        assert_eq!(super::run(input), (24, 93));
    }
}
