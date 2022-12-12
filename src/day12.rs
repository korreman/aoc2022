use crate::util::grid::{Grid, Pos};
use ascii::AsciiStr;

pub fn run(input: &AsciiStr) -> (usize, usize) {
    let mut start = Pos::ZERO;
    let mut end = Pos::ZERO;

    #[rustfmt::skip]
    let grid = Grid::parse(input.as_str(), |p, c| match c {
        'S' => { start = p; 0 }
        'E' => { end = p; 25 }
        c => c as u8 - b'a',
    });

    return bfs(&grid, start, end).unwrap();
}

pub fn bfs(grid: &Grid<u8>, start: Pos, end: Pos) -> Option<(usize, usize)> {
    let mut visited = Grid::new_filled(grid.width(), grid.height(), false);
    let mut positions = vec![end];
    let mut step = 0;
    let mut res2 = 0;
    while !positions.is_empty() {
        for position in std::mem::take(&mut positions) {
            if grid[position] == 0 && res2 == 0 {
                res2 = step;
            }
            if position == start {
                return Some((step, res2));
            }
            for neighbor in grid.neighbors(position) {
                if grid[position] as i16 - grid[neighbor] as i16 <= 1 && !visited[neighbor] {
                    visited[neighbor] = true;
                    positions.push(neighbor);
                }
            }
        }
        step += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use ascii::AsciiStr;

    use super::run;

    #[test]
    fn test() {
        let input = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi\n";
        let res = run(AsciiStr::from_ascii(input).unwrap());
        assert_eq!(res, (31, 29));
    }
}
