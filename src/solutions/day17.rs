use itertools::Itertools;
use std::{
    collections::VecDeque,
    fmt::{Display, Write},
};

// Rows are 1-byte bit vectors.
// Rightmost cell is lowest bit.
// Highest bit is unused.
type Row = u8;
// Shapes are 4 row bytes stored in a u32.
// Lowest row is lowest byte.
type Row4 = u32;

//
const SHAPES: [Row4; 5] = [
    0b00011110,                            // horizontal line
    0b00001000_00011100_00001000,          // cross
    0b00000100_00000100_00011100,          // flipped L
    0b00010000_00010000_00010000_00010000, // vertical line
    0b00011000_00011000,                   // box
];

const WALL_LEFT: Row4 = 0b01000000_01000000_01000000_01000000;
const WALL_RIGHT: Row4 = 0b00000001_00000001_00000001_00000001;

struct Tower {
    /// Number of rows that have been freed.
    forgotten: usize,
    rows: VecDeque<Row>,
}

impl Tower {
    fn new() -> Self {
        Self {
            forgotten: 0,
            rows: VecDeque::from(vec![0b01111111, 0, 0, 0]),
        }
    }

    fn place(&mut self, mut shape: Row4, wind: &mut impl Iterator<Item = u8>) {
        let mut window: Row4 = 0;
        // Ensure a buffer of 4 empty rows above the highest rock
        // (so we can blit our shape)
        while self.rows[self.rows.len() - 4] != 0 {
            self.rows.push_back(0);
        }
        for (row_idx, row_data) in self.rows.iter().enumerate().rev().skip(1) {
            // Attempt to move the shape
            let blown_shape = match wind.next().unwrap() {
                b'<' if shape & WALL_LEFT == 0 => shape << 1,
                b'>' if shape & WALL_RIGHT == 0 => shape >> 1,
                _ => shape,
            };
            // Don't move if blocked by rocks
            if blown_shape & window == 0 {
                shape = blown_shape;
            }
            // Move window down one row
            window = (window << 8) | *row_data as u32;
            // If the shape now collides, place it one row higher than the current window
            if shape & window != 0 {
                for (i, b) in shape.to_le_bytes().iter().enumerate() {
                    self.rows[(row_idx + 1) + i] |= *b;
                }
                break;
            }
        }
    }

    fn height(&self) -> usize {
        self.forgotten + self.rows.iter().enumerate().rev().find(|x| *x.1 != 0).unwrap().0
    }
}

impl Display for Tower {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows.iter().rev() {
            for i in (0..8).rev() {
                let cell = row & (1 << i) != 0;
                if cell {
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
    let mut wind = input.trim().bytes().cycle();
    let mut shapes = SHAPES.iter().cycle();

    let mut board = Tower::new();
    let mut shape = shapes.next().unwrap();
    for _ in 0..2022 {
        //println!("{board}\n");
        board.place(*shape, &mut wind);
        shape = shapes.next().unwrap();
    }
    (board.height(), 0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        assert_eq!(super::run(input), (3068, 1514285714288));
    }
}
