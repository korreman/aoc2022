use ascii::AsciiStr;

struct Tree {
    height: i8,
    visible: bool,
    scenic_score: u32,
}

impl Tree {
    fn update(&mut self, skyline: &mut Skyline) {
        if self.height > skyline.height {
            self.visible = true;
            skyline.height = self.height;
        }
        self.scenic_score *= skyline.distances[self.height as usize];
        for dist in skyline.distances.iter_mut().take(self.height as usize + 1) {
            *dist = 0;
        }
    }
}

struct Skyline {
    height: i8,
    distances: [u32; 10],
}

impl Skyline {
    fn new() -> Self {
        Self {
            height: -1,
            distances: [0; 10],
        }
    }

    fn step(&mut self) {
        self.distances.iter_mut().for_each(|d| *d += 1);
    }
}

pub fn run(input: &AsciiStr) -> (usize, u32) {
    let mut grid: Vec<Vec<Tree>> = input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|b| Tree {
                    height: (b - b'0') as i8,
                    visible: false,
                    scenic_score: 1,
                })
                .collect()
        })
        .collect();

    let mut skyline_row: Vec<Skyline> = grid[0].iter().map(|_| Skyline::new()).collect();
    for row in grid.iter_mut() {
        for (skyline, tree) in skyline_row.iter_mut().zip(row.iter_mut()) {
            tree.update(skyline);
            skyline.step();
        }

        let mut skyline = Skyline::new();
        for tree in row.iter_mut() {
            tree.update(&mut skyline);
            skyline.step();
        }

        skyline = Skyline::new();
        for tree in row.iter_mut().rev() {
            tree.update(&mut skyline);
            skyline.step();
        }
    }

    for skyline in skyline_row.iter_mut() {
        *skyline = Skyline::new();
    }

    for row in grid.iter_mut().rev() {
        for (skyline, tree) in skyline_row.iter_mut().zip(row.iter_mut()) {
            tree.update(skyline);
            skyline.step();
        }
    }

    let res1 = grid.iter().flatten().filter(|t| t.visible).count();
    let res2 = grid.iter().flatten().map(|t| t.scenic_score).max().unwrap();

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
