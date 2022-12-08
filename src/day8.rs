use ascii::AsciiStr;

struct Tree {
    height: i8,
    visible: bool,
    scenic_score: u32,
}

struct Skyline {
    height: i8,
    distances: [u32; 10],
}

impl Skyline {
    #[inline]
    fn new() -> Self {
        Self {
            height: -1,
            distances: [0; 10],
        }
    }

    #[inline]
    fn step(&mut self, idx: u32, tree: &mut Tree) {
        if tree.height > self.height {
            tree.visible = true;
            self.height = tree.height;
        }
        tree.scenic_score *= idx - self.distances[tree.height as usize];
        for dist in self.distances.iter_mut().take(tree.height as usize + 1) {
            *dist = idx;
        }
    }
}

pub fn run(input: &AsciiStr) -> (usize, u32) {
    let mut lines = input.lines();
    let width = lines.next().unwrap().len();
    let mut grid: Vec<Tree> = input
        .lines()
        .flatten()
        .map(|digit| {
            let height = (digit.as_byte() - b'0') as i8;
            assert!(height >= 0, "{height}"); // Why does this improve performance???
            Tree {
                height,
                visible: false,
                scenic_score: 1,
            }
        })
        .collect();

    let mut skyline_t = Skyline::new(); // From top
    let mut skyline_b = Skyline::new(); // From bottom
    let mut skyline_l = Skyline::new(); // From left
    let mut skyline_r = Skyline::new(); // From right

    for i in 0..width {
        for j in 0..width {
            skyline_t.step(j as u32, &mut grid[j * width + i]);
            skyline_l.step(j as u32, &mut grid[i * width + j]);
            skyline_b.step(j as u32, &mut grid[(width - j - 1) * width + i]);
            skyline_r.step(j as u32, &mut grid[i * width + (width - 1 - j)]);
        }
        skyline_t = Skyline::new();
        skyline_b = Skyline::new();
        skyline_l = Skyline::new();
        skyline_r = Skyline::new();
    }

    let res1 = grid.iter().filter(|t| t.visible).count();
    let res2 = grid.iter().map(|t| t.scenic_score).max().unwrap();

    (res1, res2)
}

#[cfg(test)]
mod test {
    use ascii::AsciiStr;

    #[test]
    fn test() {
        let input = AsciiStr::from_ascii("30373\n25512\n65332\n33549\n35390\n").unwrap();
        let (res1, res2) = super::run(input);
        assert_eq!((res1, res2), (21, 8));
    }
}
