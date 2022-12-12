use std::{cmp::Reverse, collections::BinaryHeap};

use crate::util::{
    grid::{Grid, Pos},
    pathfinding::{a_star, dijkstra},
    queue::{KVPair, SlidingBucketQueue},
};
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

    type Queue1 = BinaryHeap<Reverse<KVPair<usize, Pos>>>;
    let res1 = a_star::<Queue1, _, _, _, _>(
        &grid,
        start,
        |a, b| {
            if grid[b] as i16 - grid[a] as i16 <= 1 {
                Some(1)
            } else {
                None
            }
        },
        |p| p.dist(&end),
        |pos| pos == end,
    )
    .unwrap_or(0);

    let res2 = dijkstra::<SlidingBucketQueue<2, Pos>, _, _, _>(
        &grid,
        end,
        |a, b| {
            if grid[a] as i16 - grid[b] as i16 <= 1 {
                Some(1)
            } else {
                None
            }
        },
        |p| grid[p] == 0,
    )
    .unwrap();

    (res1, res2)
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
