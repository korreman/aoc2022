use ascii::{AsciiChar, AsciiStr};

pub fn run(input: &AsciiStr) -> (u32, u32) {
    // Tracks the directory sizes of each parent.
    let mut stack: Vec<u32> = vec![];
    // Records all directory sizes
    let mut dirs: Vec<u32> = vec![];
    // Tracks size of current directory.
    let mut dir = 0;

    let mut res1 = 0;
    for line in input.lines() {
        // We only need to parse relevant CLI lines
        match line.as_bytes().strip_prefix(b"$ cd ") {
            Some(b"..") => {
                dirs.push(dir);
                if dir <= 100_000 {
                    res1 += dir;
                }
                dir += stack.pop().unwrap();
            }
            Some(_) => {
                stack.push(dir);
                dir = 0;
            }
            _ => {
                let word = line.split(AsciiChar::Space).next().unwrap();
                if let Ok(file_size) = word.as_str().parse::<u32>() {
                    dir += file_size;
                }
            }
        }
    }

    // Exit remaining directories
    while let Some(parent_size) = stack.pop() {
        dirs.push(dir);
        if dir <= 100_000 {
            res1 += dir;
        }
        dir += parent_size;
    }

    let space_to_free = 30_000_000 - (70_000_000 - dir);
    let res2 = *dirs
        .iter()
        .filter(|dir_size| **dir_size >= space_to_free)
        .min()
        .expect("no delete target found");

    (res1, res2)
}
