use std::{cmp::Reverse, collections::BinaryHeap};

use crate::util::{
    grid::{Grid, Pos},
    pathfinding::{a_star, dijkstra},
    queue::{KVPair, RadixHeap, SlidingBucketQueue},
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

    type Queue = RadixHeap<Pos>;
    let cost = |a, b| {
        if grid[a] as i16 - grid[b] as i16 <= 1 {
            Some(1)
        } else {
            None
        }
    };
    let res1 = a_star::<Queue, _, _, _, _>(&grid, end, cost, |p| p.dist(&end), |pos| pos == start);

    let res2 = a_star::<Queue, _, _, _, _>(&grid, end, cost, |p| p.x, |p| grid[p] == 0);

    (res1.unwrap(), res2.unwrap())
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
