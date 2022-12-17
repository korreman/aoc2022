use std::{
    collections::{HashMap, VecDeque},
    fmt::{Display, Write},
};

// Rows are 1-byte bit vectors.
// Rightmost cell is lowest bit.
// Highest bit is unused.
type Row = u8;
// Shapes are 4 row bytes stored in a u32.
// Lowest row is lowest byte.
type Row4 = u32;
type Row8 = u64;

//
const ROCKS: [Row4; 5] = [
    0b00011110,                            // horizontal line
    0b00001000_00011100_00001000,          // cross
    0b00000100_00000100_00011100,          // flipped L
    0b00010000_00010000_00010000_00010000, // vertical line
    0b00011000_00011000,                   // box
];

const WALL_LEFT: Row4 = 0b01000000_01000000_01000000_01000000;
const WALL_RIGHT: Row4 = 0b00000001_00000001_00000001_00000001;

#[derive(PartialEq, Eq, Hash, Debug)]
struct HistoryKey {
    wind_state: usize,
    rock: Row4,
    bottom: Row8,
}

#[derive(Debug, Clone, Copy)]
struct HistoryEntry {
    height: usize,
    rocks: usize,
}

struct Tower {
    /// Number of rows that have been freed.
    forgotten: usize,
    rows: VecDeque<Row>,
    history: HashMap<HistoryKey, HistoryEntry>,
}

impl Tower {
    fn new() -> Self {
        Self {
            forgotten: 0,
            rows: VecDeque::from(vec![0b01111111, 0, 0, 0, 0]),
            history: HashMap::new(),
        }
    }

    fn place(
        &mut self,
        mut rock: Row4,
        rocks: usize,
        wind: &mut impl Iterator<Item = (usize, u8)>,
    ) -> Option<(HistoryEntry, HistoryEntry)> {
        // SEARCH
        let mut window: Row4 = 0;
        let mut target_row = None;
        let mut curr_wind = (0, 0);
        for (row_idx, row_data) in self.rows.iter().enumerate().rev().skip(1) {
            // Attempt to move the rock
            curr_wind = wind.next().unwrap();
            let blown_rock = match curr_wind.1 {
                b'<' if rock & WALL_LEFT == 0 => rock << 1,
                b'>' if rock & WALL_RIGHT == 0 => rock >> 1,
                _ => rock,
            };
            // Don't move if blocked by rocks
            if blown_rock & window == 0 {
                rock = blown_rock;
            }
            // Move window down one row
            window = (window << 8) | *row_data as u32;
            // If the rock now collides, the target row is the previous one
            if rock & window != 0 {
                target_row = Some(row_idx + 1);
                break;
            }
        }

        // PLACE
        let target_row = target_row.unwrap();
        let mut blocked = 0;
        for (i, b) in rock.to_le_bytes().iter().enumerate() {
            self.rows[target_row + i] |= *b;
            blocked |= self.rows[target_row + i];
        }
        // Ensure a buffer of 4 empty rows above the highest rock
        // (so we can blit our rock)
        while self.rows[self.rows.len() - 4] != 0 {
            self.rows.push_back(0);
        }

        // SHORTEN, DETECT CYCLES
        if blocked == 0b01111111 {
            self.forgotten += target_row;
            drop(self.rows.drain(0..target_row));

            if self.rows.iter().take_while(|&&row| row != 0).count() <= 8 {
                let bottom = u64::from_le_bytes([
                    *self.rows.get(0).unwrap_or(&0),
                    *self.rows.get(1).unwrap_or(&0),
                    *self.rows.get(2).unwrap_or(&0),
                    *self.rows.get(3).unwrap_or(&0),
                    *self.rows.get(4).unwrap_or(&0),
                    *self.rows.get(5).unwrap_or(&0),
                    *self.rows.get(6).unwrap_or(&0),
                    *self.rows.get(7).unwrap_or(&0),
                ]);
                let key = HistoryKey {
                    wind_state: curr_wind.0,
                    rock,
                    bottom,
                };

                let entry = HistoryEntry {
                    height: self.forgotten,
                    rocks: rocks,
                };

                if let Some(pentry) = self.history.get(&key) {
                    return Some((*pentry, entry));
                } else {
                    self.history.insert(key, entry);
                }
            }
        }
        None
    }

    fn height(&self) -> usize {
        self.forgotten
            + self
                .rows
                .iter()
                .enumerate()
                .rev()
                .find(|x| *x.1 != 0)
                .unwrap()
                .0
    }
}

impl Display for Tower {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows.iter().rev() {
            for i in (0..7).rev() {
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
    let mut wind = input.trim().bytes().enumerate().cycle();
    let mut rocks = ROCKS.iter().cycle();
    let mut tower = Tower::new();

    let mut i = 0;
    while i < 2022 {
        i += 1;
        tower.place(*rocks.next().unwrap(), i, &mut wind);
    }
    let res1 = tower.height();

    let limit = 1_000_000_000_000;
    while i < limit {
        i += 1;
        if let Some((pentry, entry)) = tower.place(*rocks.next().unwrap(), i, &mut wind) {
            let cycle_length = entry.rocks - pentry.rocks;
            let cycle_height = entry.height - pentry.height;

            let skip_cycles = (limit - i) / cycle_length;
            i += skip_cycles * cycle_length;
            tower.forgotten += skip_cycles * cycle_height;
            break;
        }
    }
    while i < limit {
        i += 1;
        tower.place(*rocks.next().unwrap(), i, &mut wind);
    }
    (res1, tower.height())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        assert_eq!(super::run(input), (3068, 1514285714288));
    }
}
