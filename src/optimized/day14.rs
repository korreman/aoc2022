use std::fmt::{Display, Write};

use crate::util::grid::{pos, Grid, Pos};
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
    let mut structures: Vec<Vec<Pos>> = input
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

    let width = 1001;
    let height = structures.iter().flatten().map(|p| p.y).max().unwrap() + 3;
    let mut grid = Grid::new_filled(width, height, Cell::Air);

    structures.push(vec![
        pos(0, grid.height() - 1),
        pos(grid.width() - 1, grid.height() - 1),
    ]);
    for structure in &structures {
        for (a, b) in structure.iter().tuple_windows() {
            for pos in a.line(b).unwrap() {
                grid[pos] = Cell::Rock;
            }
        }
    }

    // Simulate

    // Optimization:
    // After a grain has been placed,
    // the next grain will follow the exact same path save for the final position.
    // We track this path and always start at the end of it.
    // (Tracking only the previous position won't work.)
    let mut history = vec![pos(500, 0)];
    let mut res1 = 0;
    let mut res2 = 0;
    while let Some(mut p) = history.pop() {
        'fall: loop {
            for &target_x in &[p.x, p.x - 1, p.x + 1] {
                let target = pos(target_x, p.y + 1);
                if grid[target] == Cell::Air {
                    history.push(p);
                    p = target;
                    continue 'fall;
                }
            }
            break;
        }
        if p.y == grid.height() - 2 && res1 == 0 {
            res1 = res2;
        } else if p.y == 0 {
            return (res1, res2 + 1);
        }
        grid[p] = Cell::Sand;
        res2 += 1;
    }
    (0, 0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";
        assert_eq!(super::run(input), (24, 93));
    }
}
