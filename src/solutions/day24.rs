use std::ops::Index;
use std::ops::IndexMut;

use itertools::Itertools;

use crate::util::{graph::*, grid::*, grid3d, pathfinding::bfs};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct TimePos {
    pos: grid3d::Pos,
}

struct Blizzards<T> {
    width: usize,
    height: usize,
    u: Grid<bool>,
    d: Grid<bool>,
    l: Grid<bool>,
    r: Grid<bool>,
    nodes: grid3d::Grid<T>,
}

impl<T> Blizzards<T> {
    fn valid_pos(&self, p: &TimePos) -> bool {
        let d = self.u[pos(p.pos.x, (p.pos.y + p.pos.z) % self.height)];
        let r = self.l[pos((p.pos.x + p.pos.z) % self.width, p.pos.y)];
        let u = self.d[pos(
            p.pos.x,
            (p.pos.y + (self.height - p.pos.z % self.height)) % self.height,
        )];
        let l = self.r[pos(
            (p.pos.x + (self.width - p.pos.z % self.width)) % self.width,
            p.pos.y,
        )];
        p.pos.z < self.nodes.depth() && u && d && r && l
    }
}

impl<T> Index<TimePos> for Blizzards<T> {
    type Output = T;

    fn index(&self, index: TimePos) -> &Self::Output {
        //println!("{index:?}");
        &self.nodes[index.pos]
    }
}

impl<T> IndexMut<TimePos> for Blizzards<T> {
    fn index_mut(&mut self, index: TimePos) -> &mut Self::Output {
        &mut self.nodes[index.pos]
    }
}

impl<T> Graph<T> for Blizzards<T> {}
impl<T> GraphImpl<T> for Blizzards<T> {
    type Handle = TimePos;

    type Neighbors = std::vec::IntoIter<TimePos>;
    fn neighbors(&self, handle: Self::Handle) -> Self::Neighbors {
        let mut candidates = self
            .u
            .neighbors(pos(handle.pos.x, handle.pos.y))
            .map(|pos| TimePos {
                pos: grid3d::Pos {
                    x: pos.x,
                    y: pos.y,
                    z: handle.pos.z + 1,
                },
            })
            .collect_vec();
        candidates.push(TimePos {
            pos: grid3d::Pos {
                x: handle.pos.x,
                y: handle.pos.y,
                z: handle.pos.z + 1,
            },
        });
        candidates.retain(|p| self.valid_pos(p));
        candidates.into_iter()
    }

    // NOTE: Not implemented
    type AllHandles = std::vec::IntoIter<TimePos>;
    fn handles(&self) -> Self::AllHandles {
        vec![].into_iter()
    }

    type Map<U> = Blizzards<U>;
    fn map<U, F: FnMut(&T) -> U>(&self, f: F) -> Self::Map<U> {
        let new_nodes = self.nodes.map(f);
        Blizzards {
            width: self.width,
            height: self.height,
            u: self.u.clone(),
            d: self.d.clone(),
            l: self.l.clone(),
            r: self.r.clone(),
            nodes: new_nodes,
        }
    }
}

pub fn run(input: &str) -> (usize, usize) {
    let width = input.lines().next().unwrap().len() - 2;
    let height = input.lines().count() - 2;
    let mut u = Grid::new_filled(width, height, true);
    let mut d = Grid::new_filled(width, height, true);
    let mut r = Grid::new_filled(width, height, true);
    let mut l = Grid::new_filled(width, height, true);
    drop(Grid::parse(input, |p, c| {
        if p.x > 0 && p.y > 0 {
            let bpos = pos(p.x - 1, p.y - 1);
            match c {
                '^' => u[bpos] = false,
                'v' => d[bpos] = false,
                '<' => l[bpos] = false,
                '>' => r[bpos] = false,
                '.' => (),
                '#' => (),
                c => panic!("unrecognized character '{c}'"),
            };
        }
        ()
    }));
    let blizzards: Blizzards<()> = Blizzards {
        width,
        height,
        u,
        d,
        l,
        r,
        nodes: grid3d::Grid::new_filled(width, height, 10000, ()), // TODO: Magic number
    };

    let cost = 2 + bfs(
        &blizzards,
        TimePos {
            pos: grid3d::pos(0, 0, 1),
        },
        |_, _| true,
        |_, p| p.pos == grid3d::pos(width - 1, height - 1, p.pos.z),
    )
    .unwrap();
    //let cost_back = 2 + bfs(
    //    &blizzards,
    //    TimePos {
    //        pos: grid3d::pos(width - 1, height - 1, cost + 6),
    //    },
    //    |_, _| true,
    //    |_, p| p.pos == grid3d::pos(0, 0, p.pos.z),
    //)
    //.unwrap();
    //let cost_back_again = 2 + bfs(
    //    &blizzards,
    //    TimePos {
    //        pos: grid3d::pos(0, 0, cost + cost_back + 2),
    //    },
    //    |_, _| true,
    //    |_, p| p.pos == grid3d::pos(width - 1, height - 1, p.pos.z),
    //)
    //.unwrap();

    (cost, 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_blizzards() {
        let input = "\
#.######
#......#
#......#
#......#
#......#
######.#\
";
        assert_eq!(run(input).0, 10);
    }

    #[test]
    fn diag_d() {
        let input = "\
#.######
#vvv.vv#
#vv.vvv#
#v.vvv.#
#.vvv.v#
######.#\
";
        assert_eq!(run(input).0, 10);
    }

    #[test]
    fn diag_r() {
        let input = "\
#.######
#>>>>>.#
#>>>>.>#
#>>>.>>#
#>>.>>>#
######.#\
";
        assert_eq!(run(input).0, 10);
    }

    #[test]
    fn diag_l() {
        let input = "\
#.######
#<.<<<.#
#<<.<<<#
#<<<.<<#
#......#
######.#\
";
        assert_eq!(run(input).0, 10);
    }

    #[test]
    fn diag_u() {
        let input = "\
#.######
#.^^.^.#
#.^^^..#
#..^^^.#
#.^.^^.#
######.#\
";
        assert_eq!(run(input).0, 10);
    }

    #[test]
    fn test() {
        let input = "\
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#\
";
        assert_eq!(run(input), (18, 54));
    }
}
