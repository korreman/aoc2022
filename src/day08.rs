struct Tree {
    height: u8,
    visible: bool,
    scenic_score: u32,
}

struct Skyline {
    height: u8,
    distances: [u32; 10],
}

impl Skyline {
    #[inline]
    fn new(tree: &mut Tree) -> Self {
        Self {
            height: tree.height,
            distances: [0; 10],
        }
    }

    #[inline]
    fn step(&mut self, idx: u32, tree: &mut Tree) {
        if tree.height > self.height {
            tree.visible = true;
            self.height = tree.height;
        }
        tree.scenic_score *= idx - unsafe { self.distances.get_unchecked(tree.height as usize) };
        for dist in self.distances.iter_mut().take(tree.height as usize + 1) {
            *dist = idx;
        }
    }
}

pub fn run(input: &str) -> (usize, u32) {
    let mut lines = input.lines();
    let width = lines.next().unwrap().len();
    let mut grid: Vec<Tree> = input
        .as_bytes()
        .iter()
        .filter(|b| **b != b'\n')
        .map(|digit| {
            let height = digit - b'0';
            Tree {
                height,
                visible: false,
                scenic_score: 1,
            }
        })
        .collect();

    for i in 1..width - 1 {
        let mut skyline_t = Skyline::new(&mut grid[i]); // From top
        let mut skyline_b = Skyline::new(&mut grid[(width - 1) * width + i]); // From bottom
        let mut skyline_l = Skyline::new(&mut grid[i * width]); // From left
        let mut skyline_r = Skyline::new(&mut grid[(i + 1) * width - 1]); // From right
        for j in 1..width - 1 {
            skyline_t.step(j as u32, unsafe { grid.get_unchecked_mut(j * width + i) });
            skyline_b.step(j as u32, unsafe {
                grid.get_unchecked_mut((width - j - 1) * width + i)
            });
            skyline_l.step(j as u32, unsafe { grid.get_unchecked_mut(i * width + j) });
            skyline_r.step(j as u32, unsafe {
                grid.get_unchecked_mut((i + 1) * width - 1 - j)
            });
        }
    }

    let res1 = grid.iter().filter(|t| t.visible).count() + 4 * width - 4;
    let res2 = grid.iter().map(|t| t.scenic_score).max().unwrap();

    (res1, res2)
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let input = "30373\n25512\n65332\n33549\n35390\n";
        let (res1, res2) = super::run(input);
        assert_eq!((res1, res2), (21, 8));
    }
}
