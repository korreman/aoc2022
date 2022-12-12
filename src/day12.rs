use crate::util::{
    grid::{Grid, Pos},
    pathfinding::dijkstra,
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
    let res1 = dijkstra(
        &grid,
        2,
        start,
        |a, b| {
            if *b as i16 - *a as i16 <= 1 {
                Some(1)
            } else {
                None
            }
        },
        |pos| pos == end,
    )
    .unwrap();

    let res2 = dijkstra(
        &grid,
        2,
        end,
        |a, b| {
            if *a as i16 - *b as i16 <= 1 {
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
