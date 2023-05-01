use crate::util::grid::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Target,
    BadTarget,
    Elf,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Cell::Empty => ".",
            Cell::Target => "o",
            Cell::BadTarget => "x",
            Cell::Elf => "#",
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    NO,
    EA,
    SO,
    WE,
}

pub fn run(input: &str) -> (u64, u64) {
    let mut elves = Vec::new();
    let grid = Grid::parse(input, |p, c| match c {
        '.' => Cell::Empty,
        '#' => {
            elves.push((p, None));
            Cell::Elf
        }
        c => panic!("unrecognized character '{c}'"),
    });
    let padding = grid.width().max(grid.height());
    let mut grid = grid.pad(padding, Cell::Empty);
    for (elf, _) in &mut elves {
        elf.x += padding;
        elf.y += padding;
    }
    //println!("{grid}");
    let mut directions = vec![Dir::NO, Dir::SO, Dir::WE, Dir::EA];
    let mut round = 0;
    let mut res1 = 0;
    loop {
        for (elf, target) in &mut elves {
            if grid[pos(elf.x + 1, elf.y + 0)] != Cell::Elf
                && grid[pos(elf.x + 1, elf.y - 1)] != Cell::Elf
                && grid[pos(elf.x + 0, elf.y - 1)] != Cell::Elf
                && grid[pos(elf.x - 1, elf.y - 1)] != Cell::Elf
                && grid[pos(elf.x - 1, elf.y + 0)] != Cell::Elf
                && grid[pos(elf.x - 1, elf.y + 1)] != Cell::Elf
                && grid[pos(elf.x - 0, elf.y + 1)] != Cell::Elf
                && grid[pos(elf.x + 1, elf.y + 1)] != Cell::Elf
            {
                continue;
            }

            for dir in &directions {
                let test_positions = match dir {
                    Dir::NO => [
                        pos(elf.x - 1, elf.y - 1),
                        pos(elf.x, elf.y - 1),
                        pos(elf.x + 1, elf.y - 1),
                    ],
                    Dir::SO => [
                        pos(elf.x - 1, elf.y + 1),
                        pos(elf.x, elf.y + 1),
                        pos(elf.x + 1, elf.y + 1),
                    ],
                    Dir::EA => [
                        pos(elf.x + 1, elf.y + 1),
                        pos(elf.x + 1, elf.y),
                        pos(elf.x + 1, elf.y - 1),
                    ],
                    Dir::WE => [
                        pos(elf.x - 1, elf.y + 1),
                        pos(elf.x - 1, elf.y),
                        pos(elf.x - 1, elf.y - 1),
                    ],
                };

                if test_positions.iter().all(|test| grid[*test] != Cell::Elf) {
                    if grid[test_positions[1]] == Cell::Target
                        || grid[test_positions[1]] == Cell::BadTarget
                    {
                        grid[test_positions[1]] = Cell::BadTarget;
                    } else {
                        grid[test_positions[1]] = Cell::Target;
                    }
                    *target = Some(test_positions[1]);
                    break;
                }
            }
        }
        // Rotate directions
        let first = directions.remove(0);
        directions.push(first);

        //println!("{grid}");
        let mut changed = false;
        for (elf, target) in &mut elves {
            if let &mut Some(t) = target {
                if grid[t] == Cell::Target {
                    grid[*elf] = Cell::Empty;
                    grid[t] = Cell::Elf;
                    *elf = t;
                    changed = true;
                } else {
                    grid[t] = Cell::Empty;
                }
                *target = None;
            }
        }
        //println!("{grid}");
        round += 1;
        if round == 10 {
            let min_x = elves.iter().map(|(p, _)| p.x).min().unwrap();
            let max_x = elves.iter().map(|(p, _)| p.x).max().unwrap();
            let min_y = elves.iter().map(|(p, _)| p.y).min().unwrap();
            let max_y = elves.iter().map(|(p, _)| p.y).max().unwrap();
            res1 = (max_x - min_x + 1) * (max_y - min_y + 1) - elves.len();
        }
        if !changed {
            break;
        }
    }
    //println!("{grid}");
    (res1 as u64, round)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = "\
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..\
";
        assert_eq!(run(input), (110, 20));
    }
}
