use std::fmt::{Display, Write};

use crate::util::grid::{pos, Pos};

type Shape = Vec<Pos>;

struct Board {
    grid: Vec<[bool; 9]>,
}

impl Board {
    fn new() -> Self {
        Self {
            grid: vec![[true; 9]],
        }
    }

    fn next_pos(&self) -> Pos {
        pos(3, self.grid.len() + 3)
    }

    fn test_pos(&self, p: Pos) -> bool {
        if !(1..=7).contains(&p.x) {
            false
        } else if self.grid.len() <= p.y {
            true
        } else {
            !self.grid[p.y][p.x]
        }
    }

    fn test_shape(&self, p: Pos, shape: &Shape) -> bool {
        shape.iter().all(|sp| {
            let point = pos(p.x + sp.x, p.y + sp.y);
            self.test_pos(point)
        })
    }

    const EMPTY_ROW: [bool; 9] = [true, false, false, false, false, false, false, false, true];
    fn add_shape(&mut self, p: Pos, shape: &Shape) {
        for sp in shape {
            let point = pos(p.x + sp.x, p.y + sp.y);
            while point.y >= self.grid.len() {
                self.grid.push(Self::EMPTY_ROW);
            }
            self.grid[point.y][point.x] = true;
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter().rev() {
            for cell in row {
                if *cell {
                    f.write_char('#')?;
                } else {
                    f.write_char('.')?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

pub fn run(input: &str) -> (usize, usize) {
    let shapes: [Shape; 5] = [
        vec![pos(0, 0), pos(1, 0), pos(2, 0), pos(3, 0)],
        vec![pos(1, 0), pos(0, 1), pos(1, 1), pos(2, 1), pos(1, 2)],
        vec![pos(0, 0), pos(1, 0), pos(2, 0), pos(2, 1), pos(2, 2)],
        vec![pos(0, 0), pos(0, 1), pos(0, 2), pos(0, 3)],
        vec![pos(0, 0), pos(1, 0), pos(0, 1), pos(1, 1)],
    ];
    let mut wind = input.trim().bytes().cycle();
    let mut shapes = shapes.iter().cycle();
    let mut shape = shapes.next().unwrap();
    let mut board = Board::new();
    let mut p = board.next_pos();
    for i in 0..2022 {
        loop {
            let wind_p = match wind.next().unwrap() {
                b'>' => pos(p.x + 1, p.y),
                b'<' => pos(p.x - 1, p.y),
                _ => panic!(),
            };
            if board.test_shape(wind_p, shape) {
                p = wind_p;
            }
            let fall_p = pos(p.x, p.y - 1);
            if board.test_shape(fall_p, shape) {
                p = fall_p;
            } else {
                board.add_shape(p, shape);
                shape = shapes.next().unwrap();
                p = board.next_pos();
                break;
            }
        }
    }
    (board.grid.len() - 1, 0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        assert_eq!(super::run(input), (3068, 1514285714288));
    }
}
