use crate::util::{
    dijkstra::Dijkstra,
    grid::{Grid, Pos},
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
    let mut dijkstra1 = Dijkstra::new(&grid, 2, start);
    let res1 = dijkstra1
        .run(
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

    let mut dijkstra2 = Dijkstra::new(&grid, 2, end);
    dijkstra2.run(
        |a, b| {
            if *a as i16 - *b as i16 <= 1 {
                Some(1)
            } else {
                None
            }
        },
        |_| false,
    );

    let mut res2 = usize::MAX;
    let mut costs = dijkstra2.to_costs();
    costs.for_each(|pos, cost| {
        if grid[pos] == 0 {
            res2 = res2.min(*cost)
        }
    });
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
