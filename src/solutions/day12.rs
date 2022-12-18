use crate::util::{
    grid::{Grid, Pos},
    pathfinding::bfs,
};

pub fn run(input: &str) -> (usize, usize) {
    let mut start = Pos::ZERO;
    let mut end = Pos::ZERO;

    #[rustfmt::skip]
    let grid = Grid::parse(input, |p, c| match c {
        'S' => { start = p; 0 }
        'E' => { end = p; 25 }
        c => c as u8 - b'a',
    });

    let mut res2 = None;
    let (res1, _) = bfs(
        &grid,
        end,
        |p, n| grid[p] as i16 - grid[n] as i16 <= 1,
        |c, p| {
            if grid[p] == 0 && res2.is_none() {
                res2 = Some(c);
            }
            p == start
        },
    );
    (res1.unwrap(), res2.unwrap())
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test() {
        let input = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi\n";
        let res = run(input);
        assert_eq!(res, (31, 29));
    }
}
