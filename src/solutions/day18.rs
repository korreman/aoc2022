use itertools::Itertools;

use crate::util::{
    graph::GraphImpl,
    grid3d::{pos, Grid},
    pathfinding::bfs,
};

pub fn run(input: &str) -> (usize, usize) {
    // Parse
    let cubes = input
        .lines()
        .map(|line| line.split(',').collect_tuple().unwrap())
        .map(|(x, y, z)| {
            pos(
                x.parse::<usize>().unwrap() + 1,
                y.parse::<usize>().unwrap() + 1,
                z.parse::<usize>().unwrap() + 1,
            )
        })
        .collect_vec();

    // Part 1
    let size = cubes.iter().map(|p| p.x.max(p.y.max(p.z))).max().unwrap() + 2;
    let mut grid = Grid::new_filled(size, size, size, false);
    let mut res1 = 0;
    for cube in cubes {
        res1 += 6;
        grid[cube] = true;
        for n in grid.neighbors(cube) {
            if grid[n] {
                res1 -= 2;
            }
        }
    }

    // Part 2
    let mut res2 = 0;
    let _ = bfs(
        &grid,
        pos(0, 0, 0),
        |_, p| !grid[p],
        |_, p| {
            for n in grid.neighbors(p) {
                if grid[n] {
                    res2 += 1;
                }
            }
            false
        },
    );
    (res1, res2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_input() {
        let input = "2,2,2\n1,2,2\n3,2,2\n2,1,2\n2,3,2\n2,2,1\n2,2,3\n2,2,4\n2,2,6\n1,2,5\n3,2,5\n2,1,5\n2,3,5";
        assert_eq!(super::run(input), (64, 58));
    }

    #[test]
    fn test_tiny() {
        let input = "1,1,1\n2,1,1";
        assert_eq!(super::run(input).0, 10);
    }
}
