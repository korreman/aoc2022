use crate::util::{
    grid::{Grid, Pos},
    pathfinding::dijkstra,
    queue::SlidingBucketQueue,
};
use ascii::AsciiStr;

pub fn run(input: &AsciiStr) -> (usize, usize) {
    let mut start = Pos::ZERO;
    let mut end = Pos::ZERO;
    let grid = Grid::parse(input.as_str(), |p, c| match c {
        'S' => {
            start = p;
            0
        }
        'E' => {
            end = p;
            25
        }
        c => c as u8 - b'a',
    });
    let res1 = dijkstra::<SlidingBucketQueue<2, Pos>, _, _, _>(
        &grid,
        start,
        |a, b| {
            if grid[b] as i16 - grid[a] as i16 <= 1 {
                Some(1)
            } else {
                None
            }
        },
        |pos| pos == end,
    )
    .unwrap();

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
        let (res1, res2) = run(AsciiStr::from_ascii(input).unwrap());
        assert_eq!(res1, 31);
        assert_eq!(res2, 29);
    }
}
